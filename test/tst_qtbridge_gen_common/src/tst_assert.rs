// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#![cfg(test)]
use proc_macro2::TokenStream;
use pretty_assertions::assert_eq;
use quote::ToTokens;
use qtbridge_gen_common::format_code::format_rust_code;

pub(crate) fn assert_tokens_eq(actual: &impl ToTokens, expected: &TokenStream) {
    let actual_tokens = actual.to_token_stream();
    let actual_str = actual_tokens.to_string();
    let expected_str = expected.to_string();
    if actual_str != expected_str {
        let actual_str_fmt = format_rust_code(&actual_tokens).unwrap();
        let expected_str_fmt = format_rust_code(expected).unwrap();
        assert_eq!(actual_str_fmt, expected_str_fmt);
    }
}
