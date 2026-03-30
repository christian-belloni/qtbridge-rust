// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use super::proxy_rust::QObjectProxyRust;
#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qmetaobject/cpp/qmetaobject.h");
        type QMetaObject = qtbridge_type_lib::QMetaObject;
        include!("qtbridge-type-lib/src/generated/core/qmetatype/cpp/qmetatype.h");
        type QMetaType = qtbridge_type_lib::QMetaType;
        include!("qt_ifaces/src/qobject/proxy_rust_bridge.rs.h");
        type QObjectProxyRust = super::QObjectProxyRust;
    }
    #[namespace = "rust::bridge"]
    unsafe extern "C++" {
        include!("qt_ifaces/src/qobject/cpp/QObjectProxyCpp.h");
        type QObjectProxyCpp;
        # [rust_name = create_qobject_proxy_cpp]
        unsafe fn create_QObjectProxyCpp(rust_obj: *mut u8, rust_proxy: *mut QObjectProxyRust) -> *mut QObjectProxyCpp;
        # [rust_name = create_qobject_proxy_cpp_at]
        unsafe fn create_QObjectProxyCpp_At(addr: *mut u8, rust_obj: *mut u8, rust_proxy: *mut QObjectProxyRust) -> *mut QObjectProxyCpp;
        # [rust_name = static_qmeta_object_of_qobject_proxy_cpp]
        fn staticQMetaObjectOf_QObjectProxyCpp() -> &'static QMetaObject;
        # [rust_name = size_of_qobject_proxy_cpp]
        fn sizeOf_QObjectProxyCpp() -> usize;
        # [rust_name = align_of_qobject_proxy_cpp]
        fn alignOf_QObjectProxyCpp() -> usize;
        # [rust_name = qmetatype_list_of_qobject_proxy_cpp]
        fn qmetaTypeListOf_QObjectProxyCpp() -> QMetaType;
    }
}
pub use ffi::QObjectProxyCpp;
