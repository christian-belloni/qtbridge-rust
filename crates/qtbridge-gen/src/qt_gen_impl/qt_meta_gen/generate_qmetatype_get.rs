// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use quote::quote;

pub fn generate_qmeta_type_get(struct_ident: &syn::Ident, generics: &syn::Generics) -> syn::Result<syn::ItemImpl> {
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let has_generics = !generics.params.is_empty();

    let body = match has_generics {
        true => quote! { let iface = qtbridge::qtbridge_runtime::qmetatypeforqobject::interface_for_generic::<Self>(); },
        false => quote! {
            use std::sync::OnceLock;
            static META_TYPE_INTERFACE: OnceLock<qtbridge::qtbridge_type_lib::QMetaTypeInterface> = OnceLock::new();
            let iface = META_TYPE_INTERFACE.get_or_init(qtbridge::qtbridge_runtime::qmetatypeforqobject::init_interface_for::<Self>);
        },
    };

    let code = quote! {
        impl #impl_generics qtbridge::qtbridge_type_lib::QMetaTypeGet for #struct_ident #type_generics #where_clause {
            fn get_qmetatype() -> qtbridge::qtbridge_type_lib::QMetaType {
                #body
                qtbridge::qtbridge_type_lib::QMetaType::new_with_interface(iface as *const _)
            }
        }
    };

    syn::parse2(code)
}
