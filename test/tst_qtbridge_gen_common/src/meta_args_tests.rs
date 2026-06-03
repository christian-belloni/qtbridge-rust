// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]
use qtbridge_gen_common::type_registry::meta_types::{get_qmetatype_support_for_type, MetaTypeMapping};

fn check(src_type_str: &str, expected: MetaTypeMapping) {
    let src_type: syn::Type = syn::parse_str(src_type_str).unwrap();
    let result = get_qmetatype_support_for_type(&src_type).unwrap();
    assert_eq!(result, expected, "for `{src_type_str}`");
}

// small ctor to keep the Converted rows readable
fn converted(s: &str) -> MetaTypeMapping {
    MetaTypeMapping::Converted(syn::parse_str(s).unwrap())
}

#[test]
pub fn tst_qmetatype_support_for_primitives()
{
    let inputs = [
        ("i8",    MetaTypeMapping::Direct),
        ("u8",    MetaTypeMapping::Direct),
        ("i16",   MetaTypeMapping::Direct),
        ("u16",   MetaTypeMapping::Direct),
        ("i32",   MetaTypeMapping::Direct),
        ("u32",   MetaTypeMapping::Direct),
        ("i64",   MetaTypeMapping::Direct),
        ("u64",   MetaTypeMapping::Direct),
        ("isize", MetaTypeMapping::Direct),
        ("usize", MetaTypeMapping::Direct),
        ("f32",   MetaTypeMapping::Direct),
        ("f64",   MetaTypeMapping::Direct),
        ("bool",  MetaTypeMapping::Direct),
    ];

    for (src_type_str, exp_type_type) in inputs {
        check(src_type_str, exp_type_type)
    }
}

#[test]
pub fn tst_qmetatype_support_for_strings()
{
    qtbridge_type_lib::init();
    let inputs = [
        ("str",    converted("qtbridge_type_lib::QString")),
        ("String", converted("qtbridge_type_lib::QString")),
    ];

    for (src_type_str, exp_type_type) in inputs {
        check(src_type_str, exp_type_type)
    }
}

#[test]
pub fn tst_qmetatype_support_for_vectors() {
    qtbridge_type_lib::init();
    let inputs = [
        ("Vec<i8>",     converted("qtbridge_type_lib::QList_i8")),
        ("Vec<u8>",     converted("qtbridge_type_lib::QList_u8")),
        ("Vec<i16>",    converted("qtbridge_type_lib::QList_i16")),
        ("Vec<u16>",    converted("qtbridge_type_lib::QList_u16")),
        ("Vec<i32>",    converted("qtbridge_type_lib::QList_i32")),
        ("Vec<u32>",    converted("qtbridge_type_lib::QList_u32")),
        ("Vec<i64>",    converted("qtbridge_type_lib::QList_i64")),
        ("Vec<u64>",    converted("qtbridge_type_lib::QList_u64")),
        ("Vec<f32>",    converted("qtbridge_type_lib::QList_f32")),
        ("Vec<f64>",    converted("qtbridge_type_lib::QList_f64")),
        ("Vec<String>", converted("qtbridge_type_lib::QList_QString")),
    ];

    for (src_type_str, exp_type_type) in inputs {
        check(src_type_str, exp_type_type)
    }
}
