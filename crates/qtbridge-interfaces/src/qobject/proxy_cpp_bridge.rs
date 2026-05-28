// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge_runtime::qproxies::QCppProxy;
use qtbridge_runtime::DynamicMetaObjectData;
use qtbridge_type_lib::{QMetaObject, QMetaType};

use super::proxy_rust::QObjectProxyRust;
#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qmetaobject/cpp/qmetaobject.h");
        type QMetaObject = qtbridge_type_lib::QMetaObject;
        include!("qtbridge-type-lib/src/generated/core/qmetatype/cpp/qmetatype.h");
        type QMetaType = qtbridge_type_lib::QMetaType;
        include!("qtbridge-runtime/src/cpp/dynamicmetaobjectdata.h");
        type DynamicMetaObjectData = qtbridge_runtime::DynamicMetaObjectData;
        include!("qtbridge-interfaces/src/qobject/proxy_rust_bridge.rs.h");
        type QObjectProxyRust = super::QObjectProxyRust;
    }
    #[namespace = "rust::bridge"]
    unsafe extern "C++" {
        include!("qtbridge-interfaces/src/qobject/cpp/QObjectProxyCpp.h");
        type QObjectProxyCpp;
        # [rust_name = create_qobject_proxy_cpp]
        unsafe fn create_QObjectProxyCpp(rust_proxy: *mut QObjectProxyRust, metaobject: *const DynamicMetaObjectData) -> *mut QObjectProxyCpp;
        # [rust_name = create_qobject_proxy_cpp_at]
        unsafe fn create_QObjectProxyCpp_At(rust_proxy: *mut QObjectProxyRust, metaobject: *const DynamicMetaObjectData, addr: *mut u8) -> *mut QObjectProxyCpp;
        # [rust_name = static_qmeta_object_of_qobject_proxy_cpp]
        fn staticQMetaObjectOf_QObjectProxyCpp() -> &'static QMetaObject;
        # [rust_name = size_of_qobject_proxy_cpp]
        fn sizeOf_QObjectProxyCpp() -> usize;
        # [rust_name = align_of_qobject_proxy_cpp]
        fn alignOf_QObjectProxyCpp() -> usize;
        # [rust_name = qmetatype_list_of_qobject_proxy_cpp]
        fn qmetaTypeListOf_QObjectProxyCpp() -> QMetaType;
        # [rust_name = emit_signal_cpp]
        fn emitSignal(self: Pin<&mut Self>, signal_name: &str, argv: &[*const u8]);
    }
}
pub use ffi::QObjectProxyCpp;

impl QCppProxy for QObjectProxyCpp {
    type ProxyRustType = QObjectProxyRust;
    fn get_static_meta_object() -> &'static QMetaObject {
        ffi::static_qmeta_object_of_qobject_proxy_cpp()
    }
    fn get_size() -> usize {
        ffi::size_of_qobject_proxy_cpp()
    }
    fn get_align() -> usize {
        ffi::align_of_qobject_proxy_cpp()
    }
    fn get_qmetatype_list() -> QMetaType {
        ffi::qmetatype_list_of_qobject_proxy_cpp()
    }
    unsafe fn create(rust_proxy: *mut Self::ProxyRustType, metaobject: &'static DynamicMetaObjectData) -> *mut Self {
        unsafe { ffi::create_qobject_proxy_cpp(rust_proxy, metaobject) }
    }
    unsafe fn create_at(rust_proxy: *mut Self::ProxyRustType, metaobject: &'static DynamicMetaObjectData, addr: *mut u8) -> *mut Self {
        unsafe { ffi::create_qobject_proxy_cpp_at(rust_proxy, metaobject, addr) }
    }
    fn emit_signal(self: std::pin::Pin<&mut Self>, signal_name: &str, argv: &[*const u8]) {
        self.emit_signal_cpp(signal_name, argv)
    }
}
