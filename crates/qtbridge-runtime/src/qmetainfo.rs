// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::{DynamicMetaObjectBuilder, DynamicMetaObjectData};
use qtbridge_type_lib::QMetaObject;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;

pub trait QMetaInfo : 'static {
    fn class_name() -> &'static str {
        ::std::any::type_name::<Self>()
    }

    fn register_meta(meta_obj: std::pin::Pin<&mut DynamicMetaObjectBuilder>); // Called once per type (per specific class name)

    fn get_static_meta_object() -> &'static QMetaObject;

    /// Return DynamicMetaObjectData containing information
    /// about signals/slots/properties for given Rust object.
    fn get_shared_dynamic_meta_object_data() -> &'static DynamicMetaObjectData;

    /// Creates a new `DynamicMetaObjectData` object.
    ///
    /// Registers signals/slots/properties for the given Rust type.
    ///
    /// Returns a raw pointer to the heap-allocated object.
    /// Ownership is not managed internally; the caller is responsible for it.
    fn create_dynamic_meta_object_data_for_type() -> *const DynamicMetaObjectData {
        let mut builder = crate::create_dynamic_meta_object_builder(
            Self::class_name(),
            Self::get_static_meta_object());
        Self::register_meta(builder.pin_mut());
        builder.pin_mut()
            .take_dynamic_metaobject_data()
    }

}

pub fn dynamic_meta_object_data_for_generic<T: QMetaInfo + ?Sized>() -> &'static DynamicMetaObjectData {
    thread_local!(static DYNAMIC_META_MAP: RefCell<HashMap<TypeId, *const DynamicMetaObjectData>> =
        RefCell::new(HashMap::new()));

    let type_id = TypeId::of::<T>();
    {
        let meta_data_ptr = DYNAMIC_META_MAP.with_borrow(|dynamic_meta_builder_map| {
            dynamic_meta_builder_map.get(&type_id)
                .copied()
                .unwrap_or_default()
        });
        if let Some(meta_data_ref) = unsafe { meta_data_ptr.as_ref() } {
            return meta_data_ref;
        }
    }

    let meta_data_ptr = T::create_dynamic_meta_object_data_for_type();
    let meta_data_ref = unsafe { meta_data_ptr.as_ref() }.unwrap();
    DYNAMIC_META_MAP.with_borrow_mut(|dynamic_meta_builder_map| {
        dynamic_meta_builder_map.insert(type_id, meta_data_ptr);
    });

    meta_data_ref
}
