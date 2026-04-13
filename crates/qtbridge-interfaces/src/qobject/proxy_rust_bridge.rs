// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use super::proxy_rust::QObjectProxyRust;
#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {}
    extern "Rust" {
        type QObjectProxyRust;
        # [Self = QObjectProxyRust]
        # [cxx_name = dropSelf]
        unsafe fn drop_self(self_ptr: *mut QObjectProxyRust, rust_obj_ptr: *const u8);
    }
}
unsafe impl cxx::ExternType for QObjectProxyRust {
    type Id = cxx::type_id!(QObjectProxyRust);
    type Kind = cxx::kind::Trivial;
}
