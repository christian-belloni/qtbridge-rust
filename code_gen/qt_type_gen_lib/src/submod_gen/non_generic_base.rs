// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::rc::Rc;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::spanned::Spanned;

use qtbridge_gen_common::cpp_include::CppInclude;
use qtbridge_gen_common::type_registry::qt::monomorphed::QtMonomorphedType;
use qtbridge_gen_common::type_registry::type_traits::{FindType, TypeInfo};
use qtbridge_gen_common::type_tokens::TypeTokens;
use qtbridge_gen_common::type_utils::ident_to_type;
use qtbridge_gen_common::function_bridge::CppFunctionBridge;
use qtbridge_gen_common::multi_type_mapping::MultiTypeMapping;
use qtbridge_gen_common::naming;
use qtbridge_gen_common::path_utils::relative_input_file_path_to_path_qualified;
use qtbridge_gen_common::type_dependencies::{qt_types_to_bridge_imports, type_tokens_to_cpp_includes};
use qtbridge_gen_common::type_mapping::TypeMapping;
use qtbridge_gen_common::type_mapping_nested::TypeMappingNested;
use qtbridge_gen_common::type_to_cpp::type_to_cpp;

use crate::function::Function;
use crate::generic_instantiation_decl::GenericInstantiationDecl;
use crate::module::Module;
use crate::reexport::Reexport;
use crate::structure::BridgeStruct;
use crate::submod_gen::common::get_derive_attr;
use crate::submodule_type_tokens::SubmoduleTypeTokens;
use crate::trait_impl::TraitImpl;

use super::common::{get_functions_substituted, get_traits_substituted, get_unresolved_type_dependencies};

/// Base for NonGenericSubmoduleGenerator and MonomorphedSubmoduleGenerator
/// holding functionality common for these structures
pub struct NonGenericSubmoduleGeneratorBase {
    src_module: Rc<Module>,
    input_file_path: String,
    submod_ident: syn::Ident,
    struct_ident: Option<syn::Ident>,
    type_map: TypeMappingNested<MultiTypeMapping>,
    funcs_substituted: Vec<Function>,
    traits_substituted: Vec<TraitImpl>,
    type_tokens: SubmoduleTypeTokens,
    qmetatype_id: Option<i32>,
}

impl NonGenericSubmoduleGeneratorBase {
    pub fn new(src_module: Rc<Module>, input_file_path: String, submod_ident: syn::Ident,
        mut struct_ident: Option<syn::Ident>, inst: Option<&GenericInstantiationDecl>) -> syn::Result<Self> {
        let struct_ = src_module.structure();

        struct_ident = struct_ident
            .or_else(|| struct_.map(|s| s.ident().clone()));
        let struct_type = ident_to_type(struct_ident
            .clone()
            .unwrap_or_else(|| format_ident!("dummy")));

        let type_map_impl = if let Some(inst) = inst.as_ref() {
            inst.types()
                .get_type_map_for(struct_.as_ref().unwrap().generics())?
        }
        else {
            MultiTypeMapping::default()
        };
        let type_map = TypeMappingNested::new(type_map_impl);

        let qmetatype = match &inst {
            Some(inst_decl) => inst_decl.qmetatype_id(),
            None => struct_
                .and_then(|s| s.qmetatype_attr())
        };
        let qmetatype_id = qmetatype.map(|q| q.id().unwrap_or_default());

        let funcs_substituted = get_functions_substituted(src_module.functions(), &struct_type, &type_map)?;
        let traits_substituted = match inst.as_ref() {
            Some(inst) =>get_traits_substituted(
                src_module.traits().iter()
                    .filter(|trait_| trait_.is_included_for_struct_instantiations(inst.types())),
                &type_map)?,
            None => get_traits_substituted(src_module.traits().iter(), &type_map)?
        };

        Ok(Self {
            src_module,
            input_file_path,
            submod_ident,
            struct_ident,
            type_map,
            funcs_substituted,
            traits_substituted,
            type_tokens: SubmoduleTypeTokens::default(),
            qmetatype_id,
        })
    }

    pub fn module(&self) -> &Module {
        self.src_module.as_ref()
    }

    pub fn input_file_path(&self) -> String {
        self.input_file_path.clone()
    }

    pub fn structure(&self) -> Option<&BridgeStruct> {
        self.src_module.structure()
    }

    pub fn struct_ident(&self) -> Option<&syn::Ident> {
        self.struct_ident.as_ref()
    }

    pub fn src_struct_ident(&self) -> Option<&syn::Ident> {
        Some(self.structure()?.ident())
    }

    pub fn submod_name(&self) -> String {
        self.submod_ident.to_string()
    }

    pub fn type_map(&self) -> &TypeMappingNested<MultiTypeMapping> {
        &self.type_map
    }

    pub fn qmetatype_id(&self) -> Option<i32> {
        self.qmetatype_id
    }

    pub fn is_qmetatypeid_func_needed(&self) -> bool {
        self.qmetatype_id.as_ref().is_some_and(|id| *id == 0)
    }

    pub fn type_tokens(&self) -> &SubmoduleTypeTokens {
        &self.type_tokens
    }

    pub fn traits_mut(&mut self) -> impl Iterator<Item = &mut TraitImpl> {
        self.traits_substituted.iter_mut()
    }

    pub fn functions(&self) -> impl Iterator<Item = &Function> {
        self.funcs_substituted.iter()
    }

    pub fn functions_mut(&mut self) -> impl Iterator<Item = &mut Function> {
        self.funcs_substituted.iter_mut()
    }

    pub fn check_unclassified_type_tokens(&mut self) -> syn::Result<()> {
        self.type_tokens.check_unclassified()
    }

    pub fn is_monomorphed(&self) -> bool {
        !self.type_map().get_impl().is_empty()
    }

    /// https://cxx.rs/concepts.html
    pub fn is_shared_struct(&self) -> bool {
        self.structure_has_fields()
    }

    pub fn is_opaque_struct(&self) -> bool {
        !self.is_shared_struct()
    }

    fn structure_has_fields(&self) -> bool {
        self.structure().is_some_and(|s| !s.fields().is_empty())
    }

    pub fn has_inline_cpp_functions(&self) -> bool {
        if self.funcs_substituted.iter()
            .any(|func| !func.cpp_functions().is_empty()) {
                return true
            }

        if self.traits_substituted.iter()
            .any(|tr| tr.functions().iter()
                .any(|func| !func.cpp_functions().is_empty())) {
                    return true
                }

        false
    }

    pub fn namespace(&self) -> Option<String> {
        let result = self.structure()?
            .namespace()?
            .clone();
        Some(result)
    }

    pub fn get_include_path(&self) -> syn::Result<String> {
        let path_qualified = relative_input_file_path_to_path_qualified(&self.input_file_path)
            .map_err(|err| syn::Error::new(self.struct_ident().span(), format!("Failed get include path for submodule '{}': {err}", self.submod_name())))?;
        let filename = naming::cpp::filename::type_gen_header(&self.submod_name());
        Ok(format!("qtbridge-type-lib/src/generated/{}/cpp/{filename}", path_qualified.join("/")))
    }

    fn get_struct_bridge_decl(&self) -> Option<TokenStream> {
        let struct_ = self.structure()?;
        let ident = self.struct_ident();
        let namespace_attr = self.get_namespace_attr();

        let tokens = if struct_.fields().is_empty() {
            let docs = struct_.docs();
            // Struct content is not defined on the Rust side - opaque data type
            quote! {
                #namespace_attr
                #(#docs)*
                type #ident;
            }
        }
        else {
            // We have declaration of struct content on the Rust side (below mod ffi) - shared data type
            quote! {
                #[allow(dead_code)]
                #namespace_attr
                type #ident = super::#ident;
            }
        };
        Some(tokens)
    }

    fn get_namespace_attr(&self) -> Option<TokenStream> {
        self.structure()
            .and_then(|s| s.namespace())
            .map(|ns| quote! { #[namespace = #ns]})
    }

    pub fn get_ffi_mod_content(&self) -> syn::Result<TokenStream> {
        let submodule_name = self.submod_name();
        let bridge_header = self.get_include_path()
            .map_err(|err| syn::Error::new(self.struct_ident().span(), format!("Failed to get input path: {err}")))?;
        let struct_bridge_decl = self.get_struct_bridge_decl();

        let bridge_namespace = naming::cpp::namespace::type_bridge(&submodule_name);
        let def_traits_funcs = self.get_def_cpp_traits_bridge_funcs()?;
        let qmetatype_get_trait_bridge = self.get_qmetatype_get_trait_bridge_code()?;
        let inline_cpp_funcs_bridges = self.get_inline_cpp_functions_bridges()?;
        let all_funcs = def_traits_funcs.into_iter()
            .chain(qmetatype_get_trait_bridge)
            .chain(inline_cpp_funcs_bridges)
            .collect::<Vec<_>>();

        let mut bridge_tokens = TypeTokens::default();
        all_funcs.iter()
            .try_for_each(|func_brdige| bridge_tokens.collect_from_signature(func_brdige.signature()))?;
        let mut bridge_imports = qt_types_to_bridge_imports(
            bridge_tokens.iter_qt(), true
        )?;
        // Avoid defining type alias and importing the same type twice
        // (one in #struct_bridge_decl and second time in #bridge_imports)
        if let Some(struct_ident) = self.struct_ident() {
            bridge_imports.retain(|imp| imp.type_name() != struct_ident);
        }

        Ok(quote! {
            unsafe extern "C++" {
                include!(#bridge_header);
                #struct_bridge_decl
                // https://cxx.rs/extern-c++.html#reusing-existing-binding-types
                #(#bridge_imports)*
            }

            #[namespace = #bridge_namespace]
            unsafe extern "C++" {
                #(#all_funcs)*
            }
        })
    }

    fn get_def_cpp_traits_bridge_funcs(&self) -> syn::Result<Vec<CppFunctionBridge>> {
        let Some(struct_) = self.structure() else {
            return Ok(vec![])
        };

        let struct_ident = struct_.ident();
        let ident = self.struct_ident().unwrap();
        let ident_str = struct_ident.to_string();

        let mut result = Vec::new();
        if struct_.is_trait_cpp_derived("Drop") {
            let func_name_rust = naming::rust::function::drop(&ident_str);
            let func_name_cpp = naming::cpp::function::drop(&ident_str);
            result.push(CppFunctionBridge::new(func_name_rust, syn::parse2(quote! {
                fn #func_name_cpp(v: &mut #ident)
            })?)?);
        }
        if struct_.is_trait_cpp_derived("Default") {
            let func_name_rust = naming::rust::function::default(&ident_str);
            let func_name_cpp = naming::cpp::function::default(&ident_str);
            result.push(CppFunctionBridge::new(func_name_rust, syn::parse2(quote! {
                fn #func_name_cpp() -> #ident
            })?)?);
        }
        if struct_.is_trait_cpp_derived("Clone") {
            let func_name_rust = naming::rust::function::clone(&ident_str);
            let func_name_cpp = naming::cpp::function::clone(&ident_str);
            result.push(CppFunctionBridge::new(func_name_rust, syn::parse2(quote! {
                fn #func_name_cpp(v: &#ident) -> #ident
            })?)?);
        }

        Ok(result)
    }

    fn get_qmetatype_get_trait_bridge_code(&self) -> syn::Result<Option<CppFunctionBridge>> {
        let Some(struct_) = self.src_module.structure() else {
            return Ok(None)
        };
        if !self.is_qmetatypeid_func_needed() {
            return Ok(None)
        }

        let struct_name = struct_.ident().to_string();
        let func_name_rust = naming::rust::function::qmetatype(&struct_name);
        let func_name_cpp = naming::cpp::function::qmetatype(&struct_name);

        Ok(Some(CppFunctionBridge::new(func_name_rust, syn::parse2(quote! {
            fn #func_name_cpp() -> QMetaType
        })?)?))
    }

    fn get_inline_cpp_functions_bridges(&self) -> syn::Result<Vec<CppFunctionBridge>> {

        let func_prefix = Function::get_inline_functions_default_prefix();
        let func_self_type: Option<syn::Type> = self.struct_ident()
            .map(|ident| ident_to_type(ident.clone()));
        let is_opaque_struct = self.is_opaque_struct();

        let mut result = Vec::new();
        for function in &self.funcs_substituted {
            result.extend(function.get_cpp_funcs_bridges(&func_prefix, func_self_type.as_ref(), is_opaque_struct)?);
        }

        for trait_ in &self.traits_substituted {
            let trait_func_prefix = trait_.get_inline_trait_functions_default_prefix()?;
            let trait_self_type = trait_.self_type();

            for function in trait_.functions() {
                result.extend(function.get_cpp_funcs_bridges(&trait_func_prefix, Some(trait_self_type), is_opaque_struct)?);
            }
        }

        Ok(result)
    }

    pub fn get_common_content_after_ffi_block(&self) ->syn::Result<TokenStream> {
        let ident = self.struct_ident();
        let struct_ = self.structure();
        let is_struct_without_fields = struct_
            .is_some_and(|s| s.fields().is_empty());

        let use_struct_from_ffi = is_struct_without_fields
            .then_some(quote! {
                #[allow(dead_code)]
                #[allow(unused_imports)]
                pub use ffi::#ident;
            });

        let struct_repr = self.get_struct_repr_code();
        let impl_extern_type = self.get_impl_extern_trait_code();
        //let func_block = self.get_func_block()?;
        let def_traits_code = self.get_def_cpp_traits_rust_code();
        let qmetatype_get_trait_code = self.get_qmetatype_get_trait_rust_code();
        let traits_code = Self::get_traits_rust_code(&self.traits_substituted)?;
        let other_items = self.src_module.other_items(); //TODO: replace type here as well

        Ok(quote! {
            #use_struct_from_ffi
            #struct_repr
            #impl_extern_type

            //#func_block
            #def_traits_code
            #qmetatype_get_trait_code
            #traits_code
            #(#other_items)*
        })
    }

    fn get_struct_repr_code(&self) -> Option<TokenStream> {
        if self.is_monomorphed() {
            return None
        }

        let struct_ = self.structure()?;
        let fields = struct_.fields();
        if fields.is_empty() {
            return None
        }

        let docs = struct_.docs();
        let derived_attr = get_derive_attr(struct_.derived_traits());

        // We append suffix with generic instantiation types to struct ident separated by '_'.
        // For instance 'QHash_QString_QVariant'.
        // Need to mute corresponding warning.
        let ident = self.struct_ident()?;
        let maybe_non_camel_case = (ident != struct_.ident())
            .then_some(quote!{ #[allow(non_camel_case_types)] });

        let struct_repr = quote! {
            #(#docs)*
            #derived_attr
            #[repr(C)]
            #maybe_non_camel_case
            pub struct #ident {
                #(#fields,)*
            }
        };

        Some(struct_repr)
    }

    fn get_impl_extern_trait_code(&self) -> Option<TokenStream> {
        let struct_ = self.structure()?;
        let ident = self.struct_ident()?;
        let ident_str = ident.to_string();

        let maybe_namespace_w_colons = struct_.namespace()
                .map(|ns| format!("{ns}::"))
                .unwrap_or_default();
        let type_id = format!("{maybe_namespace_w_colons}{ident_str}");

        self.is_shared_struct()
            .then_some(quote! {
                unsafe impl cxx::ExternType for #ident {
                    type Id = cxx::type_id!(#type_id);
                    type Kind = cxx::kind::Trivial;
                }
            })
    }

    fn get_def_cpp_traits_rust_code(&self) -> Option<TokenStream> {
        let struct_ = self.structure()?;

        let ident = self.src_struct_ident()?;
        let ident_str = ident.to_string();
        let ident_inst = self.struct_ident();

        let mut result = TokenStream::new();
        if !self.is_monomorphed() && struct_.is_trait_cpp_derived("Drop") {
            let func_name = naming::rust::function::drop(&ident_str);
            quote!{
                impl Drop for #ident_inst {
                    fn drop(&mut self) {
                        ffi::#func_name(self)
                    }
                }
            }.to_tokens(&mut result);
        }
        if struct_.is_trait_cpp_derived("Default") {
            let func_name = naming::rust::function::default(&ident_str);
            quote!{
                impl Default for #ident_inst {
                    fn default() -> Self {
                        ffi::#func_name()
                    }
                }
            }.to_tokens(&mut result);
        }
        if struct_.is_trait_cpp_derived("Clone") {
            let func_name = naming::rust::function::clone(&ident_str);
            quote!{
                impl Clone for #ident_inst {
                    fn clone(&self) -> Self {
                        ffi::#func_name(self)
                    }
                }
            }.to_tokens(&mut result);
        }

        Some(result)
    }

    fn get_qmetatype_get_trait_rust_code(&self) -> Option<TokenStream> {
        let id = self.qmetatype_id()?;
        let impl_code = match id {
            1.. => quote! {
                crate::QMetaType::new(#id)
            },
            _ => {
                let struct_name_src = self.src_struct_ident()?;
                let func_name = naming::rust::function::qmetatype(struct_name_src);
                quote! {
                    ffi::#func_name()
                }
            }
        };

        let struct_ident_inst = self.struct_ident()?;
        let trait_code = quote! {
            impl crate::QMetaTypeGet for #struct_ident_inst {
                fn get_qmetatype() -> crate::QMetaType {
                    #impl_code
                }
            }
        };

        Some(trait_code)
    }

    fn get_traits_rust_code(traits_substituted: &[TraitImpl]) -> syn::Result<TokenStream> {
        let mut result = TokenStream::new();

        for trait_ in traits_substituted {
            let prefix = trait_.get_inline_trait_functions_default_prefix()?;
            trait_.get_rust_code(&prefix)?.to_tokens(&mut result);
        }

        Ok(result)
    }

    pub fn generate_cpp(&self) -> syn::Result<(String, String)> {
        let submodule_name = self.submod_name();
        let module = self.module();
        let struct_ = self.structure();

        let include_guard_id = format!("_{submodule_name}_RUST_BRIDGE_H_").to_ascii_uppercase();
        let bridge_namespace = naming::cpp::namespace::type_bridge(&submodule_name);

        let namespace = struct_
            .and_then(|s| s.namespace());
        let maybe_using_namespace = namespace
            .map(|ns| format!("using namespace {ns};\n"))
            .unwrap_or_default();

        let mut includes = type_tokens_to_cpp_includes(self.type_tokens().all())?;
        includes.extend(module.cpp_includes().iter().cloned());
        includes.insert(CppInclude::new_in_quotes("rust/cxx.h"));
        // Remove include of self that may come from type tokens
        if let Some(struct_ident) = self.struct_ident() &&
           let Some(mono) = QtMonomorphedType::find_by_name(&struct_ident.to_string()) &&
           let Some(include) = mono.cpp_include() {
           includes.retain(|cpp_include| cpp_include.path_with_delims() != include);
        }

        let includes_str = includes.iter()
            .fold(String::new(), |mut acc, i| {
                acc.push_str(&i.to_cpp_code());
                acc
            });

        let maybe_struct_inst_alias = self.get_struct_inst_alias()?;

        let (cpp_trait_decl, cpp_trait_def) = self.get_def_cpp_traits_cpp_code()
            .unwrap_or_default();
        let (qmetatype_get_trait_decl, qmetatype_get_trait_def) = self.get_qmetatype_get_trait_cpp_code()
            .unwrap_or_default();
        let maybe_qmetatype_id_check = self.get_static_qmetatype_id_check_cpp_code()
            .unwrap_or_default();
        let (cpp_func_decl, cpp_func_def) = self.get_inline_cpp_functions_cpp_code()?;
        let maybe_reallocatable_struct = self.generate_is_relocatable();

        // Generate header code
        let header = format!(
r#"#ifndef {include_guard_id}
#define {include_guard_id}

{includes_str}

{maybe_struct_inst_alias}

namespace {bridge_namespace} {{

{maybe_using_namespace}

{cpp_trait_decl}
{qmetatype_get_trait_decl}
{cpp_func_decl}

}} // namespace {bridge_namespace}

{maybe_reallocatable_struct}
#endif // {include_guard_id}
"#);

        // Generate code of cpp file
        let header_name = naming::cpp::filename::type_gen_header(&submodule_name);
        let cpp = format!(
r#"#include "{header_name}"

namespace {bridge_namespace} {{

{maybe_using_namespace}
{maybe_qmetatype_id_check}
{cpp_trait_def}
{qmetatype_get_trait_def}
{cpp_func_def}

}} // namespace {bridge_namespace}

"#);
        Ok((header, cpp))
    }

    fn get_struct_inst_alias(&self) -> syn::Result<String> {
        let Some(struct_) = self.structure() else {
            return Ok(String::new())
        };

        if !struct_.is_generic() {
            return Ok(String::new())
        }

        let gen_cpp_types = struct_.generics().list().iter()
            .map(|ident| {
                let rust_type = self.type_map.get_impl().map(ident)
                    .ok_or_else(|| syn::Error::new(ident.span(), format!("Substitution for type {ident} was not found")))?;
                let cpp_type = type_to_cpp(&rust_type)
                    .map_err(|err| syn::Error::new(err.span(), format!("Type {ident} can't be used in generic type bridge. Error: {err}")))?;
                Ok(cpp_type)
            })
            .collect::<syn::Result<Vec<_>>>()?
            .join(", ");

        let mono_ident = self.struct_ident().unwrap();
        let src_struct_ident = self.src_struct_ident().unwrap();
        let maybe_namespace_w_colons = struct_.namespace()
            .map(|ns| format!("{ns}::"))
            .unwrap_or_default();
        let result = format!("using {mono_ident} = ::{maybe_namespace_w_colons}{src_struct_ident}<{gen_cpp_types}>;");
        Ok(result)
    }

    pub fn generate_is_relocatable(&self) -> String {
        if !self.is_shared_struct() {
            return "".into();
        }

        let ident = self.struct_ident().unwrap();

        // C++ define used to avoid multiple definitions for the same type
        // when instantiating types that are distinct in Rust but considered the same in C++,
        // e.g. QList<uint64_t> and QList<size_t>.
        let mut maybe_guard_begin = String::new();
        let mut maybe_guard_end = String::new();
        let define_needed = self.structure()
            .is_some_and(|s| s.is_generic());
        if define_needed {
            let mut define_ident = ident.to_string()
                .split('_')
                .into_iter()
                .map(|comp| {
                    #[cfg(target_pointer_width = "64")]
                    match comp {
                        "usize" => "u64",
                        "isize" => "i64",
                        _ => comp
                    }
                    #[cfg(target_pointer_width = "32")]
                    match comp {
                        "usize" => "u32",
                        "isize" => "i32",
                        _ => comp
                    }
                })
                .collect::<Vec<_>>()
                .join("_")
                .to_ascii_uppercase();
            define_ident.push_str("_IS_RELOCATABLE");
            maybe_guard_begin = format!(
r#"
#ifndef {define_ident}
#define {define_ident}
"#);
            maybe_guard_end = format!("#endif // #ifndef {define_ident}");
        }

        let maybe_namespace_w_colons = self.structure()
            .and_then(|s| s.namespace())
            .map(|ns| format!("{ns}::"))
            .unwrap_or_default();
        let code = format!(
r#"
{maybe_guard_begin}
namespace rust {{

template <>
struct IsRelocatable<::{maybe_namespace_w_colons}{ident}> : ::std::true_type {{}};

 }} // namespace rust
{maybe_guard_end}
"#);
        code
    }

    fn get_def_cpp_traits_cpp_code(&self) -> Option<(String, String)> {
        let struct_ = self.structure()?;
        let ident = self.src_struct_ident()?;
        let ident_str = ident.to_string();
        let ident_inst = self.struct_ident()?;

        let mut decl = String::new();
        let mut def = String::new();

        if struct_.is_trait_cpp_derived("Drop") {
            let func_name = naming::cpp::function::drop(&ident_str);
            let sig = format!("void {func_name}({ident_inst}& v)");
            decl.push_str(&format!("{sig};\n"));
            def.push_str(&format!(r#"
{sig}
{{
    v.~{ident_inst}();
}}
"#));
        }

        if struct_.is_trait_cpp_derived("Default") {
            let func_name = naming::cpp::function::default(&ident_str);
            let sig = format!("{ident_inst} {func_name}()");
            decl.push_str(&format!("{sig};\n"));
            def.push_str(&format!(r#"
{sig}
{{
    return {ident_inst}();
}}
"#));
        }

        if struct_.is_trait_cpp_derived("Clone") {
            let func_name = naming::cpp::function::clone(&ident_str);
            let sig = format!("{ident_inst} {func_name}(const {ident_inst}& src)");
            decl.push_str(&format!("{sig};\n"));
            def.push_str(&format!(r#"
{sig}
{{
    return {{src}};
}}
"#));
        }

        Some((decl, def))
    }

    fn get_qmetatype_get_trait_cpp_code(&self) -> Option<(String, String)> {
        if !self.is_qmetatypeid_func_needed() {
            return None
        }

        let struct_name_src = self.src_struct_ident()?;
        let struct_name_inst = self.struct_ident()?;
        let func_name = naming::cpp::function::qmetatype(&struct_name_src);
        let sig = format!("QMetaType {func_name}()");

        let decl = format!("{sig};");
        let def = format!(r#"
{sig}
{{
        return QMetaType::fromType<{struct_name_inst}>();
}}
"#);
        Some((decl, def))
    }

    fn get_static_qmetatype_id_check_cpp_code(&self) -> Option<String> {
        let id = self.qmetatype_id()?;
        if id <= 0 {
            return None
        }

        let ident = self.struct_ident()?;
        Some(format!("static_assert(qMetaTypeId<{ident}>() == {id});"))
    }

    fn get_inline_cpp_functions_cpp_code(&self) -> syn::Result<(String, String)> {

        let mut decls = String::new();
        let mut defs = String::new();

        for function in &self.funcs_substituted {
            let func_prefix = Function::get_inline_functions_default_prefix();
            let (decl, def) = function.get_cpp_funcs_cpp_code(&func_prefix)?;
            decls.push_str(&format!("\n{decl}"));
            defs.push_str(&format!("\n{def}"));
        }

        for trait_ in &self.traits_substituted {
            let trait_func_prefix = trait_.get_inline_trait_functions_default_prefix()?;

            for function in trait_.functions() {
                let (decl, def) = function.get_cpp_funcs_cpp_code(&trait_func_prefix)?;
                decls.push_str(&format!("\n{decl}"));
                defs.push_str(&format!("\n{def}"));
            }
        }

        Ok((decls, defs))
    }

    pub fn collect_type_tokens(&mut self) -> syn::Result<()>{
        let mut tokens = SubmoduleTypeTokens::default();

        tokens.collect_from_functions(&self.funcs_substituted)?;
        tokens.collect_from_traits(&self.traits_substituted)?;

        if let Some(struct_) = self.module().structure() {
            // Remove self under different names
            if let Some(src_ident) = self.struct_ident() {
                tokens.remove_qt_and_unclassified(&src_ident.clone().into());
            }
            if let Some(src_struct_ident) = self.src_struct_ident() {
                tokens.remove_qt_and_unclassified(&src_struct_ident.clone().into());
            }
            if struct_.is_generic() {
                let self_path_w_args = struct_.get_path_instantiated(self.type_map.get_impl())?;
                tokens.remove_unclassified(&self_path_w_args);
            }

            if self.is_qmetatypeid_func_needed() {
                // We generate bridge function that returns QMetaType for given type
                tokens.all_mut().insert_ident_type(format_ident!("QMetaType"));
            }
        }

        self.type_tokens = tokens;
        Ok(())
    }

    pub fn get_non_bridge_reexport(&self) -> syn::Result<Reexport> {
        // TODO: other items must have substituted types as well
        let mut result = Reexport::new();
        self.src_module.other_items().iter()
            .try_for_each(|item| result.collect_from_item(item))?;

        Ok(result)
    }

    pub fn get_unresolved_dependencies(&self) -> Vec<syn::Path> {
        get_unresolved_type_dependencies(self.type_tokens())
    }
}
