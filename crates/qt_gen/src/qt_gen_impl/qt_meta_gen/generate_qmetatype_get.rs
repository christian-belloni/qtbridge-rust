// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::TokenStream;
use quote::{quote};

use qtbridge_gen_common::type_qualified_mapping::CallOrigin;

pub fn generate_qmeta_type_get(struct_ident: &syn::Ident, generics: &syn::Generics, origin: &CallOrigin) -> syn::Result<TokenStream> {
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let type_library = origin.type_module();
    let bridge_library = origin.bridge_module();

    let has_generics = !generics.params.is_empty();

    let body = match has_generics {
        true => quote! { let iface = #bridge_library::qmetatypeforqobject::interface_for_generic::<Self>(); },
        false => quote! {
            use std::sync::OnceLock;
            static META_TYPE_INTERFACE: OnceLock<#type_library::QMetaTypeInterface> = OnceLock::new();
            let iface = META_TYPE_INTERFACE.get_or_init(#bridge_library::qmetatypeforqobject::init_interface_for::<Self>);
        },
    };

    let code = quote! {
        impl #impl_generics #type_library::QMetaTypeGet for #struct_ident #type_generics #where_clause {
            fn get_qmetatype() -> #type_library::QMetaType {
                #body
                #type_library::QMetaType::new_with_interface(iface as *const _)
            }
        }
    };

    Ok(code)
}
