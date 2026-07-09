// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::QList;
use crate::{QHash, QHashImpl, QString, QVariant};
#[allow(non_camel_case_types)]
/// This is a monomorphized form of type [QHash] for types [QString], [QVariant].
pub type QHash_QString_QVariant = QHash<QString, QVariant>;
/// This is an alias for type [QHash] for types [QString], [QVariant].
pub type QVariantHash = QHash<QString, QVariant>;
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qhash/cpp/qhash_qstring_qvariant.h");
        #[allow(dead_code)]
        type QHash_QString_QVariant = super::QHash_QString_QVariant;
        include!("qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_qstring.h");
        type QList_QString = crate::QList_QString;
        include!("qtbridge-type-lib/src/generated/core/qlist/cpp/qlist_qvariant.h");
        type QList_QVariant = crate::QList_QVariant;
        include!("qtbridge-type-lib/src/generated/core/qstring/cpp/qstring.h");
        type QString = crate::QString;
        include!("qtbridge-type-lib/src/generated/core/qvariant/cpp/qvariant.h");
        type QVariant = crate::QVariant;
    }
    #[namespace = "rust::bridge::qhash_qstring_qvariant"]
    unsafe extern "C++" {
        # [rust_name = qhash_drop]
        fn QHash_Drop(v: &mut QHash_QString_QVariant);
        # [rust_name = qhash_default]
        fn QHash_Default() -> QHash_QString_QVariant;
        # [rust_name = qhash_clone]
        fn QHash_Clone(v: &QHash_QString_QVariant) -> QHash_QString_QVariant;
        # [rust_name = inline_cpp_fn_clear]
        fn inlineCppFn_clear(_obj: &mut QHash_QString_QVariant);
        # [rust_name = inline_cpp_fn_contains]
        fn inlineCppFn_contains(_obj: &QHash_QString_QVariant, key: &QString) -> bool;
        # [rust_name = inline_cpp_fn_insert]
        fn inlineCppFn_insert(_obj: &mut QHash_QString_QVariant, key: &QString, value: &QVariant);
        # [rust_name = inline_cpp_fn_is_empty]
        fn inlineCppFn_is_empty(_obj: &QHash_QString_QVariant) -> bool;
        # [rust_name = inline_cpp_fn_remove]
        fn inlineCppFn_remove(_obj: &mut QHash_QString_QVariant, key: &QString) -> bool;
        # [rust_name = inline_cpp_fn_size]
        fn inlineCppFn_size(_obj: &QHash_QString_QVariant) -> isize;
        # [rust_name = inline_cpp_fn_keys]
        fn inlineCppFn_keys(_obj: &QHash_QString_QVariant) -> QList_QString;
        # [rust_name = inline_cpp_fn_values]
        fn inlineCppFn_values(_obj: &QHash_QString_QVariant) -> QList_QVariant;
        # [rust_name = inline_cpp_fn_value]
        fn inlineCppFn_value(_obj: &QHash_QString_QVariant, key: &QString) -> QVariant;
        # [rust_name = inline_cpp_fn_trait_impl_std_ops_index_ref_qstring_for_qhash_qstring_qvariant_index]
        unsafe fn inlineCppFn_TraitImpl_std_ops_Index_ref_QString_for_QHash_QString_QVariant_index(_obj: &QHash_QString_QVariant, key: &QString) -> *const QVariant;
    }
}
unsafe impl cxx::ExternType for QHash_QString_QVariant {
    type Id = cxx::type_id!("QHash_QString_QVariant");
    type Kind = cxx::kind::Trivial;
}
impl Default for QHash_QString_QVariant {
    fn default() -> Self {
        ffi::qhash_default()
    }
}
impl Clone for QHash_QString_QVariant {
    fn clone(&self) -> Self {
        ffi::qhash_clone(self)
    }
}
impl From<&[(QString, QVariant)]> for QHash<QString, QVariant> {
    fn from(src: &[(QString, QVariant)]) -> Self {
        let mut result = Self::default();
        src.iter().for_each(|(k, v)| result.insert(k, v));
        result
    }
}
impl<const N: usize> From<[(QString, QVariant); N]> for QHash<QString, QVariant> {
    fn from(src: [(QString, QVariant); N]) -> Self {
        let mut result = Self::default();
        src.iter().for_each(|(k, v)| result.insert(k, v));
        result
    }
}
impl std::ops::Index<&QString> for QHash<QString, QVariant> {
    type Output = QVariant;
    fn index(&self, index: &QString) -> &Self::Output {
        let cpp = ffi::inline_cpp_fn_trait_impl_std_ops_index_ref_qstring_for_qhash_qstring_qvariant_index;
        unsafe { cpp(self, index).as_ref() }.expect("Given key does not exist in QHash")
    }
}
impl QHashImpl<QString, QVariant> for QHash_QString_QVariant {
    fn clear(&mut self) {
        ffi::inline_cpp_fn_clear(self)
    }
    fn contains(&self, key: &QString) -> bool {
        ffi::inline_cpp_fn_contains(self, key)
    }
    fn insert(&mut self, key: &QString, value: &QVariant) {
        let cpp = ffi::inline_cpp_fn_insert;
        cpp(self, key, value)
    }
    fn is_empty(&self) -> bool {
        ffi::inline_cpp_fn_is_empty(self)
    }
    fn remove(&mut self, key: &QString) -> bool {
        let cpp = ffi::inline_cpp_fn_remove;
        cpp(self, key)
    }
    fn size(&self) -> isize {
        let cpp = ffi::inline_cpp_fn_size;
        cpp(self)
    }
    fn keys(&self) -> QList<QString> {
        let cpp = ffi::inline_cpp_fn_keys;
        cpp(self)
    }
    fn values(&self) -> QList<QVariant> {
        let cpp = ffi::inline_cpp_fn_values;
        cpp(self)
    }
    fn value(&self, key: &QString) -> QVariant {
        let cpp = ffi::inline_cpp_fn_value;
        cpp(self, key)
    }
    fn do_drop(&mut self) {
        ffi::qhash_drop(self)
    }
}
