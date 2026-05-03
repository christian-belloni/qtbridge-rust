// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use super::proxy_cpp_bridge::{QObjectProxyCpp, ffi};
use crate::RustObjAccess;
use crate::call_rust_trait_impl;
use qtbridge_runtime::qrustproxy::{QRustProxy, ConstructionMode};
use qtbridge_runtime::{DispatchMetaCall, QObjectHolder};
use qtbridge_type_lib::{QMetaObject, QMetaType, QVariant};
use std::cell::RefCell;
use std::rc::Rc;

/* QObject trait left out on purpose */

pub trait QObjectAdapter: DispatchMetaCall {
}

impl<T> QObjectAdapter for T
where T: QObjectHolder<ProxyRust = QObjectProxyRust> {}

pub struct QObjectProxyRust {
    cpp_proxy: *mut QObjectProxyCpp,
    #[allow(dead_code)]
    rust_obj: RustObjAccess<dyn QObjectAdapter>,
    on_drop: Box<dyn FnOnce()>,
}

impl QRustProxy for QObjectProxyRust {

    type ProxyCppType = QObjectProxyCpp;
    type AdapterType = dyn QObjectAdapter;

    fn new<OnDropFn: FnOnce() + 'static>(rust_obj: &Rc<RefCell<dyn QObjectAdapter>>, construct: ConstructionMode, on_drop: OnDropFn) -> *mut Self {
        let boxed_self = Box::new(Self {
            cpp_proxy: std::ptr::null_mut(),
            rust_obj: match construct {
                ConstructionMode::Strong | ConstructionMode::AtAddress(_) => RustObjAccess::new_strong(rust_obj.clone()),
                ConstructionMode::Weak => RustObjAccess::new_weak(Rc::downgrade(rust_obj)),
            },
            on_drop: Box::new(on_drop),
        });
        let raw_self = Box::into_raw(boxed_self);

        unsafe{ (*raw_self).cpp_proxy = match construct {
            ConstructionMode::AtAddress(addr) => {
                ffi::create_qobject_proxy_cpp_at(addr, raw_self)
            }
            ConstructionMode::Strong | ConstructionMode::Weak => {
                ffi::create_qobject_proxy_cpp(raw_self)
            }
        }};
        raw_self
    }
    fn get_static_meta_object() -> &'static QMetaObject {
        ffi::static_qmeta_object_of_qobject_proxy_cpp()
    }
    fn get_size_of_cpp_proxy() -> usize {
        ffi::size_of_qobject_proxy_cpp()
    }
    fn get_align_of_cpp_proxy() -> usize {
        ffi::align_of_qobject_proxy_cpp()
    }
    fn get_qmetatype_list_of_cpp_proxy() -> QMetaType {
        ffi::qmetatype_list_of_qobject_proxy_cpp()
    }
    fn get_cpp_proxy(&self) -> *const QObjectProxyCpp {
        self.cpp_proxy as *const _
    }
    fn get_cpp_proxy_mut(&self) -> *mut QObjectProxyCpp {
        self.cpp_proxy
    }
}

impl QObjectProxyRust {
    pub fn drop_self(self_ptr: *mut Self) {
        let boxed_self = unsafe { Box::from_raw(self_ptr) };
        (boxed_self.on_drop)();
    }

    pub fn invoke_slot(&self, slot_id: u32, inputs: &[*const u8], outputs: &[*mut u8]) {
        call_rust_trait_impl!(self, invoke_slot(slot_id, inputs, outputs))
    }
    pub fn invoke_slot_mut(&mut self, slot_id: u32, inputs: &[*const u8], outputs: &[*mut u8]) {
        call_rust_trait_impl!(mut self, invoke_slot_mut(slot_id, inputs, outputs))
    }
    pub fn read_property(&self, prop_id: u32) -> QVariant {
        call_rust_trait_impl!(self, read_property(prop_id))
    }
    pub fn write_property(&mut self, prop_id: u32, value: &QVariant) {
        call_rust_trait_impl!(mut self, write_property(prop_id, value))
    }
    pub fn get_rust_object_rc_ptr(&self) -> *const u8 {
        self.rust_obj.get_rc()
            .map_or(std::ptr::null(), |rc| Rc::into_raw(rc) as *const u8)
    }
}
