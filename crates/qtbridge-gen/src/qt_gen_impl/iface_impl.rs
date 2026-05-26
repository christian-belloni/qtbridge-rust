// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::TokenStream;
use quote::quote;

use qtbridge_gen_common::naming;
use qtbridge_gen_common::type_qualified_mapping::crate_names;

// Info extracted by parsing impl block of some structure by procedural macro 'qobject_impl'
pub struct InterfaceImpl {
    struct_ident: syn::Ident,     // Name of struct that implements given interface
    iface_ident: syn::Ident,      // The name of the Qt-interface the struct is implementing
    impl_generics: syn::Generics, // All the generics added to the implementation and their clauses
}

impl InterfaceImpl {
    pub fn new(struct_ident: syn::Ident, iface_ident: syn::Ident, impl_generics: syn::Generics) -> syn::Result<Self> {

        Ok(Self {
            struct_ident,
            iface_ident,
            impl_generics,
        })
    }

    /// Generate implementation details that we place in a dedicated module
    /// not to clutter impl block of the given structure too much.
    pub fn generate_impl_details(&self) -> syn::Result<TokenStream> {
        let struct_ident = &self.struct_ident;

        let iface_name = &self.iface_ident;
        let iface_module = naming::rust::module::from_struct_name(iface_name);
        let proxy_rust = naming::rust::structure::proxy_rust(iface_name);

        let (impl_generics, type_generics, where_clause) = self.impl_generics.split_for_impl();

        let iface_library = crate_names::iface_module();
        let bridge_library = crate_names::bridge_module();

        let code = quote! {

            impl #impl_generics #bridge_library::QObjectHolder for #struct_ident #type_generics #where_clause {

                type ProxyRust = #iface_library::#iface_module::#proxy_rust;

                fn as_adaptor_trait(
                    rust_obj_rc: std::rc::Rc<std::cell::RefCell<Self>>
                ) -> std::rc::Rc<std::cell::RefCell<
                    <Self::ProxyRust as #bridge_library::qproxies::QRustProxy>::AdapterType>>
                {
                    rust_obj_rc
                }

            }
        };
        Ok(code)
    }
}
