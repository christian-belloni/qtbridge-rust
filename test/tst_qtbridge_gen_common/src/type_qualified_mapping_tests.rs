// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]

use insta::assert_snapshot;
use quote::ToTokens;
use syn::{parse_quote, visit_mut::VisitMut};

use qtbridge_gen_common::format_code::format_rust_code;
use qtbridge_gen_common::type_qualified_mapping::TypeQualifiedMapping;

#[test]
fn can_be_called_on_nested_path_that_includes_qself() {
    let mut path = parse_quote! {
        std::rc::Rc<std::cell::RefCell<
            <Self::ProxyRust as qtbridge_runtime::qproxies::QRustProxy>::AdapterType
        >>
    };

    let mut type_map = TypeQualifiedMapping::default();
    type_map.visit_path_mut(&mut path);
    let actual = path.to_token_stream().to_string();
    assert!(type_map.result().is_ok());
    assert_snapshot!(actual);
}

#[test]
fn can_be_called_on_macro() {
    let mut expr_macro = parse_quote! {
        thread_local! {
            static DYNAMIC_META_OBJECT : OnceLock<&'static qtbridge_runtime::DynamicMetaObjectData>
                = OnceLock::new();
        }
    };

    let mut type_map = TypeQualifiedMapping::default();
    type_map.visit_expr_macro_mut(&mut expr_macro);
    let actual = expr_macro.to_token_stream().to_string();
    assert!(type_map.result().is_ok());
    assert_snapshot!(actual);
}

#[test]
fn can_be_called_on_macro_with_qtbridge_qualified_path() {
    let mut expr_macro = parse_quote! {
        thread_local! {
            static DYNAMIC_META_OBJECT : OnceLock<&'static qtbridge::qtbridge_runtime::DynamicMetaObjectData>
                = OnceLock::new();
        }
    };

    let mut type_map = TypeQualifiedMapping::default();
    type_map.visit_expr_macro_mut(&mut expr_macro);
    assert!(type_map.result().is_ok());
    let actual = expr_macro.to_token_stream().to_string();
    assert_snapshot!(actual);
}

#[test]
fn can_be_called_on_panic_macro() {
    let mut expr_macro = parse_quote! {
        panic!(
            "Failed to convert value '{}' to type '{}' in qproperty '{}'",
            value.to_string(),
            std::any::type_name_of_val(&self.first_value),
            "this_value"
        )
    };

    let mut type_map = TypeQualifiedMapping::default();
    type_map.visit_expr_macro_mut(&mut expr_macro);
    assert!(type_map.result().is_ok());
    let actual = expr_macro.to_token_stream().to_string();
    assert_snapshot!(actual);
}

#[test]
fn can_be_called_on_trait_impl() {
    let mut trait_impl = parse_quote! {
        impl qtbridge_runtime::QObjectHolder for Backend {
            type ProxyRust = qtbridge_interfaces::qobject::QObjectProxyRust;
            fn as_adaptor_trait(rust_obj_rc: std::rc::Rc<std::cell::RefCell<Self>>) ->
                std::rc::Rc<std::cell::RefCell<
                    <Self::ProxyRust as qtbridge_runtime::qproxies::QRustProxy>::AdapterType>
                >
            {
                rust_obj_rc
            }
        }
    };

    let mut type_map = TypeQualifiedMapping::default();
    type_map.visit_item_impl_mut(&mut trait_impl);
    assert!(type_map.result().is_ok());
    let actual = format_rust_code(&trait_impl.to_token_stream())
        .unwrap();
    assert_snapshot!(actual);
}

