// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::mem::MaybeUninit;
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qutf8stringview/cpp/qutf8stringview.h");
        #[allow(dead_code)]
        type QUtf8StringView = super::QUtf8StringView;
        include!("qtbridge-type-lib/src/generated/core/qmetatype/cpp/qmetatype.h");
        type QMetaType = crate::QMetaType;
    }
    #[namespace = "rust::bridge::qutf8stringview"]
    unsafe extern "C++" {
        # [rust_name = qutf8_string_view_default]
        fn QUtf8StringView_Default() -> QUtf8StringView;
        # [rust_name = inline_cpp_fn_as_bytes]
        fn inlineCppFn_as_bytes(_obj: &QUtf8StringView, ptr: &mut *const u8, size: &mut isize);
    }
}
#[doc(hidden)]
#[repr(C)]
pub struct QUtf8StringView {
    _data: MaybeUninit<usize>,
    _size: MaybeUninit<usize>,
}
unsafe impl cxx::ExternType for QUtf8StringView {
    type Id = cxx::type_id!("QUtf8StringView");
    type Kind = cxx::kind::Trivial;
}
impl Default for QUtf8StringView {
    fn default() -> Self {
        ffi::qutf8_string_view_default()
    }
}
impl QUtf8StringView {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
    }
    #[allow(dead_code)]
    pub fn as_bytes(&self) -> &[u8] {
        let mut ptr = std::ptr::null();
        let mut size = 0isize;
        ffi::inline_cpp_fn_as_bytes(self, &mut ptr, &mut size);
        unsafe { std::slice::from_raw_parts(ptr, size as usize) }
    }
}
