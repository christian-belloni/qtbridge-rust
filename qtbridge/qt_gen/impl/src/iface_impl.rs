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
        let bridge_library = self.origin.bridge_module();

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
                    <Self as #bridge_library::QObjectHolder>::get_rust_proxy(self)

                }
                fn get_rust_proxy_mut(&self) -> &mut #iface_library::#iface_module::#proxy_ident {
                    <Self as #bridge_library::QObjectHolder>::get_rust_proxy_mut(self)
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
        let bridge_library = self.origin.bridge_module();

        let code = quote! {

            impl #impl_generics #bridge_library::QObjectHolder for #struct_ident #type_generics #where_clause {

                type ProxyRust = #iface_library::#iface_module::#proxy_rust;

                fn register_instance_in_map(rust_obj_rc: std::rc::Rc<std::cell::RefCell<Self>>, construction: #bridge_library::qrustproxy::ConstructionMode)
                {
                    let key = (*rust_obj_rc).as_ptr() as *const u8;
                    let proxy = <Self as #iface_library::#iface_module::#iface_traits_name>::create_rust_proxy(rust_obj_rc, construction);
                    Self::try_borrow_mut_proxies_map(|proxies| {
                        proxies.insert(key, proxy as *const u8);
                    })
                }
            }
        };
        Ok(code)
    }
}
