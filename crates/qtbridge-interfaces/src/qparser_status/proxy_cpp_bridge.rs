// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge_runtime::qproxies::QCppProxy;
use qtbridge_runtime::DynamicMetaObjectData;
use qtbridge_type_lib::{QMetaObject, QMetaType};

use super::proxy_rust::QParserStatusProxyRust;
#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qmetaobject/cpp/qmetaobject.h");
        type QMetaObject = qtbridge_type_lib::QMetaObject;
        include!("qtbridge-type-lib/src/generated/core/qmetatype/cpp/qmetatype.h");
        type QMetaType = qtbridge_type_lib::QMetaType;
        include!("qtbridge-runtime/src/cpp/dynamicmetaobjectdata.h");
        type DynamicMetaObjectData = qtbridge_runtime::DynamicMetaObjectData;
        include!("qtbridge-interfaces/src/qparser_status/proxy_rust_bridge.rs.h");
        type QParserStatusProxyRust = super::QParserStatusProxyRust;
    }
    #[namespace = "rust::bridge"]
    unsafe extern "C++" {
        include!("qtbridge-interfaces/src/qparser_status/cpp/QParserStatusProxyCpp.h");
        type QParserStatusProxyCpp;
        # [rust_name = create_qparser_status_proxy_cpp]
        unsafe fn create_QParserStatusProxyCpp(rust_proxy: *mut QParserStatusProxyRust, metaobject: *const DynamicMetaObjectData) -> *mut QParserStatusProxyCpp;
        # [rust_name = create_qparser_status_proxy_cpp_at]
        unsafe fn create_QParserStatusProxyCpp_At(rust_proxy: *mut QParserStatusProxyRust, metaobject: *const DynamicMetaObjectData, addr: *mut u8) -> *mut QParserStatusProxyCpp;
        # [rust_name = static_qmeta_object_of_qparser_status_proxy_cpp]
        fn staticQMetaObjectOf_QParserStatusProxyCpp() -> &'static QMetaObject;
        # [rust_name = size_of_qparser_status_proxy_cpp]
        fn sizeOf_QParserStatusProxyCpp() -> usize;
        # [rust_name = align_of_qparser_status_proxy_cpp]
        fn alignOf_QParserStatusProxyCpp() -> usize;
        # [rust_name = qmetatype_list_of_qparser_status_proxy_cpp]
        fn qmetaTypeListOf_QParserStatusProxyCpp() -> QMetaType;
        fn parserStatusCastOf_QParserStatusProxyCpp() -> i32;
        # [rust_name = emit_signal_cpp]
        fn emitSignal(self: Pin<&mut Self>, signal_name: &str, argv: &[*const u8]);
    }
}
pub use ffi::QParserStatusProxyCpp;

impl QCppProxy for QParserStatusProxyCpp {
    type ProxyRustType = QParserStatusProxyRust;
    fn get_static_meta_object() -> &'static QMetaObject {
        ffi::static_qmeta_object_of_qparser_status_proxy_cpp()
    }
    fn get_size() -> usize {
        ffi::size_of_qparser_status_proxy_cpp()
    }
    fn get_align() -> usize {
        ffi::align_of_qparser_status_proxy_cpp()
    }
    fn get_qmetatype_list() -> QMetaType {
        ffi::qmetatype_list_of_qparser_status_proxy_cpp()
    }
    fn parser_status_cast() -> i32 {
        ffi::parserStatusCastOf_QParserStatusProxyCpp()
    }
    unsafe fn create(rust_proxy: *mut Self::ProxyRustType, metaobject: &'static DynamicMetaObjectData) -> *mut Self {
        unsafe { ffi::create_qparser_status_proxy_cpp(rust_proxy, metaobject) }
    }
    unsafe fn create_at(rust_proxy: *mut Self::ProxyRustType, metaobject: &'static DynamicMetaObjectData, addr: *mut u8) -> *mut Self {
        unsafe { ffi::create_qparser_status_proxy_cpp_at(rust_proxy, metaobject, addr) }
    }
    fn emit_signal(self: std::pin::Pin<&mut Self>, signal_name: &str, argv: &[*const u8]) {
        self.emit_signal_cpp(signal_name, argv)
    }
}
