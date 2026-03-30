// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#![cfg(test)]
use qtbridge_gen_common::type_to_cpp::type_to_cpp;
use syn::parse_str;

#[test]
fn require_that_type_to_cpp_returns_expected_string_when_called_on_supported_types_values() {
    qt_type_lib::init();
    let cases = [
        // Primitive types
        ("usize", "size_t"),
        ("isize", "ptrdiff_t"),
        ("i64",  "int64_t"),
        ("u64",  "uint64_t"),
        ("i32",  "int32_t"),
        ("u32",  "uint32_t"),
        ("i16",  "int16_t"),
        ("u16",  "uint16_t"),
        ("i8" ,  "int8_t"),
        ("u8" ,  "uint8_t"),
        ("bool", "bool"),
        ("f32",  "float"),
        ("f64",  "double"),

        // Strings
        ("String", "rust::String"),
        ("str",    "rust::Str"),

        // Qt types
        ("QMetaObject", "QMetaObject"),
        ("QModelIndex", "QModelIndex"),
        ("QString",     "QString"),
        ("QStringList", "QStringList"),
        ("QVariant",    "QVariant"),
        ("QObject",     "QObject"),

        // Qt types qualified with qt_type_lib
        ("qt_type_lib::QMetaObject", "QMetaObject"),
        ("qt_type_lib::QModelIndex", "QModelIndex"),
        ("qt_type_lib::QString",     "QString"),
        ("qt_type_lib::QStringList", "QStringList"),
        ("qt_type_lib::QVariant",    "QVariant"),
        ("qt_type_lib::QObject",     "QObject"),

        // Vectors
        ("Vec<i32>",     "rust::Vec<int32_t>"),
        ("Vec<String>",  "rust::Vec<rust::String>"),
        ("Vec<QString>", "rust::Vec<QString>"),

        // Vector qualified with std
        ("std::Vec<f32>",    "rust::Vec<float>"),
        ("std::Vec<f64>",    "rust::Vec<double>"),
        ("std::Vec<String>", "rust::Vec<rust::String>"),

        // CXX types
        ("UniquePtr<QStringList>",      "std::unique_ptr<QStringList>"),
        ("cxx::UniquePtr<QMetaObject>", "std::unique_ptr<QMetaObject>"),

        // Function pointers
        ("fn (&i64, *mut f32, bool) -> *const u8", "rust::Fn<uint8_t const*(const int64_t&, float*, bool)>")
    ];

    for (rust_str, expected_cpp) in cases {
        let rust_type: syn::Type = parse_str(&rust_str).unwrap();
        let actual = type_to_cpp(&rust_type);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap().as_str(), expected_cpp)
    }
}

#[test]
fn require_that_type_to_cpp_returns_expected_string_when_called_on_reference_to_supported_types() {
    let cases = [
        ("&i64",           "const int64_t&"),
        ("&f64",           "const double&"),
        ("&mut bool",      "bool&"),
        ("&std::Vec<f32>", "const rust::Vec<float>&"),
    ];

    for (rust_str, expected_cpp) in cases {
        let rust_type: syn::Type = parse_str(&rust_str).unwrap();
        let actual = type_to_cpp(&rust_type);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap().as_str(), expected_cpp)
    }
}

#[test]
fn require_that_type_to_cpp_returns_expected_string_when_called_on_pointer_to_supported_types() {
    let cases = [
        ("*const i8",            "int8_t const*"),
        ("*mut f32",             "float*"),
        ("*mut bool",            "bool*"),
        ("*const *const i8",     "int8_t const* const*"),
        ("*const std::Vec<i32>", "rust::Vec<int32_t> const*"),
    ];

    for (rust_str, expected_cpp) in cases {
        let rust_type: syn::Type = parse_str(&rust_str).unwrap();
        let actual = type_to_cpp(&rust_type);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap().as_str(), expected_cpp)
    }
}

#[test]
fn require_that_type_to_cpp_returns_err_when_called_on_unsupported_types() {
    let cases = [
        "foo",
        "bar",
        "BTreeMap<i32, u32>",
        "HashMap<i32, u32>",
    ];

    for rust_str in cases {
        let rust_type: syn::Type = parse_str(&rust_str).unwrap();
        let actual = type_to_cpp(&rust_type);
        assert!(actual.is_err());
    }
}
