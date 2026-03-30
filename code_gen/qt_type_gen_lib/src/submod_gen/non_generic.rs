// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::rc::Rc;

use proc_macro2::{Span, TokenStream};
use quote::quote;

use qtbridge_gen_common::type_registry::type_traits::MetaTypeId;
use qtbridge_gen_common::type_registry::QtType;
use qtbridge_gen_common::type_registry::qt::non_generic::QtNonGenericType;

use crate::function::Function;
use crate::module::Module;
use crate::reexport::Reexport;
use crate::submod_gen::common::file_path_str_to_qualified_path_before_name;

use super::common::get_submod_ident;
use super::non_generic_base::NonGenericSubmoduleGeneratorBase;
use super::submodule_generator::SubmoduleGenerator;

/// Generator for struct that has no generic parameters
/// (but traits implementation for structure are still allowed to have generic parameters)
/// or the list of functions without struct at all
pub struct NonGenericSubmoduleGenerator {
    base: NonGenericSubmoduleGeneratorBase,
}

impl NonGenericSubmoduleGenerator {
    pub fn new(src_module: Rc<Module>, input_file_path: &str) -> syn::Result<Self> {
        let mut struct_ident = None;
        if let Some(struct_) = src_module.structure() {
            struct_ident = Some(struct_.ident().clone());
        }
        let submod_ident = get_submod_ident(src_module.as_ref(), None)?;

        let base = NonGenericSubmoduleGeneratorBase::new(src_module, input_file_path.into(), submod_ident, struct_ident, None)?;

        let mut instance = Self {
            base,
        };
        instance.collect_type_tokens()?;

        Ok(instance)
    }

    fn path_in_gen(&self) -> syn::Result<String> {
        file_path_str_to_qualified_path_before_name(&self.base.input_file_path())
            .map_err(|err| {
                let span = self.base.struct_ident()
                    .map(|i| i.span())
                    .unwrap_or(Span::call_site());
                syn::Error::new(span, format!("Failed to get path of concrete type: {err}"))
            })
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
        let functions = self.get_functions_rust_code()?;
        let mut code = quote! {
            #(#[allow(dead_code)] #functions)*
        };

        if self.base.structure().is_some() {
            let ident = self.base.struct_ident();

            code = quote! {
                impl #ident {
                    #code
                }
            };
        }

        Ok(code)
    }

    fn get_functions_rust_code(&self) -> syn::Result<Vec<syn::ItemFn>> {
        let prefix = Function::get_inline_functions_default_prefix();
        let result = self.base.functions()
            .map(|func| func.get_rust_func(&prefix))
            .collect();
        Ok(result)
    }

    fn collect_type_tokens(&mut self) -> syn::Result<()> {
        self.base.collect_type_tokens()
    }
}

impl SubmoduleGenerator for NonGenericSubmoduleGenerator {
    fn generate_rust(&self) -> syn::Result<TokenStream> {
        let ffi_mod_block = self.get_mod_ffi_block()?;
        let after_ffi_mod_block = self.get_after_mod_ffi_code()?;

        Ok(quote! {
            #ffi_mod_block
            #after_ffi_mod_block
        })
    }

    fn generate_cpp(&self) -> syn::Result<(String, String)> {
        self.base.generate_cpp()
    }

    fn register_type(&self) -> syn::Result<()> {
        let Some(ident) = self.base.struct_ident() else {
            return Ok(())
        };
        let metatype = match self.base.qmetatype_id() {
            Some(id) =>
                match id {
                    1.. => MetaTypeId::Constant(id),
                    _ => MetaTypeId::Runtime,
                }
            None => MetaTypeId::None
        };

        let ns = self.base.namespace()
            .unwrap_or_default();
        QtType::add_concrete(QtNonGenericType::new(ident.to_string(), self.path_in_gen()?, metatype, ns));

        Ok(())
    }

    fn submod_name(&self) -> String {
        self.base.submod_name()
    }

    fn input_file_path(&self) -> String {
        self.base.input_file_path()
    }

    fn check_unclassified_type_tokens(&mut self) -> syn::Result<()> {
        self.base.check_unclassified_type_tokens()
    }

    fn substitute_monomorphed_types_if_needed(&mut self) -> syn::Result<()> {
        // Nothing to do for non generic struct
        Ok(())
    }

    fn get_non_bridge_reexport(&self) -> syn::Result<Reexport> {
        self.base.get_non_bridge_reexport()
    }

    fn get_unresolved_type_dependencies(&self) -> Vec<syn::Path> {
        self.base.get_unresolved_dependencies()
    }

    fn is_cxx_present(&self) -> bool {
        if self.base.structure().is_some() {
            return true
        }

        self.base.has_inline_cpp_functions()
    }
}
