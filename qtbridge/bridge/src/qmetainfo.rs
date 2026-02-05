// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::{DynamicMetaObjectBuilder, QObjectHolder};

pub trait QMetaInfo {
    fn class_name() -> &'static str;
    fn register_meta(meta_obj: std::pin::Pin<&mut DynamicMetaObjectBuilder>); // Called once per type (per specific class name)

    /// Return Dynamic QMetaObject containing information
    /// about signals/slots/properties for given Rust object.
    fn get_shared_dynamic_meta_object() -> &'static DynamicMetaObjectBuilder;
}

pub fn create_dynamic_meta_object_builder_for_type<T: QMetaInfo + QObjectHolder>() -> *const DynamicMetaObjectBuilder {
    let meta = crate::create_dynamic_meta_object_builder(
        T::class_name(),
        <T as QObjectHolder>::get_static_meta_object());
    let pinned_meta = unsafe {
        std::pin::Pin::new_unchecked(meta.as_mut().unwrap())
    };
    <T as QMetaInfo>::register_meta(pinned_meta);
    meta
}
