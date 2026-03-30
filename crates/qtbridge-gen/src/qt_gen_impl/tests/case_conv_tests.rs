// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#![cfg(test)]
use qtbridge_gen_common::case_conv::{camel_to_snake, is_pascal_case, snake_to_camel};

#[test]
pub fn require_that_camel_to_snake_returns_output_that_agrees_with_reference() {
    let cases = [
        ("", ""),
        ("one", "one"),
        ("oneTwoThree", "one_two_three"),
        ("FirstCapital", "first_capital"),
        ("already_in_snake", "already_in_snake"),
        ("capitalAfter_Underscore", "capital_after_underscore"),
        ("two__underscores_in_snake", "two__underscores_in_snake"),
        ("RGB", "rgb"),
        ("getResultAsQVariant", "get_result_as_qvariant"),

    ];

    for (src, expected) in cases {
        let actual = camel_to_snake(src);
        assert_eq!(actual, expected)
    }
}

#[test]
pub fn require_that_snake_to_camel_returns_output_that_agrees_with_reference() {
    let cases = [
        ("", ""),
        ("one", "one"),
        ("one_two_three", "oneTwoThree"),
        ("alreadyInCamel", "alreadyInCamel"),
    ];

    for (src, expected) in cases {
        let actual = snake_to_camel(src);
        assert_eq!(actual, expected)
    }
}

#[test]
pub fn require_that_is_pascal_case_returns_output_that_agrees_with_reference() {
    let cases = [
        ("", false),
        ("one", false),
        ("OneTwoThree", true),
        ("inCamelCase", false),
    ];

    for (src, expected) in cases {
        let actual = is_pascal_case(src);
        assert_eq!(actual, expected)
    }
}

