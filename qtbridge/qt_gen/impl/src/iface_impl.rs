// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::TokenStream;
use quote::quote;

use qt_gen_common::naming;
use qt_gen_common::type_qualified_mapping::CallOrigin;

// Info extracted by parsing impl block of some structure by procedural macro 'qobject_impl'
pub struct InterfaceImpl {
    struct_ident: syn::Ident,     // Name of struct that implements given interface
    iface_ident: syn::Ident,      // The name of the Qt-interface the struct is implementing
    impl_generics: syn::Generics, // All the generics added to the implementation and their clauses
    origin: CallOrigin,           // Used to do the correct imports within qtbridges or externally
}

impl InterfaceImpl {
    pub fn new(struct_ident: syn::Ident, iface_ident: syn::Ident, impl_generics: syn::Generics, origin: CallOrigin) -> syn::Result<Self> {

        Ok(Self {
            struct_ident,
            iface_ident,
            impl_generics,
            origin,
        })
    }

    pub fn generate_iface_proxy_get_trait_impl(&self) -> syn::Result<syn::ItemImpl> {
        let iface_name = &self.iface_ident;
        let iface_module = naming::rust::module::from_struct_name(iface_name);
        //TODO: It is not pretty that we need so many traits. We should reduce it.
        let iface_ident = quote::format_ident!("{}Adapter", iface_name);
        let proxy_iface_ident = quote::format_ident!("{}ProxyGet", iface_name);
        let proxy_ident = quote::format_ident!("{}ProxyRust", iface_name);
        let struct_name = &self.struct_ident;
        let (impl_generics, type_generics, where_clause) = self.impl_generics.split_for_impl();
        let iface_library = self.origin.iface_module();
        let trait_library = self.origin.trait_module();

        let get_trait_body = if iface_ident == "QObject" {
            quote! { panic!("QObject does not implement a Rust trait interface")}
        } else {
            quote! { self }
        };

        let trait_methods = quote! {
            fn get_trait(&self) -> &dyn #iface_library::#iface_module::#iface_ident {
                #get_trait_body
            }

            fn get_trait_mut(&mut self) -> &mut dyn #iface_library::#iface_module::#iface_ident {
                #get_trait_body
            }
        };

        let trait_impl: syn::ItemImpl = syn::parse2(quote! {
            impl #impl_generics #iface_library::#iface_module::#proxy_iface_ident for #struct_name #type_generics #where_clause {
                fn get_rust_proxy(&self) -> &#iface_library::#iface_module::#proxy_ident {
                    <Self as #trait_library::QObjectHolder>::get_rust_proxy(self)

                }
                fn get_rust_proxy_mut(&self) -> &mut #iface_library::#iface_module::#proxy_ident {
                    <Self as #trait_library::QObjectHolder>::get_rust_proxy_mut(self)
                }
                #trait_methods
            }
        })?;
        Ok(trait_impl)
    }

    pub fn generate_iface_base_trait_impl(&self) -> syn::Result<syn::ItemImpl> {
        let iface_name = &self.iface_ident;
        let iface_module = naming::rust::module::from_struct_name(iface_name);
        let proxy_iface_ident = quote::format_ident!("{}Base", iface_name);
        let struct_name = &self.struct_ident;
        let (impl_generics, type_generics, where_clause) = self.impl_generics.split_for_impl();
        let iface_library = self.origin.iface_module();

        let trait_impl: syn::ItemImpl = syn::parse2(quote! {
            impl #impl_generics #iface_library::#iface_module::#proxy_iface_ident for #struct_name #type_generics #where_clause { }
        })?;
        Ok(trait_impl)
    }

    /// Generate implementation details that we place in a dedicated module
    /// not to clutter impl block of the given structure too much.
    pub fn generate_impl_details(&self) -> syn::Result<TokenStream> {
        let struct_ident = &self.struct_ident;

        let iface_name = &self.iface_ident;
        let iface_module = naming::rust::module::from_struct_name(iface_name);
        let proxy_rust = naming::rust::structure::proxy_rust(iface_name);
        let iface_traits_name = naming::rust::traits::iface_trait(iface_name);

        let (impl_generics, type_generics, where_clause) = self.impl_generics.split_for_impl();

        let iface_library = self.origin.iface_module();
        let type_library = self.origin.type_module();
        let trait_library = self.origin.trait_module();

        let code = quote! {

            /// Shared map containing all registered instances of given user-defined type (multiton).
            /// This cannot live in a trait
            impl #impl_generics #struct_ident #type_generics #where_clause {
                /// Invoke the provided function if mutable borrowing succeeds.
                fn try_borrow_mut_proxies_map_impl<F, R>(f: F) -> R
                where F: FnOnce(&mut std::collections::HashMap<*const u8, *const #iface_library::#iface_module::#proxy_rust>) -> R
                {
                    use std::cell::BorrowMutError;
                    use std::cell::RefCell;
                    use std::collections::HashMap;
                    thread_local!(static INSTANCES: RefCell<
                        HashMap<*const u8, *const #iface_library::#iface_module::#proxy_rust>
                    > = RefCell::new(HashMap::new()));
                    INSTANCES.try_with(|proxies_map_cell| -> Result<R, BorrowMutError> {
                        let mut proxies_map_ref_mut = proxies_map_cell.try_borrow_mut()?;
                        Ok(f(&mut proxies_map_ref_mut))
                    })
                    .unwrap()
                    .expect("Failed to borrow_mut map of proxies")
                }
            }

            impl #impl_generics #trait_library::QObjectHolder for #struct_ident #type_generics #where_clause {

                type ProxyRust = #iface_library::#iface_module::#proxy_rust;

                fn try_borrow_mut_proxies_map<F, R>(f: F) -> R
                    where F: FnOnce(&mut std::collections::HashMap<*const u8, *const Self::ProxyRust>) -> R
                {
                    Self::try_borrow_mut_proxies_map_impl(f)
                }

                fn try_get_qobject(&self) -> Option<&mut #type_library::QObject>
                {
                    let rust_proxy = Self::try_get_rust_proxy_mut(&self)?;
                    let cpp_proxy = rust_proxy.get_cpp_proxy();
                    let qobject_ptr: *const #type_library::QObject = cpp_proxy.cast();
                    unsafe { qobject_ptr.cast_mut().as_mut() }
                }

                fn register_instance_in_map(rust_obj_rc: std::rc::Rc<std::cell::RefCell<Self>>, register_strong: bool)
                {
                    use std::rc::Rc;
                    use std::cell::RefCell;
                    let key = (*rust_obj_rc).as_ptr() as *const u8;
                    Self::try_borrow_mut_proxies_map(|proxies| {
                        let dyn_rc: Rc<RefCell<dyn #iface_library::#iface_module::#iface_traits_name>> = rust_obj_rc;
                        let proxy_ptr = Self::ProxyRust::new(&dyn_rc, register_strong, Self::unregister_instance_in_map);
                        proxies.insert(key, proxy_ptr);
                    })
                }

                fn register_instance_in_map_with_cpp_proxy_at(addr: *mut u8, rust_obj_rc: std::rc::Rc<std::cell::RefCell<Self>>)
                {
                    use std::rc::Rc;
                    use std::cell::RefCell;
                    let key = (*rust_obj_rc).as_ptr() as *const u8;
                    Self::try_borrow_mut_proxies_map(|proxies| {
                        let dyn_rc: Rc<RefCell<dyn #iface_library::#iface_module::#iface_traits_name>> = rust_obj_rc;
                        let proxy_ptr = Self::ProxyRust::new_with_cpp_proxy_at(addr, &dyn_rc, Self::unregister_instance_in_map);
                        proxies.insert(key, proxy_ptr);
                    })
                }
            }
        };
        Ok(code)
    }

    /// Generate an `impl` block with functions for attaching, detaching,
    /// and accessing the corresponding `QObject`.
    pub fn generate_qobject_funcs(&self) -> syn::Result<syn::ItemImpl> {
        let struct_name = &self.struct_ident;
        let (impl_generics, type_generics, where_clause) = self.impl_generics.split_for_impl();
        let type_library = self.origin.type_module();
        let trait_library = self.origin.trait_module();

        let functions_code = [
            quote! {
                /// Create a new default-initialized object of this type with a `QObject` already attached.
                /// The object must remain at its original heap location and must not be moved out of `Rc<RefCell<T>>`.
                /// TODO: rename it so that 'qobject' is not exposed to the user.
                /// TODO: or attach a qobject on demand/when sending the object to QML engine?.
                pub fn default_with_attached_qobject() -> std::rc::Rc<std::cell::RefCell<Self>> {
                    let instance = std::rc::Rc::new(std::cell::RefCell::new(Self::default()));
                    Self::attach_qobject(&instance);
                    instance
                }
            },
            quote! {
                /// Attach a dedicated `QObject` to the Rust object given as an argument.
                /// Rust object must remain at its original heap location and must not be moved out of `Rc<RefCell<T>>`.
                /// TODO: rename it so that 'qobject' is not exposed to the user.
                /// TODO: or attach a qobject on demand/when sending the object to QML engine?.
                pub fn attach_qobject(instance: &std::rc::Rc<std::cell::RefCell<Self>>) {
                    <Self as #trait_library::QObjectHolder>::register_instance_in_map(instance.clone(), false);
                    <Self as #trait_library::QObjectHolder>::set_dynamic_meta(instance);
                }
            },
            quote! {
                /// Detach and remove the dedicated `QObject` from the specified object.
                /// This function is intended to be called during the `Drop` of this type.
                /// TODO: Rename it so that 'qobject' is not exposed to the user.
                /// TODO: Document somewhere (in the documentation of #[qobject_impl]?) that this function must be called from the `Drop` implementation of the user-defined type.
                pub fn detach_qobject(&self) {
                    if let Some(qobj) = <Self as #trait_library::QObjectHolder>::try_get_qobject(self) {
                        // QObject destructor will drop Rust proxy and remove entry from the instance map
                        #type_library::QObject::delete(std::ptr::from_mut(qobj));
                    }
                    // None if corresponding item was already deleted from the instance map
                }
            },
            quote! {
                /// Return a reference to the QObject attached to 'self'.
                // TODO: probably remove it later
                pub fn get_qobject(&self) -> &mut #type_library::QObject {
                    <Self as #trait_library::QObjectHolder>::get_qobject(self)
                }
            },
            quote! {
                /// Return a QVariant containing a pointer to the QObject proxy
                /// corresponding to 'self'.
                pub fn as_qvariant(&self) -> #type_library::QVariant {
                    // TODO: handle changes in reference counter via QMetaTypeInterface
                    let qobj_ref = <Self as #trait_library::QObjectHolder>::get_qobject(self);
                    let qobj_ptr = std::ptr::from_mut(qobj_ref);
                    qobj_ptr.into()
                }
            }
        ];
        let functions = functions_code.into_iter()
            .map(syn::parse2)
            .collect::<syn::Result<Vec<syn::ImplItemFn>>>()?;

        let mut item_impl: syn::ItemImpl = syn::parse2(quote! {
            /// Impl block with functions needed to attach, detach and reference QObject corresponding to given Rust object
            impl #impl_generics #struct_name #type_generics #where_clause {
            }
        })?;
        item_impl.items = functions.into_iter()
            .map(syn::ImplItem::from)
            .collect();

        Ok(item_impl)
    }
}
