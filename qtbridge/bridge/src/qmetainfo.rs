// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qt_type_lib::{QMetaObject, QMetaType};
use crate::DynamicMetaObjectData_Rust;

pub trait QMetaInfo {
    fn class_name() -> &'static str;
    fn register_meta(meta_obj: std::pin::Pin<&mut DynamicMetaObjectData_Rust>); // Called once per type (per specific class name)

    /// Return QMetaObject from QObject proxy.
    fn get_static_meta_object() -> &'static QMetaObject;

    /// Return Dynamic QMetaObject containing information
    /// about signals/slots/properties for given Rust object.
    fn get_shared_dynamic_meta_object_data() -> &'static DynamicMetaObjectData_Rust;

    /// Return QMetaType of `QQmlListProperty<Self>`.
    /// Needed for Qml type registration.
    fn get_list_meta_type() -> QMetaType;
}

pub fn create_dynamic_meta_object_data_for_type<T: QMetaInfo>() -> *const DynamicMetaObjectData_Rust {
    let meta = crate::create_dynamic_meta_object_data(
        T::class_name(),
        T::get_static_meta_object());
    let pinned_meta = unsafe {
        std::pin::Pin::new_unchecked(meta.as_mut().unwrap())
    };
    T::register_meta(pinned_meta);
    meta
}
