// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#![cfg(test)]
use std::iter;
use syn::parse_quote;
use qt_gen_common::signature_utils::{get_qualified_args, get_qualified_return_type};
use qt_gen_common::type_qualified_mapping::CallOrigin;

#[test]
fn require_that_get_qualified_args_adds_qtbridge_to_path_when_it_is_qt_type() {
    let src: syn::FnArg = parse_quote!{
        parent: &QVariant
    };
    let expected: syn::FnArg = parse_quote! {
        parent: &qtbridge::qt_type_lib::QVariant
    };

    let actual = get_qualified_args(iter::once(&src), CallOrigin::External).unwrap();
    assert_eq!(actual, [expected])
}

#[test]
fn require_that_get_qualified_args_keeps_argument_unchanged_when_it_is_standard_type() {
    let src: syn::FnArg = parse_quote!{
        row: i32
    };

    let actual = get_qualified_args(iter::once(&src), CallOrigin::External).unwrap();
    assert_eq!(actual, [src]);
}


#[test]
fn require_that_get_qualified_args_returns_type_path_that_agrees_with_reference() {
    let cases = [
        ("arg: i32", "arg: i32"),
        ("arg: QByteArray", "arg: qtbridge::qt_type_lib::QByteArray"),
        ("arg: Vec<f64>", "arg: Vec<f64>"),
        ("arg: Vec<QString>", "arg: Vec<qtbridge::qt_type_lib::QString>"),
        ("arg: &Vec<qt_type_lib::QByteArray>", "arg: &Vec<qtbridge::qt_type_lib::QByteArray>"),
        ("arg: &Vec<Vec<Vec<&[QVariant]>>>", "arg: &Vec<Vec<Vec<&[qtbridge::qt_type_lib::QVariant]>>>"),
    ];

    for (input_str, expected_str) in cases {
        let input = syn::parse_str(input_str).unwrap();
        let expected = syn::parse_str(expected_str).unwrap();

        let actual = get_qualified_args(iter::once(&input), CallOrigin::External)
             .unwrap();
        assert_eq!(actual, [expected]);
    }
}

#[test]
fn require_that_get_qualified_return_type_adds_qtbridge_to_path_when_it_is_qt_type() {
    let src: syn::ReturnType = parse_quote!{
        -> QStringList
    };
    let expected: syn::ReturnType = parse_quote! {
        -> qtbridge::qt_type_lib::QStringList
    };

    let actual = get_qualified_return_type(&src, CallOrigin::External).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn require_that_get_qualified_return_type_keeps_argument_unchanged_when_it_is_standard_type() {
    let src: syn::ReturnType = parse_quote!{
        -> &str
    };
    let expected: syn::ReturnType = parse_quote! {
        -> &str
    };

    let actual = get_qualified_return_type(&src, CallOrigin::External).unwrap();
    assert_eq!(actual, expected);
}
