// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::collections::{BTreeMap, HashSet};
use std::rc::Rc;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;

use qtbridge_gen_common::signature_utils::get_arg_ident;
use qtbridge_gen_common::type_registry::QtType;
use qtbridge_gen_common::type_to_string::{angle_bracketed_generic_arguments_to_string, path_to_string};
use qtbridge_gen_common::type_utils::{are_all_args_generic_idents, get_angle_bracketed_generic_arguments_of_last_path_segment, get_ident_of_last_path_segment};
use qtbridge_gen_common::type_tokens::TypeTokens;
use qtbridge_gen_common::type_registry::qt::generic::QtGenericTypeWithoutArgs;

use crate::function::Function;
use crate::generic_idents::GenericIdents;
use crate::module::Module;
use crate::reexport::Reexport;
use crate::structure::{BridgeStruct, StructureField};
use crate::submod_gen::common::{file_path_str_to_qualified_path_before_name, get_derive_attr};
use crate::submodule_type_tokens::SubmoduleTypeTokens;

use super::common::get_unresolved_type_dependencies;
use super::submodule_generator::SubmoduleGenerator;

/// Generator for the main (generic) submodule of a generic structure.
/// In this submodule we put:
/// * declaration of generic struct
/// * declaration of the 'Impl' trait (containing all the functions of struct
///   that needs to be implemented specifically per monomorphization)
/// * functions of struct calling corresponding functions of the 'Impl' trait
pub struct GenericSubmoduleGenerator {
    src_module: Rc<Module>,
    input_file_path: String,
    type_tokens: SubmoduleTypeTokens,
}

impl GenericSubmoduleGenerator {
    pub fn new(src_module: Rc<Module>, input_file_path: &str) -> syn::Result<Self> {
        let mut instance = Self {
            src_module,
            input_file_path: input_file_path.into(),
            type_tokens: SubmoduleTypeTokens::default(),
        };
        instance.collect_type_tokens()?;
        Ok(instance)
    }

    fn module(&self) -> &Module {
        &self.src_module
    }

    fn path_before_name(&self) -> syn::Result<String> {
        file_path_str_to_qualified_path_before_name(&self.input_file_path)
            .map_err(|err| syn::Error::new(self.struct_ident().span(), format!("Failed to get qualified path of generic type: {err}")))
    }

    fn structure(&self) -> &BridgeStruct {
        self.module().structure()
            .expect("structure is None")
    }

    fn struct_ident(&self) -> &syn::Ident {
        self.structure().ident()
    }

    /// Get the name of Impl trait
    fn impl_ident(&self) -> syn::Ident {
        let struct_ident = self.struct_ident();
        let name = format!("{struct_ident}Impl");
        syn::Ident::new(&name, struct_ident.span())
    }

    fn generic_list_tokens(&self) -> Option<TokenStream> {
        let generic_list = self.generics().list();
        (!generic_list.is_empty())
            .then(|| quote! { #(#generic_list),* })
    }

    fn generic_list_tokens_in_angle_brackets(&self) -> Option<TokenStream> {
        let mut tokens = self.generic_list_tokens()?;
        if !tokens.is_empty() {
            tokens = quote! { <#tokens>};
        }
        Some(tokens)
    }

    fn generics(&self) -> &GenericIdents {
        self.structure().generics()
    }

    fn get_rust_code(&self) ->syn::Result<TokenStream> {
        let bounds = self.get_additional_generic_type_bounds()?;

        let struct_repr = self.get_struct_repr_code(&bounds)?;
        let func_block = self.get_func_block(&bounds)?;
        let impl_trait = self.get_impl_trait(&bounds);
        let drop_trait = self.get_drop_trait(&bounds);

        Ok(quote! {
            #struct_repr
            #func_block
            #impl_trait
            #drop_trait
        })
    }

    /// Return additional trait bounds that need to be added to:
    /// * struct declaration.
    /// * impl block.
    /// * 'implementation' trait declaration.
    /// * Drop impl.
    ///
    /// To determine resulting set of bounds we need to find all generic types
    /// using generics from this struct. E.g.
    /// QHash<K, V>::keys() -> QList<K>
    /// will add requirement
    /// QList<K>: crate::QListImpl<K>,
    /// to bounds of QHash impls.
    fn get_additional_generic_type_bounds(&self) -> syn::Result<TokenStream> {
        let struct_gen_idents = self.generics().list();
        let mut tokens = TypeTokens::default();
        self.get_functions_of_impl_trait()
            .try_for_each(|func| tokens.collect_from_signature(func.signature()))?;

        let gen_types: HashSet<_> = tokens.iter_unclassified()
            .filter(|path| are_all_args_generic_idents(path, struct_gen_idents))
            .cloned()
            .collect();

        let map: BTreeMap<String, TokenStream> = gen_types.iter()
            .map(|path| {
                let path_str = path_to_string(path)?;
                let gen_ident = get_ident_of_last_path_segment(path)
                    .ok_or_else(|| syn::Error::new(path.span(), "Failed to get path's ident"))?;
                let gen_args = get_angle_bracketed_generic_arguments_of_last_path_segment(path)
                    .ok_or_else(|| syn::Error::new(path.span(), "Supposed to be path with angle bracketed arguments"))?;
                let args_str = angle_bracketed_generic_arguments_to_string(gen_args)?;
                let str = format!("{path_str}: crate::{gen_ident}Impl<{args_str}>");
                let tokens = syn::parse_str(&str)?;
                Ok((path_str, tokens))
            })
            .collect::<syn::Result<_>>()?;
        let bound_predicates = map.into_values();

        Ok(quote! { #(#bound_predicates),* })
    }

    fn get_struct_repr_code(&self, additional_bounds: &TokenStream) -> syn::Result<TokenStream> {
        let struct_ = self.structure();

        let docs = struct_.docs();
        let derive_attr = get_derive_attr(struct_.derived_traits());
        let ident = struct_.ident();
        let impl_ident = self.impl_ident();
        let generics = self.generic_list_tokens_in_angle_brackets();

        let fields = struct_.fields();
        if fields.is_empty() {
            return Err(syn::Error::new(ident.span(), "Generic structure must have specified fields"))
        }

        // Get phantom field mentioning generic types not used in fields
        let phantom_field = self.get_phantom_field(fields)?;

        let struct_repr = quote! {
            #(#docs)*
            #derive_attr
            #[repr(C)]
            pub struct #ident #generics
                where Self: #impl_ident #generics,
                #additional_bounds
            {
                #(#fields,)*
                #phantom_field
            }
        };

        Ok(struct_repr)
    }

    fn get_phantom_field(&self, fields: &[StructureField]) -> syn::Result<TokenStream> {
        let mut type_tokens = TypeTokens::default();
        // Iterate types mentioned in fields
        fields.iter()
            .try_for_each(|field| type_tokens.collect_from_type(field.get_type()))?;

        // Collect types unmentioned in struct fields
        let phantom_types_vec: Vec<_> = self.generics().list().iter()
            .filter(|gen_ident| {
                let gen_path: syn::Path = (*gen_ident).clone().into();
                !type_tokens.contains_unclassified(&gen_path)
            })
            .cloned()
            .collect();

        if phantom_types_vec.is_empty() {
            // No need in phantom field
            return Ok(TokenStream::new())
        }

        let mut phantom_types = quote! { #(#phantom_types_vec),* };
        if phantom_types_vec.len() > 1 {
            phantom_types = quote! { (#phantom_types) };
        }
        Ok(quote! {
            phantoms: core::marker::PhantomData<#phantom_types>,
        })
    }

    fn get_func_block(&self, additional_bounds: &TokenStream) -> syn::Result<Option<TokenStream>> {
        let functions_code = self.get_functions_rust_code()?;
        let maybe_code = (!functions_code.is_empty())
            .then(|| {
                let ident = self.struct_ident();
                let impl_ident = self.impl_ident();
                let generics = self.generic_list_tokens_in_angle_brackets();
                quote! {
                    impl #generics #ident #generics
                        where Self: #impl_ident #generics,
                        #additional_bounds
                    {
                        #functions_code
                    }
                }
            });
        Ok(maybe_code)
    }

    fn get_functions_rust_code(&self) -> syn::Result<TokenStream> {
        let impl_ident = self.impl_ident();
        let generics = self.generic_list_tokens_in_angle_brackets();
        let prefix = Function::get_inline_functions_default_prefix();
        let mut result = TokenStream::new();
        for function in self.module().functions() {
            let func_tokens = if !function.cpp_functions().is_empty() {
                let docs = function.docs();
                let vis = function.visibility();
                let sign = function.signature();
                let sign_ident = &sign.ident;
                let args = sign.inputs.iter()
                    .map(get_arg_ident)
                    .collect::<syn::Result<Vec<_>>>()?;
                quote! {
                    #(#docs)*
                    #vis #sign {
                        <Self as #impl_ident #generics>::#sign_ident(#(#args),*)
                    }
                }
            }
            else {
                function.get_rust_func(&prefix)?
                    .to_token_stream()
            };
            func_tokens.to_tokens(&mut result);
        }

        Ok(result)
    }

    fn get_drop_trait(&self, additional_bounds: &TokenStream) -> TokenStream {
        let struct_ = self.structure();
        if !struct_.is_trait_cpp_derived("Drop") {
            return TokenStream::new()
        }

        let ident = struct_.ident();
        let impl_ident = self.impl_ident();
        let generics = self.generic_list_tokens_in_angle_brackets();

        quote! {
            impl #generics Drop for #ident #generics
                where Self: #impl_ident #generics,
                #additional_bounds
            {
                fn drop(&mut self) {
                    <Self as #impl_ident #generics>::do_drop(self)
                }
            }
        }
    }

    fn get_impl_trait(&self, additional_bounds: &TokenStream) -> TokenStream {
        let impl_ident = self.impl_ident();
        let generics = self.generic_list_tokens_in_angle_brackets();
        let funcs = self.get_functions_of_impl_trait()
            .map(|f| f.signature());
        let maybe_do_drop = self.structure()
            .is_trait_cpp_derived("Drop")
            .then_some(quote! { fn do_drop(&mut self); });
        let maybe_bounds = (!additional_bounds.is_empty())
            .then(|| quote! {
                where #additional_bounds
            });

        quote! {
            #[doc(hidden)]
            pub trait #impl_ident #generics
            #maybe_bounds
            {
                #(#funcs;)*
                #maybe_do_drop
            }
        }
    }

    /// Get a list of function that need to be put to Impl trait
    fn get_functions_of_impl_trait(&self) -> impl Iterator<Item = &Function> {
        // If function has inline C++ block - put it to the 'Impl' trait
        // Maybe logic must be more complicate here
        self.module().functions().iter()
            .filter(|f| !f.cpp_functions().is_empty())
    }

    fn collect_type_tokens(&mut self) -> syn::Result<()> {
        let module = self.module();
        let struct_ = self.structure();
        let generics = struct_.generics().list();

        let mut tokens = SubmoduleTypeTokens::new_for_generic(generics);
        tokens.collect_from_functions(module.functions())?;
        tokens.collect_from_traits(module.traits())?;
        tokens.remove_self();

        // Remove self under different names
        let struct_ident = struct_.ident();
        tokens.remove_qt_and_unclassified(&struct_ident.clone().into());
        let gen_arg_str = generics
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",");
        let path_w_args_str = format!("{struct_ident}<{gen_arg_str}>");
        let path_w_args_path = syn::parse_str(&path_w_args_str)?;
        tokens.remove_qt_and_unclassified(&path_w_args_path);

        self.type_tokens = tokens;
        Ok(())
    }

}

impl SubmoduleGenerator for GenericSubmoduleGenerator {
    fn generate_rust(&self) -> syn::Result<proc_macro2::TokenStream> {
        self.get_rust_code()
    }

    fn generate_cpp(&self) -> syn::Result<(String, String)> {
        Ok((String::new(), String::new()))
    }

    fn register_type(&self) -> syn::Result<()> {
        let struct_ = self.structure();
        let args = struct_.generics()
            .list()
            .iter()
            .map(ToString::to_string)
            .collect();
        QtType::add_generic(QtGenericTypeWithoutArgs::new(struct_.ident().to_string(), self.path_before_name()?, args));

        Ok(())
    }

    fn submod_name(&self) -> String {
        self.struct_ident().to_string().to_ascii_lowercase()
    }

    fn input_file_path(&self) -> String {
        self.input_file_path.clone()
    }

    fn check_unclassified_type_tokens(&mut self) -> syn::Result<()> {
        self.type_tokens.check_unclassified()
    }

    fn substitute_monomorphed_types_if_needed(&mut self) -> syn::Result<()> {
        // Do not substitute for generic type.
        // Should still compile and will looks better in generic form
        Ok(())
    }

    fn get_non_bridge_reexport(&self) -> syn::Result<Reexport> {
        // TODO: other items must have types substituted as well
        let mut result = Reexport::new();
        self.src_module.other_items().iter()
            .try_for_each(|item| result.collect_from_item(item))?;

        Ok(result)
    }

    fn get_unresolved_type_dependencies(&self) -> Vec<syn::Path> {
        get_unresolved_type_dependencies(&self.type_tokens)
    }

    fn is_cxx_present(&self) -> bool {
        false
    }

}
