// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::QObjectHolder;
use crate::QMetaInfo;
use crate::qrustproxy::ConstructionMode;
use qt_type_lib::QObject;
use qt_type_lib::QMetaTypeGet;
pub trait QmlRegister : QMetaTypeGet + QMetaInfo + QObjectHolder + Default
{
    const URI: &str;
    const ELEMENT_NAME: &str;
    const MINOR_VERSION: u8;
    const MAJOR_VERSION: u8;
    const IS_SINGLETON: bool;

    fn qml_register() {
        let meta_obj_data = <Self as QMetaInfo>::get_shared_dynamic_meta_object();
        let meta_obj = unsafe {
            meta_obj_data
                .get_dynamic_qmetaobject()
                .as_ref()
                .expect("Failed to get QMetaObject")
        };

        if Self::IS_SINGLETON {
            qt_type_lib::qml_register_singleton(
                <Self as QMetaTypeGet>::get_qmetatype(),
                monomorphize_singleton_ctor::<Self>(),
                Self::URI.as_bytes(),
                Self::MAJOR_VERSION,
                Self::MINOR_VERSION,
                Self::ELEMENT_NAME.as_bytes(),
                meta_obj,
            )
        } else {
            qt_type_lib::qml_register_element(
                Self::get_qmetatype(),
                Self::get_qmetatype_list_of_cpp_proxy(),
                Self::get_size_of_cpp_proxy() as u32,
                monomorphize_element_ctor::<Self>(),
                Self::URI.as_bytes(),
                Self::MAJOR_VERSION,
                Self::MINOR_VERSION,
                Self::ELEMENT_NAME.as_bytes(),
                meta_obj,
            );
        }
    }
}

fn element_ctor<T: QmlRegister>(addr: *mut u8, _userdata: *mut u8) {
    let instance = std::rc::Rc::new(std::cell::RefCell::new(T::default()));
    T::register_instance_in_map(instance.clone(), ConstructionMode::AtAddress(addr));
    T::set_dynamic_meta(&instance);
}

fn singleton_ctor<T: QmlRegister>() -> *mut QObject {
    let instance = std::rc::Rc::new(std::cell::RefCell::new(T::default()));
    T::register_instance_in_map(instance.clone(), ConstructionMode::Strong);
    T::set_dynamic_meta(&instance);
    std::ptr::from_mut(T::get_qobject(&instance.borrow()))
}

fn monomorphize_element_ctor<T: QmlRegister>() -> usize {
    extern "C" fn default_ctor<T: QmlRegister>(addr: *mut u8, userdata: *mut u8) {
        element_ctor::<T>(addr, userdata)
    }
    default_ctor::<T> as *const () as usize
}

fn monomorphize_singleton_ctor<T: QmlRegister>() -> usize {
    extern "C" fn default_ctor<T: QmlRegister>() -> *mut QObject {
        singleton_ctor::<T>()
    }
    default_ctor::<T> as *const () as usize
}
