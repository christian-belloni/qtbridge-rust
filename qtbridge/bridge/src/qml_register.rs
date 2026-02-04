// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::QObjectHolder;
use crate::QMetaInfo;
use qt_type_lib::QObject;
use qt_type_lib::QMetaTypeGet;
pub trait QmlRegister : QMetaTypeGet + QMetaInfo + QObjectHolder + Default
{
    const URI: &str;
    const ELEMENT_NAME: &str;
    const MINOR_VERSION: &str;
    const MAJOR_VERSION: &str;
    const IS_SINGLETON: bool;

    fn qml_register() {
        let meta_obj_data = <Self as QMetaInfo>::get_shared_dynamic_meta_object();
        let meta_obj = unsafe {
            meta_obj_data
                .get_dynamic_qmetaobject()
                .as_ref()
                .expect("Failed to get QMetaObject")
        };
        let uri = Self::URI.trim_start_matches(char::is_numeric)
            .chars()
            .map(|ch| if ch.is_alphanumeric() { ch } else { '_' })
            .collect::<String>();
        let version_major = Self::MAJOR_VERSION.parse()
            .expect("Failed to parse package major version");
        let version_minor = Self::MINOR_VERSION.parse()
            .expect("Failed to parse package major version");

        if Self::IS_SINGLETON {
            qt_type_lib::qml_register_singleton(
                <Self as QMetaTypeGet>::get_qmetatype(),
                monomorphize_singleton_ctor::<Self>(),
                uri.as_bytes(),
                version_major,
                version_minor,
                Self::ELEMENT_NAME.as_bytes(),
                meta_obj,
            )
        } else {
            qt_type_lib::qml_register_element(
                Self::get_qmetatype(),
                Self::get_qmetatype_list_of_cpp_proxy(),
                Self::get_size_of_cpp_proxy() as u32,
                monomorphize_element_ctor::<Self>(),
                uri.as_bytes(),
                version_major,
                version_minor,
                Self::ELEMENT_NAME.as_bytes(),
                meta_obj,
            );
        }
    }
}

fn element_ctor<T: QmlRegister>(addr: *mut u8, _userdata: *mut u8) {
    let instance = std::rc::Rc::new(std::cell::RefCell::new(T::default()));
    T::register_instance_in_map_with_cpp_proxy_at(addr, instance.clone());
    T::set_dynamic_meta(&instance);
}

fn singleton_ctor<T: QmlRegister>() -> *mut QObject {
    let instance = std::rc::Rc::new(std::cell::RefCell::new(T::default()));
    T::register_instance_in_map(instance.clone(), true);
    T::set_dynamic_meta(&instance);
    std::ptr::from_mut(T::get_qobject(&instance.borrow()))
}

fn monomorphize_element_ctor<T: QmlRegister>() -> usize {
    extern "C" fn default_ctor<T: QmlRegister>(addr: *mut u8, userdata: *mut u8) {
        element_ctor::<T>(addr, userdata)
    }
    default_ctor::<T> as usize
}

fn monomorphize_singleton_ctor<T: QmlRegister>() -> usize {
    extern "C" fn default_ctor<T: QmlRegister>() -> *mut QObject {
        singleton_ctor::<T>()
    }
    default_ctor::<T> as usize
}
