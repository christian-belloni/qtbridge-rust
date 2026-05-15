// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::{RustObjAccess, call_rust_trait_impl, call_cpp_impl};
use qtbridge_runtime::qproxies::{QRustProxy, QCppProxy, ConstructionMode};
use qtbridge_runtime::{DispatchMetaCall, DynamicMetaObjectData};
use qtbridge_type_lib::QVariant;
use std::cell::RefCell;
use std::rc::Rc;

pub struct GenericRustProxy<CppProxy: QCppProxy, Adapter: ?Sized> {
    pub(crate) cpp_proxy: *mut CppProxy,
    pub(crate) rust_obj: RustObjAccess<Adapter>,
    pub(crate) on_drop: Box<dyn FnOnce()>,
}

impl<CppProxy, Adapter> QRustProxy for GenericRustProxy<CppProxy, Adapter>
where
    CppProxy: QCppProxy<ProxyRustType = Self>,
    Adapter: ?Sized + DispatchMetaCall,
{
    type ProxyCppType = CppProxy;
    type AdapterType = Adapter;

    fn new(
        rust_obj: &Rc<RefCell<Adapter>>,
        metatype: &'static DynamicMetaObjectData,
        construct: ConstructionMode,
        on_drop: Box<dyn FnOnce() + 'static>,
    ) -> *mut Self {
        let boxed_self = Box::new(Self {
            cpp_proxy: std::ptr::null_mut(),
            rust_obj: match construct {
                ConstructionMode::Strong | ConstructionMode::AtAddress(_) =>
                    RustObjAccess::new_strong(rust_obj.clone()),
                ConstructionMode::Weak =>
                    RustObjAccess::new_weak(Rc::downgrade(rust_obj)),
            },
            on_drop,
        });
        let raw_self = Box::into_raw(boxed_self);
        unsafe {
            (*raw_self).cpp_proxy = match construct {
                ConstructionMode::AtAddress(addr) =>
                    CppProxy::create_at(raw_self, metatype, addr),
                ConstructionMode::Strong | ConstructionMode::Weak =>
                    CppProxy::create(raw_self, metatype),
            }
        };
        raw_self
    }

    fn get_cpp_proxy(&self) -> *const CppProxy {
        self.cpp_proxy
    }

    fn get_cpp_proxy_mut(&self) -> *mut CppProxy {
        self.cpp_proxy
    }

    fn emit_signal(&self, reference: &Adapter, signal_name: &str, argv: &[*const u8]) {
        call_cpp_impl!(self, reference, emit_signal(signal_name, argv))
    }

    fn emit_signal_mut(&self, mut_ref: &mut Adapter, signal_name: &str, argv: &[*const u8]) {
        call_cpp_impl!(mut self, mut_ref, emit_signal_mut(signal_name, argv))
    }
}

impl<CppProxy, Adapter> GenericRustProxy<CppProxy, Adapter>
where
    CppProxy: QCppProxy<ProxyRustType = Self>,
    Adapter: ?Sized + DispatchMetaCall,
{
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
