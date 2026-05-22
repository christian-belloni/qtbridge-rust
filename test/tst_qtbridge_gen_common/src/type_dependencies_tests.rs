// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#![cfg(test)]

use qtbridge_gen_common::cpp_include::CppInclude;
use qtbridge_gen_common::type_dependencies::{qt_types_to_bridge_imports, type_tokens_to_cpp_includes, qt_types_to_rust_import_paths};
use qtbridge_gen_common::type_tokens::TypeTokens;
use quote::quote;

use crate::tst_assert::assert_tokens_eq;

#[test]
fn require_that_type_tokens_to_cpp_includes_returns_output_that_agrees_with_reference() {
    qtbridge_type_lib::init();

    let type_tokens = [
        "i32",
        "f32",
        "String",
        "QString",
        "QVariant",
        "QModelIndex",
        "QMetaObject",
        "QStringList",
    ]
    .iter()
    .try_fold(TypeTokens::default(), |mut tokens, type_str| -> syn::Result<TypeTokens> {
        let path = syn::parse_str(type_str)?;
        tokens.collect_from_path(&path)?;
        Ok(tokens)
    })
    .unwrap();

    let expected = [
        "<cstdint>",
        r#""qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_qstring.h""#,
        r#""qtbridge-type-lib/src/generated/core/qmetaobject/cpp/qmetaobject.h""#,
        r#""qtbridge-type-lib/src/generated/core/qmodelindex/cpp/qmodelindex.h""#,
        r#""qtbridge-type-lib/src/generated/core/qstring/cpp/qstring.h""#,
        r#""qtbridge-type-lib/src/generated/core/qvariant/cpp/qvariant.h""#,
         r#""rust/cxx.h""#,
    ]
    .iter()
    .map(|str| CppInclude::new_from_str(str))
    .collect::<syn::Result<_>>()
    .unwrap();
    let actual = type_tokens_to_cpp_includes(&type_tokens)
        .unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn require_that_type_tokens_to_bridge_imports_returns_output_that_agrees_with_reference() {
    qtbridge_type_lib::init();

    let type_tokens = [
        "i32",
        "f32",
        "String",
        "QMetaObject",
        "QModelIndex",
        "QString",
        "QStringList",
        "QVariant",
    ]
    .iter()
    .try_fold(TypeTokens::default(), |mut tokens, type_str| -> syn::Result<TypeTokens> {
        let path = syn::parse_str(type_str)?;
        tokens.collect_from_path(&path)?;
        Ok(tokens)
    })
    .unwrap();

    let expected = quote! {
        include!("qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_qstring.h");
        type QList_QString = qtbridge_type_lib::QList_QString;

        include!("qtbridge-type-lib/src/generated/core/qmetaobject/cpp/qmetaobject.h");
        type QMetaObject = qtbridge_type_lib::QMetaObject;

        include!("qtbridge-type-lib/src/generated/core/qmodelindex/cpp/qmodelindex.h");
        type QModelIndex = qtbridge_type_lib::QModelIndex;

        include!("qtbridge-type-lib/src/generated/core/qstring/cpp/qstring.h");
        type QString = qtbridge_type_lib::QString;

        include!("qtbridge-type-lib/src/generated/core/qvariant/cpp/qvariant.h");
        type QVariant = qtbridge_type_lib::QVariant;
    };

    let imports = qt_types_to_bridge_imports(type_tokens.iter_qt(), false).unwrap();
    let actual = quote! {
        #(#imports)*
    };

    assert_tokens_eq(&actual, &expected);
}

#[test]
fn require_that_type_tokens_to_rust_import_paths_returns_output_that_agrees_with_reference() {
    qtbridge_type_lib::init();

    let type_tokens = [
        "i32",
        "f32",
        "String",
        "QMetaObject",
        "QModelIndex",
        "QString",
        "QStringList",
        "QVariant",
    ]
    .iter()
    .try_fold(TypeTokens::default(), |mut tokens, type_str| -> syn::Result<TypeTokens> {
        let path = syn::parse_str(type_str)?;
        tokens.collect_from_path(&path)?;
        Ok(tokens)
    })
    .unwrap();

    let expected = quote! {
        use qtbridge_type_lib::{QMetaObject, QModelIndex, QString, QStringList, QVariant};
    };

    let actual = qt_types_to_rust_import_paths(type_tokens.iter_qt())
        .unwrap();
    assert_tokens_eq(&actual, &expected);
}
