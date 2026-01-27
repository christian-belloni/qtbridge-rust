// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::rc::Rc;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use qt_gen_common_no_types::naming;
use qt_gen_common_no_types::parse_utils::is_doc_attribute;
use qt_gen_common_no_types::type_mapping::TypeMapping;
use qt_gen_common_no_types::type_registry;
use qt_gen_common_no_types::type_to_string::path_to_string_fallback;
use type_registry::QtType;
use type_registry::qt::generic::QtGenericArg;
use type_registry::type_traits::{FindType, TypeName};

use crate::function::Function;
use crate::generic_instantiation_decl::GenericInstantiationDecl;
use crate::module::Module;
use crate::reexport::Reexport;
use crate::structure::BridgeStruct;
use crate::submod_gen::common::file_path_str_to_qualified_path_before_name;

use super::common::{get_monomorped_struct_ident, get_submod_ident};
use super::non_generic_base::NonGenericSubmoduleGeneratorBase;
use super::submodule_generator::SubmoduleGenerator;

/// Generator for submodule containing code for monomorphed version of generic structure:
/// * type alias for struct with generic types specified
/// * implementation of 'Impl' trait
/// * CXX code needed for the concrete implementation
pub struct MonomorphedSubmoduleGenerator {
    base: NonGenericSubmoduleGeneratorBase,
    inst: GenericInstantiationDecl,
}

impl MonomorphedSubmoduleGenerator {
    pub fn new(src_module: Rc<Module>, input_file_path: &str, inst: &GenericInstantiationDecl) -> syn::Result<Self> {
        let struct_ = src_module.structure()
            .ok_or_else(|| syn::Error::new(src_module.ident().span(), "Expected module with structure"))?;
        let submod_ident = get_submod_ident(src_module.as_ref(), Some(inst.types()))?;
        let struct_ident = get_monomorped_struct_ident(struct_, inst.types())?;

        let base = NonGenericSubmoduleGeneratorBase::new(src_module, input_file_path.into(), submod_ident, Some(struct_ident), Some(inst))?;

        let mut instance = Self {
            base,
            inst: inst.clone(),
        };
        instance.collect_type_tokens()?;

        Ok(instance)
    }

    fn structure(&self) -> &BridgeStruct {
        self.base.structure().unwrap()
    }

    pub fn ident(&self) -> &syn::Ident {
        self.base.struct_ident().unwrap()
    }

    fn src_struct_ident(&self) -> &syn::Ident {
        self.structure().ident()
    }

    fn impl_ident(&self) -> syn::Ident {
        let struct_ident = self.structure().ident();
        let name = format!("{struct_ident}Impl");
        syn::Ident::new(&name, struct_ident.span())
    }

    fn path_in_gen(&self) -> syn::Result<String> {
        file_path_str_to_qualified_path_before_name(&self.base.input_file_path())
            .map_err(|err| syn::Error::new(self.ident().span(), format!("Failed to get path of monomorphed type: {err}")))
    }

    fn get_imports_for_generics(&self) -> syn::Result<TokenStream> {

        let impl_ident = self.impl_ident();
        let mut types = vec![self.src_struct_ident().clone(), impl_ident];

        for (_, gen_path) in self.base.type_map().get_impl().iter() {
            let ty = type_registry::Type::find_by_partial_path_result(gen_path)?;
            let qt_type = match ty {
                type_registry::Type::Qt(qt_type) => qt_type,
                _ => continue
            };
            types.push(format_ident!("{}", qt_type.name()));
        }
        types.sort_unstable();

        Ok(quote!{
            use crate::{#(#types),*};
        })
    }

    fn get_type_alias_code(&self) -> syn::Result<TokenStream> {
        let mono_ident = self.base.struct_ident();
        let ident = self.src_struct_ident();
        let types = self.get_generic_types()?;

        let need_allow_non_camel_case = self.structure()
            .generics()
            .list()
            .iter()
            .any(|i| i.to_string()
                .chars()
                .next()
                .is_some_and(|ch| ch.is_ascii_uppercase()));
        let maybe_allow_non_camel_case = need_allow_non_camel_case
            .then_some(quote! { #[allow(non_camel_case_types)] });
        let types_str = types.iter()
            .map(|path| format!("[{}]", path_to_string_fallback(path)))
            .collect::<Vec<_>>()
            .join(", ");
        let type_or_types_str = match types.len() {
            1 => "type",
            _ => "types",
        };
        let mono_docs = format!(" This is a monomorphized form of type [{ident}] for {type_or_types_str} {types_str}.");

        let maybe_user_defined_alias = self.inst.alias()
            .map(|alias_ident| {
                let alias_docs = format!(" This is an alias for type [{ident}] for {type_or_types_str} {types_str}.");
                quote!{
                    #[doc = #alias_docs]
                    pub type #alias_ident = #ident<#(#types),*>;
                }
            });


        Ok(quote! {
            #maybe_allow_non_camel_case
            #[doc = #mono_docs]
            pub type #mono_ident = #ident<#(#types),*>;

            #maybe_user_defined_alias
        })
    }

    fn get_generic_types(&self) -> syn::Result<Vec<syn::Path>> {
        let struct_ = self.structure();
        struct_.generics()
            .list().iter()
            .map(|gen_ident| self.base.type_map().get_impl()
                .map(gen_ident)
                .ok_or_else(|| syn::Error::new(gen_ident.span(), format!("Failed to map generic param '{gen_ident}'")))
            )
            .collect::<syn::Result<Vec<_>>>()
    }

    fn get_mod_ffi_block(&self) -> syn::Result<TokenStream> {
        let ffi_content = self.base.get_ffi_mod_content()?;

        Ok(quote! {
            #[cxx::bridge]
            mod ffi {
                #ffi_content
            }
        })
    }

    fn get_after_mod_ffi_code(&self) ->syn::Result<TokenStream> {

        let base_content = self.base.get_common_content_after_ffi_block()?;
        let func_block = self.get_func_block()?;

        Ok(quote! {
            #base_content
            #func_block
        })
    }

    fn get_func_block(&self) -> syn::Result<TokenStream> {
        let mut funcs = self.get_functions_rust_code()?;
        // Remove pub visibility - it's not allowed for trait function
        for func in &mut funcs {
            func.vis = syn::Visibility::Inherited;
        }

        let struct_ = self.structure();
        let maybe_do_drop = struct_.is_trait_cpp_derived("Drop")
            .then(|| {
                let func_name = naming::rust::function::drop(&self.src_struct_ident());
                quote! {
                    fn do_drop(&mut self) {
                        ffi::#func_name(self)
                    }
                }
            });

        let mut code = quote! {
            #(#funcs)*
            #maybe_do_drop
        };

        let impl_ident = self.impl_ident();
        let ident = self.base.struct_ident();
        let generic_types = self.get_generic_types()?;

        code = quote! {
            impl #impl_ident <#(#generic_types),*> for #ident {
                #code
            }
        };

        Ok(code)
    }

    fn get_functions_rust_code(&self) -> syn::Result<Vec<syn::ItemFn>> {
        let prefix = Function::get_inline_functions_default_prefix();
        let result = self.base.functions()
            .filter(|func| !func.cpp_functions().is_empty())
            .map(|func| {
                let mut func = func.get_rust_func(&prefix);
                // Remove doc attributes because they belong to generic file
                // but not to the monomorped one.
                func.attrs.retain(|attr| !is_doc_attribute(attr));
                func
            })
            .collect();
        Ok(result)
    }

    fn collect_type_tokens(&mut self) -> syn::Result<()> {
        self.base.collect_type_tokens()
    }
}

impl SubmoduleGenerator for MonomorphedSubmoduleGenerator {
    fn generate_rust(&self) -> syn::Result<TokenStream> {
        let generic_imports = self.get_imports_for_generics()?;
        let type_alias = self.get_type_alias_code()?;
        let ffi_mod_block = self.get_mod_ffi_block()?;
        let after_ffi_mod_block = self.get_after_mod_ffi_code()?;

        Ok(quote! {
            #generic_imports
            #type_alias
            #ffi_mod_block
            #after_ffi_mod_block
        })
    }

    fn generate_cpp(&self) -> syn::Result<(String, String)> {
        self.base.generate_cpp()
    }

    fn submod_name(&self) -> String {
        self.base.submod_name()
    }

    fn input_file_path(&self) -> String {
        self.base.input_file_path()
    }

    fn register_type(&self) -> syn::Result<()> {
        let generic_args = self.get_generic_types()?
            .iter()
            .map(QtGenericArg::try_from)
            .collect::<syn::Result<_>>()?;
        let monomorped_name = self.ident().to_string();
        let path_in_gen = self.path_in_gen()?;
        let qmetatype_id = self.inst.qmetatype_id()
            .unwrap_or(0);
        QtType::add_monomorphed(monomorped_name.clone(), self.src_struct_ident(), generic_args, path_in_gen, qmetatype_id.into())?;

        if let Some(alias) = self.inst.alias() {
            QtType::add_alias_to_monomoprhed(alias.to_string(), monomorped_name, self.path_in_gen()?, qmetatype_id.into());
        }

        Ok(())
    }

    fn check_unclassified_type_tokens(&mut self) -> syn::Result<()> {
        self.base.check_unclassified_type_tokens()
    }

    fn substitute_monomorphed_types_if_needed(&mut self) -> syn::Result<()> {
        self.base.traits_mut()
            .try_for_each(|tr| tr.substitute_generic_qt_types_in_cpp_functions())?;
        self.base.functions_mut()
            .try_for_each(|func| func.substitute_generic_qt_types_in_cpp_functions())
    }

    fn get_non_bridge_reexport(&self) -> syn::Result<Reexport> {
        self.base.get_non_bridge_reexport()
    }

    fn get_unresolved_type_dependencies(&self) -> Vec<syn::Path> {
        self.base.get_unresolved_dependencies()
    }

    fn is_cxx_present(&self) -> bool {
        true
    }
}
