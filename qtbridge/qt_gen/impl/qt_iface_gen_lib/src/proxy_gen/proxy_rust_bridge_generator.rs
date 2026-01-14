// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::TokenStream;
use quote::quote;

use qt_gen_common::case_conv;
use qt_gen_common::function_bridge::RustFunctionBridge;
use qt_gen_common::naming;
use qt_gen_common::signature_utils::{change_arg_idents_in_signature_to_camel_case, change_types_in_signature_to_monomorphed};
use qt_gen_common::type_dependencies::{TypeImportInBridge, qt_types_to_bridge_imports};
use qt_gen_common::type_tokens::TypeTokens;

use crate::InterfaceDesc;

/// Struct responsible for generation of CXX bridge for generated Rust proxy.
pub struct RustProxyBridgeGenrator<'a> {
    iface: &'a InterfaceDesc,
}

impl<'a> RustProxyBridgeGenrator<'a> {
    /// Create a new instance.
    pub fn new(iface: &'a InterfaceDesc) -> Self {
        Self {
            iface
        }
    }

    /// Run the code generation.
    /// Produce a TokenStream that will be written to the dedicated 'proxy_rust_bridge.rs' file.
    /// See 'qtbridge/qt_ifaces/src/generated/*/proxy_rust_bridge.rs' for an example of the generated code.
    /// See diagrams in 'qtbridge/qt_ifaces/docs/uml/' illustrating the structure at a higher level.
    pub fn generate(&self) -> syn::Result<TokenStream> {
        let iface = self.iface;
        let iface_name = iface.get_ident().to_string();

        let struct_name = naming::rust::structure::proxy_rust(&iface_name);

        let non_interface_functions = self.generate_non_interface_functions()?;
        let virtual_functions = self.generate_virtual_functions()?;

        let signatures = virtual_functions.iter()
            .map(|f| f.signature());

        let bridge_imports = Self::generate_bridge_imports(signatures)?;

        let code = quote! {
            use super::proxy_rust::#struct_name;
            #[cxx::bridge]
            pub mod ffi {
                unsafe extern "C++" {
                    #(#bridge_imports)*
                }

                extern "Rust" {
                    type #struct_name;

                    #(#non_interface_functions)*
                    #(#virtual_functions)*
                }
            }

            unsafe impl cxx::ExternType for #struct_name {
                type Id = cxx::type_id!(#struct_name);
                type Kind = cxx::kind::Trivial;
            }
        };
        Ok(code)
    }

    /// Generate imports for types used in function signatures that goes
    /// to block 'unsafe extern "C++"' inside 'mod ffi'.
    fn generate_bridge_imports<'b>(mut signatures: impl Iterator<Item = &'b syn::Signature>) -> syn::Result<Vec<TypeImportInBridge>> {
        let mut tokens = TypeTokens::default();
        signatures.try_for_each(|sign| tokens.collect_from_signature(sign))?;
        qt_types_to_bridge_imports(tokens.iter_qt(), false)
    }

    /// Generate bridge for virtual functions of the interface.
    fn generate_virtual_functions(&self) -> syn::Result<Vec<RustFunctionBridge>> {
        let mut result = Vec::new();

        for method in self.iface.get_virtual_methods() {
            let mut sig = change_arg_idents_in_signature_to_camel_case(
                method.get_signature())?;
            change_types_in_signature_to_monomorphed(&mut sig)?;
            let cxx_name = case_conv::snake_to_camel(&sig.ident.to_string());
            result.push(RustFunctionBridge::new(cxx_name.into(), sig)?);
        }

        Ok(result)
    }

    // Generate bridge for non-virtual functions of the interface.
    fn generate_non_interface_functions(&self) -> syn::Result<Vec<RustFunctionBridge>> {
        let iface_name = self.iface.get_ident().to_string();
        let proxy_rust_name = naming::rust::structure::proxy_rust(&iface_name);
        let drop_self_rust_name = naming::rust::function::drop_self();
        let drop_self_cpp_name = naming::cpp::function::drop_self();

        let drop_self = RustFunctionBridge::new_associated_function(Some(proxy_rust_name.clone()), drop_self_cpp_name, syn::parse2(quote! {
            unsafe fn #drop_self_rust_name(self_ptr: *mut #proxy_rust_name, rust_obj_ptr: *const u8)
        })?)?;

        let result = [
            drop_self,
        ];
        Ok(result.into())
    }
}
