// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::Ident;
use crate::qt_gen_impl;
use qt_gen_impl::qobject_macro_params::QObjectMacroParams;

pub fn qml_element(struct_ident: &Ident, params: &QObjectMacroParams) -> syn::Result<TokenStream> {

    if params.no_qml_element {
        return Ok(TokenStream::new())
    }

    let qml_register_fn_indent = format_ident!("qml_register_{struct_ident}");
    let struct_name = struct_ident.to_string();
    let is_singleton = params.singleton;

    let uri = std::env::var("CARGO_PKG_NAME")
        .map_err(|err| syn::Error::new(Span::call_site(), format!("Failed to get CARGO_PKG_NAME: {err}")))?
        .trim_start_matches(char::is_numeric)
        .chars()
        .map(|ch| if ch.is_alphanumeric() { ch } else { '_' })
        .collect::<String>();
    let minor_version: u8 = std::env::var("CARGO_PKG_VERSION_MINOR")
        .map_err(|err| syn::Error::new(Span::call_site(), format!("Failed to get CARGO_PKG_VERSION_MINOR: {err}")))?
        .parse()
        .expect("Failed to parse CARGO_PKG_VERSION_MINOR");
    let major_version: u8 = std::env::var("CARGO_PKG_VERSION_MAJOR")
        .map_err(|err| syn::Error::new(Span::call_site(), format!("Failed to get CARGO_PKG_VERSION_MAJOR: {err}")))?
        .parse()
        .expect("Failed to parse CARGO_PKG_VERSION_MAJOR");

    let qmlregister_code = if params.link_me {
        quote! {
            // TODO: make auto registration via 'linkme' dependency an optional cargo feature?
            #[linkme::distributed_slice(qtbridge::qt_type_lib::QML_REGISTER_CALLBACKS)]
            #[allow(non_camel_case_types)]
            fn #qml_register_fn_indent() {
                <#struct_ident as qtbridge::qtbridge_runtime::QmlRegister>::qml_register();
            }
        }
    } else {
        TokenStream::new()
    };

    let qml_register_impl_code = quote! {
        impl qtbridge::qtbridge_runtime::QmlRegister for #struct_ident {
            const URI: &str = #uri;
            const ELEMENT_NAME: &str = #struct_name;
            const MINOR_VERSION: u8 = #minor_version;
            const MAJOR_VERSION: u8 = #major_version;
            const IS_SINGLETON: bool = #is_singleton;
        }
    };
    let output = quote! {
        #qmlregister_code
        #qml_register_impl_code
    };
    Ok(output)
}
