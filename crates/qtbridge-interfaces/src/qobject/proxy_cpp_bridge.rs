// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge_runtime::qproxies::QCppProxy;
use qtbridge_runtime::DynamicMetaObjectData;
use qtbridge_type_lib::QMetaObject;

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
        # [Self = QObjectProxyCpp]
        # [rust_name = create]
        unsafe fn create(rust_proxy: *mut QObjectProxyRust, metaobject: *const DynamicMetaObjectData) -> *mut QObjectProxyCpp;
        # [Self = QObjectProxyCpp]
        # [rust_name = create_at]
        unsafe fn createAt(rust_proxy: *mut QObjectProxyRust, metaobject: *const DynamicMetaObjectData, addr: *mut u8)
        -> *mut QObjectProxyCpp;
        # [Self = QObjectProxyCpp]
        # [rust_name = static_qmeta_object]
        fn baseStaticMetaObject() -> &'static QMetaObject;
        # [Self = QObjectProxyCpp]
        # [rust_name = size_of]
        fn sizeOfProxy() -> usize;
        # [Self = QObjectProxyCpp]
        # [rust_name = align_of]
        fn alignOfProxy() -> usize;
        # [Self = QObjectProxyCpp]
        # [rust_name = parser_status_cast]
        fn parserStatusCast() -> i32;
        # [rust_name = emit_signal_cpp]
        fn emitSignal(self: Pin<&mut Self>, signal_name: &str, argv: &[*const u8]);
    }
}
pub use ffi::QObjectProxyCpp;

impl QCppProxy for QObjectProxyCpp {
    type ProxyRustType = QObjectProxyRust;
    fn get_static_meta_object() -> &'static QMetaObject {
        Self::static_qmeta_object()
    }
    fn get_size() -> usize {
        Self::size_of()
    }
    fn get_align() -> usize {
        Self::align_of()
    }
    fn parser_status_cast() -> i32 {
        Self::parser_status_cast()
    }
    unsafe fn create(rust_proxy: *mut Self::ProxyRustType, metaobject: &'static DynamicMetaObjectData) -> *mut Self {
        unsafe { Self::create(rust_proxy, metaobject) }
    }
    unsafe fn create_at(rust_proxy: *mut Self::ProxyRustType, metaobject: &'static DynamicMetaObjectData, addr: *mut u8) -> *mut Self {
        unsafe { Self::create_at(rust_proxy, metaobject, addr) }
    }
    fn emit_signal(self: std::pin::Pin<&mut Self>, signal_name: &str, argv: &[*const u8]) {
        self.emit_signal_cpp(signal_name, argv)
    }
}
