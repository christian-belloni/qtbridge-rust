// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::QJsonValue;
use std::mem::MaybeUninit;
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qjsonarray/cpp/qjsonarray.h");
        #[allow(dead_code)]
        type QJsonArray = super::QJsonArray;
        include!("qtbridge-type-lib/src/generated/core/qjsonvalue/cpp/qjsonvalue.h");
        type QJsonValue = crate::QJsonValue;
    }
    #[namespace = "rust::bridge::qjsonarray"]
    unsafe extern "C++" {
        # [rust_name = qjson_array_drop]
        fn QJsonArray_Drop(v: &mut QJsonArray);
        # [rust_name = qjson_array_default]
        fn QJsonArray_Default() -> QJsonArray;
        # [rust_name = qjson_array_clone]
        fn QJsonArray_Clone(v: &QJsonArray) -> QJsonArray;
        # [rust_name = inline_cpp_fn_size]
        fn inlineCppFn_size(_obj: &QJsonArray) -> isize;
        # [rust_name = inline_cpp_fn_at]
        fn inlineCppFn_at(_obj: &QJsonArray, i: isize) -> QJsonValue;
        # [rust_name = inline_cpp_fn_append]
        fn inlineCppFn_append(_obj: &mut QJsonArray, value: &QJsonValue);
        # [rust_name = inline_cpp_fn_trait_impl_partial_eq_for_qjson_array_eq]
        fn inlineCppFn_TraitImpl_PartialEq_for_QJsonArray_eq(lhs: &QJsonArray, rhs: &QJsonArray) -> bool;
    }
}
/// A JSON array, bridging Qt's `QJsonArray` to Rust.
///
/// See also: [QJsonArray documentation](https://doc.qt.io/qt-6/qjsonarray.html).
#[derive(Debug)]
#[repr(C)]
pub struct QJsonArray {
    _content: MaybeUninit<[usize; 1]>,
}
unsafe impl cxx::ExternType for QJsonArray {
    type Id = cxx::type_id!("QJsonArray");
    type Kind = cxx::kind::Trivial;
}
impl Drop for QJsonArray {
    fn drop(&mut self) {
        ffi::qjson_array_drop(self)
    }
}
impl Default for QJsonArray {
    fn default() -> Self {
        ffi::qjson_array_default()
    }
}
impl Clone for QJsonArray {
    fn clone(&self) -> Self {
        ffi::qjson_array_clone(self)
    }
}
impl PartialEq for QJsonArray {
    fn eq(&self, other: &Self) -> bool {
        ffi::inline_cpp_fn_trait_impl_partial_eq_for_qjson_array_eq(self, other)
    }
}
impl QJsonArray {
    #[allow(dead_code)]
    /// Returns the number of elements in the array.
    pub fn size(&self) -> isize {
        ffi::inline_cpp_fn_size(self)
    }
    #[allow(dead_code)]
    /// Returns the element at position `i`.
    pub fn at(&self, i: isize) -> QJsonValue {
        ffi::inline_cpp_fn_at(self, i)
    }
    #[allow(dead_code)]
    /// Appends `value` to the end of the array.
    pub fn append(&mut self, value: &QJsonValue) {
        ffi::inline_cpp_fn_append(self, value)
    }
}
