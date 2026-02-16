// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#[cxx::bridge]
pub mod ffi {

    unsafe extern "C++" {
        include!("cpp/metamethodparams.h");
        type MetaMethodOutgoingParams = crate::metamethodparams::ffi::MetaMethodOutgoingParams;

        include!("qt_type_lib/src/generated/core/qmetaobject/cpp/qmetaobject.h");
        type QMetaObject = qt_type_lib::QMetaObject;

        include!("qt_type_lib/src/generated/core/qmetatype/cpp/qmetatype.h");
        type QMetaType = qt_type_lib::QMetaType;

        include!("qt_type_lib/src/generated/core/qobject/cpp/qobject.h");
        type QObject = qt_type_lib::QObject;

        include!("qt_type_lib/src/generated/core/qvariant/cpp/qvariant.h");
        type QVariant = qt_type_lib::QVariant;
    }

    unsafe extern "C++" {
        include!("cpp/dynamicmetaobjectbuilder.h");
        type DynamicMetaObjectBuilder;

        #[rust_name = "set_to_qobject"]
        fn setToQObject(&self, dst: &mut QObject);

        #[rust_name = "add_class_info"]
        fn addClassInfo(self: Pin<&mut Self>, name: &str, value: &str);

        #[rust_name = "register_property"]
        fn registerProperty(self: Pin<&mut Self>, name: &str, meta_type: &QMetaType, getter: unsafe fn(receiver: *mut u8)->QVariant, setter: unsafe fn(receiver: *mut u8, value: &QVariant), notify_signal: &str);

        #[rust_name = "register_property_read_only"]
        fn registerPropertyReadOnly(self: Pin<&mut Self>, name: &str, meta_type: &QMetaType, getter: unsafe fn(receiver: *mut u8)->QVariant, is_constant: bool, notify_signal: &str);

        #[rust_name = "register_signal"]
        fn registerSignal(self: Pin<&mut Self>, name: &str, arg_meta_types: &[QMetaType]);

        #[rust_name = "register_slot"]
        fn registerSlot(self: Pin<&mut Self>, name: &str, arg_meta_types: &[QMetaType], return_meta_type: &QMetaType, callback: unsafe fn(receiver: *mut u8, inputs: &[*const u8], output: &[*mut u8]));

        #[rust_name = "end_meta_registration"]
        fn endMetaRegistration(self: Pin<&mut Self>);

        #[rust_name = "emit_signal"]
        fn emitSignal(self: &Self, qobj: &mut QObject, name: &str, params: &MetaMethodOutgoingParams);

        #[rust_name = "get_dynamic_qmetaobject"]
        fn getDynamicQMetaObject(self: &Self) -> *const QMetaObject;

        #[rust_name = "create_dynamic_meta_object_builder"]
        fn createDynamicMetaObjectBuilder(rust_struct_name: &str, static_meta: &QMetaObject) -> *mut DynamicMetaObjectBuilder;
    }
}

//unsafe impl Sync for crate::DynamicMetaObjectBuilder {}

pub use ffi::{DynamicMetaObjectBuilder, create_dynamic_meta_object_builder};

