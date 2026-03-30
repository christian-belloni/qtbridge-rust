// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::{DynamicMetaObjectBuilder};
use qt_type_lib::QMetaObject;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;

pub trait QMetaInfo : 'static {
    fn class_name() -> &'static str {
        ::std::any::type_name::<Self>()
    }

    fn register_meta(meta_obj: std::pin::Pin<&mut DynamicMetaObjectBuilder>); // Called once per type (per specific class name)

    fn get_static_meta_object() -> &'static QMetaObject;

    /// Return Dynamic QMetaObject containing information
    /// about signals/slots/properties for given Rust object.
    fn get_shared_dynamic_meta_object() -> &'static DynamicMetaObjectBuilder;

    fn create_dynamic_meta_object_builder_for_type() -> *const DynamicMetaObjectBuilder {
        let meta = crate::create_dynamic_meta_object_builder(
            Self::class_name(),
            Self::get_static_meta_object());
        let pinned_meta = unsafe {
            std::pin::Pin::new_unchecked(meta.as_mut().unwrap())
        };
        Self::register_meta(pinned_meta);
        meta
    }

}

pub fn dynamic_meta_type_for_generic<T: QMetaInfo + ?Sized>() -> &'static DynamicMetaObjectBuilder {
    thread_local!(static DYNAMIC_META_MAP: RefCell<HashMap<TypeId, *const DynamicMetaObjectBuilder>> =
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

    let meta_data_ptr = T::create_dynamic_meta_object_builder_for_type();
    let meta_data_ref = unsafe { meta_data_ptr.as_ref() }.unwrap();
    DYNAMIC_META_MAP.with_borrow_mut(|dynamic_meta_builder_map| {
        dynamic_meta_builder_map.insert(type_id, meta_data_ptr);
    });

    meta_data_ref
}
