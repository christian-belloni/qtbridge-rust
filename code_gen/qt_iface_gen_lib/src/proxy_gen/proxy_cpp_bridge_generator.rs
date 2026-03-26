// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;

 use qt_gen_common::case_conv;
use qt_gen_common::function_bridge::CppFunctionBridge;
use qt_gen_common::{Naming, naming, format_naming};
use qt_gen_common::signature_utils::{change_arg_idents_in_signature_to_camel_case, is_self_mut};
use qt_gen_common::type_dependencies::{TypeImportInBridge, qt_types_to_bridge_imports};
use qt_gen_common::type_tokens::TypeTokens;

use crate::InterfaceDesc;

/// Struct responsible for generation of CXX bridge for generated C++ proxy.
pub struct CppProxyBridgeGenerator<'a> {
    iface: &'a InterfaceDesc,
}

impl<'a> CppProxyBridgeGenerator<'a> {
    /// Create a new instance.
    pub fn new(iface: &'a InterfaceDesc) -> Self {
        Self {
            iface
        }
    }

    /// Run the code generation.
    /// Produce a TokenStream that will be written to the dedicated 'proxy_cpp_bridge.rs' file.
    /// See 'qtbridge/qt_ifaces/src/generated/*/proxy_cpp_bridge.rs' for an example of the generated code.
    /// See diagrams in 'qtbridge/qt_ifaces/docs/uml/' illustrating the structure at a higher level.
    pub fn generate(&self) -> syn::Result<TokenStream> {
        let iface = self.iface;
        let iface_ident = iface.get_ident();
        let cpp_proxy_name = naming::cpp::class::proxy_cpp(iface_ident);
        let namespace = naming::cpp::namespace::bridge();

        let proxy_header_path = naming::cpp::path::proxy_header(&iface_ident);

        let create_functions = self.generate_create_functions()?;
        let implemented_functions = self.generate_implemented_functions()?;

        let signatures = create_functions.iter()
                .map(|f| f.signature())
            .chain(implemented_functions.iter()
               .map(|f| f.signature()));

        let rust_proxy_import = self.generate_rust_proxy_import()?;
        let bridge_imports = self.generate_bridge_imports(signatures)?;

        let code = quote! {
            #rust_proxy_import

            #[cxx::bridge]
            pub mod ffi {
                unsafe extern "C++" {
                    #(#bridge_imports)*
                }

                #[namespace = #namespace]
                unsafe extern "C++" {
                    include!(#proxy_header_path);
                    type #cpp_proxy_name;

                    #(#create_functions)*
                    #(#implemented_functions)*
                }
            }

            pub use ffi::#cpp_proxy_name;
        };
        Ok(code)
    }

    /// Generate block with imports that goes to the top of generated file.
    fn generate_rust_proxy_import(&self) -> syn::Result<syn::ItemUse> {
        let module = naming::rust::module::proxy_rust();
        let iface_name = self.iface.get_ident().to_string();
        let rust_proxy_name = naming::rust::structure::proxy_rust(&iface_name);
        let code = format!("use super::{module}::{rust_proxy_name};");
        syn::parse_str(&code)
    }

    /// Generate imports for types used in function signatures that goes
    /// to block 'unsafe extern "C++"' inside 'mod ffi'.
    fn generate_bridge_imports<'b>(&self, mut signatures: impl Iterator<Item = &'b syn::Signature>) -> syn::Result<Vec<TypeImportInBridge>> {
        let mut tokens = TypeTokens::default();
        signatures.try_for_each(|sign| tokens.collect_from_signature(sign))?;
        let mut result = qt_types_to_bridge_imports(tokens.iter_qt(), false)?;
        result.push(self.get_bridge_import_for_rust_proxy()?);
        Ok(result)
    }

    /// Get import for Rust proxy.
    fn get_bridge_import_for_rust_proxy(&self) -> syn::Result<TypeImportInBridge> {
        let iface_name = self.iface.get_ident().to_string();
        let module_path = naming::rust::path::generated_module_dir(&iface_name);
        let filename = naming::rust::filename::proxy_rust_bridge();
        let include = format!("\"{module_path}{filename}.h\"");
        let proxy_lhs = naming::rust::structure::proxy_rust(&iface_name);
        let proxy_rhs = format_naming!("super::{proxy_lhs}");

        TypeImportInBridge::new(&include, "", proxy_lhs.to_ident(), proxy_rhs.to_path())
    }

    /// Generate functions that will be used when constructing proxies in different scenarios
    /// (from the Rust side, from QML engine, from QMetaType system)
    /// and coupling them with Rust object.
    fn generate_create_functions(&self) -> syn::Result<Vec<CppFunctionBridge>> {
        let iface = self.iface;
        let iface_name = iface.get_ident();
        let cpp_proxy_name = naming::cpp::class::proxy_cpp(&iface_name);
        let rust_proxy_name = naming::rust::structure::proxy_rust(&iface_name);

        let cpp_create_proxy_func_name    = naming::cpp::function::create_proxy_cpp(&cpp_proxy_name);
        let cpp_create_proxy_at_func_name = naming::cpp::function::create_proxy_cpp_at(&cpp_proxy_name);
        let cpp_static_meta_func_name     = naming::cpp::function::static_meta_object(&cpp_proxy_name);
        let cpp_sizeof_func_name          = naming::cpp::function::sizeof_proxy_cpp(&cpp_proxy_name);
        let cpp_alignof_func_name         = naming::cpp::function::alignof_proxy_cpp(&cpp_proxy_name);
        let cpp_qmetatype_list_func_name  = naming::cpp::function::qmetatype_list(&cpp_proxy_name);

        let rust_create_proxy_func_name    = naming::rust::function::create_proxy_cpp(&cpp_proxy_name);
        let rust_create_proxy_at_func_name = naming::rust::function::create_proxy_cpp_at(&cpp_proxy_name);
        let rust_static_meta_func_name     = naming::rust::function::static_meta_object(&cpp_proxy_name);
        let rust_sizeof_func_name          = naming::rust::function::sizeof_proxy_cpp(&cpp_proxy_name);
        let rust_alignof_func_name         = naming::rust::function::alignof_proxy_cpp(&cpp_proxy_name);
        let rust_qmetatype_list_func_name  = naming::rust::function::qmetatype_list_of_proxy_cpp(&cpp_proxy_name);

        let result = vec![
            CppFunctionBridge::new(rust_create_proxy_func_name, parse_quote! {
                unsafe fn #cpp_create_proxy_func_name(rust_obj: *mut u8, rust_proxy: *mut #rust_proxy_name) -> *mut #cpp_proxy_name
            })?,
            CppFunctionBridge::new(rust_create_proxy_at_func_name, parse_quote! {
                unsafe fn #cpp_create_proxy_at_func_name(addr: *mut u8, rust_obj: *mut u8, rust_proxy: *mut #rust_proxy_name) -> *mut #cpp_proxy_name
            })?,
            CppFunctionBridge::new(rust_static_meta_func_name, parse_quote! {
                fn #cpp_static_meta_func_name() -> &'static QMetaObject
            })?,
            CppFunctionBridge::new(rust_sizeof_func_name, parse_quote! {
                fn #cpp_sizeof_func_name() -> usize
            })?,
            CppFunctionBridge::new(rust_alignof_func_name, parse_quote! {
                fn #cpp_alignof_func_name() -> usize
            })?,
            CppFunctionBridge::new(rust_qmetatype_list_func_name, parse_quote! {
                fn #cpp_qmetatype_list_func_name() -> QMetaType
            })?,
        ];
        Ok(result)
    }

    /// Generate functions for methods implemented in base interface
    /// (both virtual and not).
    fn generate_implemented_functions(&self) -> syn::Result<Vec<CppFunctionBridge>> {
        let result = self.iface.get_implemented_methods()
            .map(|method| {
                let sig = change_arg_idents_in_signature_to_camel_case(
                    method.get_signature())?;

                let rust_name = naming::rust::function::base(&sig.ident.to_string());
                let cpp_name = Self::get_caller_cpp_name(&sig, method.is_virtual());
                let self_arg = match is_self_mut(&sig) {
                    true =>  quote!{ self: Pin<&mut Self> },
                    false => quote!{ &self },
                };
                let typed_args = sig.inputs.iter().skip(1);
                let return_type = &sig.output;

                CppFunctionBridge::new(rust_name, syn::parse2(
                    quote! {
                        fn #cpp_name(#self_arg, #(#typed_args),*) #return_type
                    }
                )?)
            })
            .collect::<syn::Result<Vec<_>>>()?;

        Ok(result)
    }

    /// Get the name for C++ function calling base implementation of C++ interface.
    fn get_caller_cpp_name(src: &syn::Signature, is_virtual: bool) -> Naming {
        let rust_name = src.ident.to_string();
        let cpp_name = case_conv::snake_to_camel(&rust_name);
        if is_virtual {
            naming::cpp::function::base(&cpp_name)
        }
        else {
            cpp_name.into()
        }
    }
}
