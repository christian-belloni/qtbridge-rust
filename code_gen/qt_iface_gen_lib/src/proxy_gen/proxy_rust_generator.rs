// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;

use qt_gen_common::naming;
use qt_gen_common::signature_utils::{change_arg_idents_in_signature_to_camel_case, get_arg_ident, is_self_mut};
use qt_gen_common::type_dependencies::qt_types_to_rust_import_paths;
use qt_gen_common::type_tokens::TypeTokens;

use crate::InterfaceDesc;

pub struct RustProxyGenerator<'a> {
    iface: &'a InterfaceDesc,
}

impl<'a> RustProxyGenerator<'a> {
    /// Create a new instance.
    pub fn new(iface: &'a InterfaceDesc) -> Self {
        Self {
            iface
        }
    }

    /// Run the code generation.
    /// Produce a TokenStream that will be written to the dedicated 'proxy_rust.rs' file.
    /// See 'qtbridge/qt_ifaces/src/generated/*/proxy_rust.rs' for an example of the generated code.
    /// See diagrams in 'qtbridge/qt_ifaces/docs/uml/' illustrating the structure at a higher level.
    pub fn generate(&self) -> syn::Result<TokenStream> {
        let iface = self.iface;
        let iface_name = iface.get_ident();

        let struct_name_rust = naming::rust::structure::proxy_rust(&iface_name);
        let struct_name_cpp = naming::rust::structure::proxy_cpp(&iface_name);

        let cpp_proxy_module = naming::rust::module::proxy_cpp_bridge();
        let iface_trait_module = naming::rust::module::interface_trait();
        let iface_trait = naming::rust::traits::iface_trait(iface_name);

        let non_interface_functions = self.generate_non_interface_functions()?;
        let virtual_functions = self.generate_virtual_functions()?;
        let implemented_functions = self.generate_implemented_functions()?;

        let signatures = non_interface_functions.iter()
                .map(|f| &f.sig)
            .chain(virtual_functions.iter()
                .map(|f| &f.sig))
            .chain(implemented_functions.iter()
               .map(|f| &f.sig))
            .collect::<Vec<_>>();

        let is_mut_function = signatures.iter()
            .any(|sign| is_self_mut(sign));
        let use_pin_maybe = is_mut_function
            .then(|| quote!{ use std::pin::Pin; });
        let qt_imports = Self::generate_imports(signatures.into_iter())?;

        let code = quote! {
            // TODO: make it possible to import with use super::#struct_ident_cpp;

            #use_pin_maybe
            use std::rc::Rc;
            use std::cell::RefCell;
            use crate::RustObjAccess;
            #qt_imports
            use super::#cpp_proxy_module::{#struct_name_cpp, ffi};
            use super::#iface_trait_module::#iface_trait;

            pub struct #struct_name_rust {
                cpp_proxy: *mut #struct_name_cpp,
                #[allow(dead_code)] // TODO: Remove this later
                rust_obj: RustObjAccess<dyn #iface_trait>,
                on_drop: fn(rust_obj: *const u8),
            }

            impl #struct_name_rust {
                #(#non_interface_functions)*
                #(#virtual_functions)*
                #(#implemented_functions)*
            }
        };
        Ok(code)
    }

    /// Generate block with imports that goes to the top of generated file.
    fn generate_imports<'b>(mut signatures: impl Iterator<Item = &'b syn::Signature>) -> syn::Result<TokenStream> {
        let mut tokens = TypeTokens::default();
        signatures.try_for_each(|sign| tokens.collect_from_signature(sign))?;
        qt_types_to_rust_import_paths(tokens.iter_qt())
    }

    /// Generate functions that forward call to devoted virtual method of interface trait.
    fn generate_virtual_functions(&self) -> syn::Result<Vec<syn::ItemFn>> {
        let iface_name = self.iface.get_ident();
        let mut result = Vec::new();

        for method in self.iface.get_virtual_methods() {
            let sig = change_arg_idents_in_signature_to_camel_case(
                method.get_signature())?;

            let ident = &sig.ident;
            let typed_args = sig.inputs.iter()
                .skip(1)
                .map(get_arg_ident)
                .collect::<syn::Result<Vec<_>>>()?;

            let is_self_mut = is_self_mut(&sig);
            let borrow_fn = match is_self_mut {
                true  => quote! { try_with_borrow_mut },
                false => quote! { try_with_borrow },
            };

            let maybe_mutably = if  is_self_mut { "mutably " } else { "" };
            let expect_msg = format!("Failed to borrow {maybe_mutably}object to call {iface_name}::{ident}()");

            let block = parse_quote! {{
                self.rust_obj.#borrow_fn(|vtable| vtable.#ident(#(#typed_args),*))
                    .expect(#expect_msg)
            }};

            let func = syn::ItemFn {
                attrs: Vec::new(),
                vis: syn::Visibility::Public(syn::token::Pub::default()),
                sig: sig.clone(),
                block,
            };
            result.push(func);
        }

        Ok(result)
    }

    /// Generate functions that forward call to the base functions in C++ implementation.
    fn generate_implemented_functions(&self) -> syn::Result<Vec<syn::ItemFn>> {
        let iface_name = self.iface.get_ident();
        let mut result = Vec::new();

        for method in self.iface.get_implemented_methods() {
            let sig = change_arg_idents_in_signature_to_camel_case(
                method.get_signature())?;
            let src_ident = &sig.ident;

            let rust_name = naming::rust::function::base(src_ident);
            let cpp_name = naming::rust::function::base(src_ident);

            let is_self_mut = sig.receiver()
                .is_some_and(|receiver| receiver.mutability.is_some());
            let receiver: syn::Receiver = match is_self_mut {
                true => parse_quote!{&mut self},
                false => parse_quote!{&self},
            };
            let args_sig = std::iter::once(syn::FnArg::from(receiver))
                .chain(sig.inputs.iter()
                    .skip(1)
                    .cloned())
                .collect::<Vec<_>>();
            let args_names = args_sig.iter()
                .skip(1)
                .map(get_arg_ident)
                .collect::<syn::Result<Vec<_>>>()?;
            let new_sig = syn::Signature {
                ident: rust_name.to_ident(),
                inputs: syn::punctuated::Punctuated::from_iter(args_sig),
                ..sig.clone()
            };

            let maybe_mutably = if  is_self_mut { "mutably " } else { "" };
            let expect_msg = format!("Failed to borrow {maybe_mutably}object to call {iface_name}::{cpp_name}()");

            let block_code = match is_self_mut {
                true => quote! {{
                    let proxy = unsafe { self.cpp_proxy.as_mut().unwrap() };
                    let proxy_pinned = unsafe {
                        Pin::new_unchecked(proxy)
                    };
                    self.rust_obj.try_with_assuming_borrowed_mut(|_| proxy_pinned.#cpp_name(#(#args_names),*))
                        .expect(#expect_msg)

                }},
                false => quote! {{
                    let proxy = unsafe { self.cpp_proxy.as_ref().unwrap() };
                    self.rust_obj.try_with_assuming_borrowed(|_| proxy.#cpp_name(#(#args_names),*))
                        .expect(#expect_msg)
                }},
            };

            let func = syn::ItemFn {
                attrs: Vec::new(),
                vis: syn::Visibility::Public(syn::token::Pub::default()),
                sig: new_sig.clone(),
                block: syn::parse2(block_code)?,
            };
            result.push(func);
        }

        Ok(result)
    }

    /// Generate other functions of Rust proxy:
    /// * for constructing from Rust/Qml.
    /// * returning information about corresponding C++ proxy.
    /// * returning C++ proxy.
    fn generate_non_interface_functions(&self) -> syn::Result<Vec<syn::ItemFn>> {
        let iface_name = self.iface.get_ident();

        let iface_trait = naming::rust::traits::iface_trait(iface_name);
        let cpp_proxy_name = naming::cpp::class::proxy_cpp(iface_name);
        let cpp_proxy_rust_name = naming::rust::structure::proxy_cpp_in_camel_case(iface_name);
        let cpp_create_proxy_func_name = naming::rust::function::create_proxy_cpp(&cpp_proxy_rust_name);
        let cpp_create_proxy_at_func_name = naming::rust::function::create_proxy_cpp_at(&cpp_proxy_rust_name);
        let drop_self_rust_name = naming::rust::function::drop_self();
        let get_cpp_proxy_name = naming::rust::function::get_cpp_proxy();
        let get_cpp_proxy_mut_name = naming::rust::function::get_cpp_proxy_mut();
        let get_static_meta_object_name = naming::rust::function::static_meta_object(&cpp_proxy_name);
        let size_of_proxy_name = naming::rust::function::sizeof_proxy_cpp(&cpp_proxy_name);
        let align_of_proxy_name = naming::rust::function::alignof_proxy_cpp(&cpp_proxy_name);
        let get_qmetatype_list_of_proxy_name = naming::rust::function::qmetatype_list_of_proxy_cpp(&cpp_proxy_name);

        let functions_code = [
            quote! {
                pub fn new(rust_obj: &Rc<RefCell<dyn #iface_trait>>, register_strong: bool, on_drop: fn(rust_obj: *const u8)) -> *mut Self {
                    let raw_rust_obj = rust_obj.as_ptr();
                    let boxed_self = Box::new(Self {
                        cpp_proxy: std::ptr::null_mut(),
                        rust_obj: match register_strong {
                            true => RustObjAccess::new_strong(rust_obj.clone()),
                            false => RustObjAccess::new_weak(Rc::downgrade(rust_obj))
                        },
                        on_drop,
                    });
                    let raw_self = Box::into_raw(boxed_self);
                    unsafe {(*raw_self).cpp_proxy = ffi::#cpp_create_proxy_func_name(raw_rust_obj.cast(), raw_self) };
                    raw_self
                }
            },
            quote! {
                pub fn new_with_cpp_proxy_at(addr: *mut u8, rust_obj: &Rc<RefCell<dyn #iface_trait>>, on_drop: fn(rust_obj: *const u8)) -> *mut Self {
                    let raw_rust_obj = rust_obj.as_ptr();
                    let boxed_self = Box::new(Self {
                        cpp_proxy: std::ptr::null_mut(),
                        rust_obj: RustObjAccess::new_strong(rust_obj.clone()),
                        on_drop,
                    });
                    let raw_self = Box::into_raw(boxed_self);
                    unsafe { (*raw_self).cpp_proxy = ffi::#cpp_create_proxy_at_func_name(addr, raw_rust_obj.cast(), raw_self) };
                    raw_self
                }
            },
            quote! {
                pub fn #drop_self_rust_name(raw_self: *mut Self, rust_obj_ptr: *const u8) {
                    let boxed_self = unsafe { Box::from_raw(raw_self) };
                    (boxed_self.on_drop)(rust_obj_ptr);
                }
            },
            quote! {
                pub fn get_static_meta_object() -> &'static QMetaObject {
                    ffi::#get_static_meta_object_name()
                }
            },
            quote! {
                pub fn get_size_of_cpp_proxy() -> usize {
                    ffi::#size_of_proxy_name()
                }
            },
            quote! {
                pub fn get_align_of_cpp_proxy() -> usize {
                    ffi::#align_of_proxy_name()
                }
            },
            quote! {
                pub fn get_qmetatype_list_of_cpp_proxy() -> QMetaType {
                    ffi::#get_qmetatype_list_of_proxy_name()
                }
            },
            quote! {
                pub fn #get_cpp_proxy_name(&self) -> *const #cpp_proxy_name {
                    self.cpp_proxy as *const _
                }
            },
            quote! {
                pub fn #get_cpp_proxy_mut_name(&self) -> *mut #cpp_proxy_name {
                    self.cpp_proxy
                }
            }
        ];

        functions_code.into_iter()
            .map(syn::parse2)
            .collect()
    }
}
