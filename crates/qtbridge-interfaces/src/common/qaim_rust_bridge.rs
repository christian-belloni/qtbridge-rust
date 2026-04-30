// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge_type_lib::{QByteArray, QHash, QModelIndex, QVariant};
use crate::{RustObjAccess, call_cpp_impl};
use super::QAIMProxyCpp;

pub struct QGenericAIMProxyRust<T: ?Sized + 'static> {
    pub(crate) cpp_proxy: *mut QAIMProxyCpp,
    pub(crate) rust_proxy: *mut QAIMProxyRust,
    pub(crate) rust_obj: RustObjAccess<T>
}

impl<T: ?Sized + 'static> QGenericAIMProxyRust<T> {
    pub fn create_index(&self, row: i32, column: i32, ptr: usize) -> QModelIndex {
        call_cpp_impl!(self, base_create_index(row, column, ptr))
    }
    pub fn base_role_names(&self) -> QHash<i32, QByteArray> {
        call_cpp_impl!(self, base_role_names())
    }
    pub fn base_set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
        call_cpp_impl!(mut self, base_set_data(index, value, role))
    }
    pub fn base_remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool {
        call_cpp_impl!(mut self, base_remove_rows(first, count, parent))
    }
    pub fn base_sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex {
        call_cpp_impl!(self, base_sibling(row, column, idx))
    }
    pub fn base_data_changed(&mut self, top_left: &QModelIndex, bottom_right: &QModelIndex) {
        call_cpp_impl!(mut self, base_data_changed(top_left, bottom_right))
    }
    pub fn base_begin_insert_rows(&mut self, parent: &QModelIndex, first: i32, last: i32) {
        call_cpp_impl!(mut self, base_begin_insert_rows(parent, first, last))
    }
    pub fn base_end_insert_rows(&mut self) {
        call_cpp_impl!(mut self, base_end_insert_rows())
    }
    pub fn base_begin_move_rows(&mut self, source_parent: &QModelIndex, source_first: i32, source_last: i32, destination_parent: &QModelIndex, destination_child: i32) {
        call_cpp_impl!(mut self, base_begin_move_rows(source_parent, source_first, source_last, destination_parent, destination_child))
    }
    pub fn base_end_move_rows(&mut self) {
        call_cpp_impl!(mut self, base_end_move_rows())
    }
    pub fn base_begin_remove_rows(&mut self, parent: &QModelIndex, first: i32, last: i32) {
        call_cpp_impl!(mut self, base_begin_remove_rows(parent, first, last))
    }
    pub fn base_end_remove_rows(&mut self) {
        call_cpp_impl!(mut self, base_end_remove_rows())
    }
    pub fn base_begin_reset_model(&mut self) {
        call_cpp_impl!(mut self, base_begin_reset_model())
    }
    pub fn base_end_reset_model(&mut self) {
        call_cpp_impl!(mut self, base_end_reset_model())
    }
}
pub trait QAIMProxyImpl {
    fn index(&self, row: i32, col: i32, parent: &QModelIndex) -> QModelIndex;
    fn parent(&self, child: &QModelIndex) -> QModelIndex;
    fn row_count(&self, parent: &QModelIndex) -> i32;
    fn column_count(&self, parent: &QModelIndex) -> i32;
    fn data(&self, index: &QModelIndex, role: i32) -> QVariant;
    fn role_names(&self) -> QHash<i32, QByteArray>;
    fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool;
    fn remove_columns(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool;
    fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool;
    fn sibling(&self, row: i32, col: i32, idx: &QModelIndex) -> QModelIndex;
    fn invoke_slot(&self, slot_id: u32, inputs: &[*const u8], outputs: &[*mut u8]);
    fn invoke_slot_mut(&mut self, slot_id: u32, inputs: &[*const u8], outputs: &[*mut u8]);
    fn read_property(&self, prop_id: u32) -> QVariant;
    fn write_property(&mut self, prop_id: u32, value: &QVariant);
}

macro_rules! impl_generic_aim_proxy {
    ($adapter:ty) => {
        impl QRustProxy for QGenericAIMProxyRust<$adapter> {

            type ProxyCppType = QAIMProxyCpp;
            type AdapterType = $adapter;

            fn new<OnDropFn: FnOnce() + 'static>(rust_obj: &Rc<RefCell<$adapter>>, construct: ConstructionMode, on_drop: OnDropFn) -> *mut Self {
                let boxed_self = Box::new(Self {
                    cpp_proxy: std::ptr::null_mut(),
                    rust_proxy: std::ptr::null_mut(),
                    rust_obj: match construct {
                        ConstructionMode::Strong | ConstructionMode::AtAddress(_) => RustObjAccess::new_strong(rust_obj.clone()),
                        ConstructionMode::Weak => RustObjAccess::new_weak(Rc::downgrade(rust_obj)),
                    }
                });
                let raw_self = Box::into_raw(boxed_self);

                let aim_proxy = QAIMProxyRust::new(raw_self, on_drop);
                unsafe { (*raw_self).rust_proxy = aim_proxy; }

                unsafe{ (*raw_self).cpp_proxy = match construct {
                    ConstructionMode::AtAddress(addr) => {
                        ffi::create_qaim_proxy_cpp_at(addr, (*raw_self).rust_proxy)
                    }
                    ConstructionMode::Strong | ConstructionMode::Weak => {
                        ffi::create_qaim_proxy_cpp((*raw_self).rust_proxy)
                    }
                }};
                raw_self
            }
            fn get_static_meta_object() -> &'static QMetaObject {
                ffi::static_qmeta_object_of_qaim_proxy_cpp()
            }
            fn get_size_of_cpp_proxy() -> usize {
                ffi::size_of_qaim_proxy_cpp()
            }
            fn get_align_of_cpp_proxy() -> usize {
                ffi::align_of_qaim_proxy_cpp()
            }
            fn get_qmetatype_list_of_cpp_proxy() -> QMetaType {
                ffi::qmetatype_list_of_qaim_proxy_cpp()
            }
            fn get_cpp_proxy(&self) -> *const QAIMProxyCpp {
                self.cpp_proxy as *const _
            }
            fn get_cpp_proxy_mut(&self) -> *mut QAIMProxyCpp {
                self.cpp_proxy
            }
        }

        impl QAIMProxyImpl for QGenericAIMProxyRust<$adapter> {
            fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex {
                call_rust_trait_impl!(self, index(row, column, parent))
            }
            fn parent(&self, child: &QModelIndex) -> QModelIndex {
                call_rust_trait_impl!(self, parent(child))
            }
            fn row_count(&self, parent: &QModelIndex) -> i32 {
                call_rust_trait_impl!(self, row_count(parent))
            }
            fn column_count(&self, parent: &QModelIndex) -> i32 {
                call_rust_trait_impl!(self, column_count(parent))
            }
            fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
                call_rust_trait_impl!(self, data(index, role))
            }
            fn role_names(&self) -> QHash<i32, QByteArray> {
                call_rust_trait_impl!(self, role_names())
            }
            fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
                call_rust_trait_impl!(mut self, set_data(index, value, role))
            }
            fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool {
                call_rust_trait_impl!(mut self, remove_rows(first, count, parent))
            }
            fn remove_columns(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool {
                call_rust_trait_impl!(mut self, remove_columns(first, count, parent))
            }
            fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex {
                call_rust_trait_impl!(self, sibling(row, column, idx))
            }
            fn invoke_slot(&self, slot_id: u32, inputs: &[*const u8], outputs: &[*mut u8]) {
                call_rust_trait_impl!(self, invoke_slot(slot_id, inputs, outputs))
            }
            fn invoke_slot_mut(&mut self, slot_id: u32, inputs: &[*const u8], outputs: &[*mut u8]) {
                call_rust_trait_impl!(mut self, invoke_slot_mut(slot_id, inputs, outputs))
            }
            fn read_property(&self, prop_id: u32) -> QVariant {
                call_rust_trait_impl!(self, read_property(prop_id))
            }
            fn write_property(&mut self, prop_id: u32, value: &QVariant) {
                call_rust_trait_impl!(mut self, write_property(prop_id, value))
            }
        }
    };
}
pub(crate) use impl_generic_aim_proxy;

pub struct QAIMProxyRust {
    rust_obj: *mut dyn QAIMProxyImpl,
    on_drop: Box<dyn FnOnce()>,
}

impl QAIMProxyRust {
    fn obj(&self) -> &dyn QAIMProxyImpl {
        unsafe { &*self.rust_obj }
    }
    fn obj_mut(&mut self) -> &mut dyn QAIMProxyImpl {
        unsafe { &mut *self.rust_obj }
    }

    pub fn new<T: QAIMProxyImpl + 'static, OnDropFn: FnOnce() + 'static>(ptr: *mut T, on_drop: OnDropFn) -> *mut Self {
        Box::into_raw(Box::new(Self {
            rust_obj: ptr as *mut dyn QAIMProxyImpl,
            on_drop: Box::new(on_drop),
        }))
    }
    pub fn drop_self(self_ptr: *mut Self) {
        unsafe { drop(Box::from_raw((*self_ptr).rust_obj)) };
        let boxed_self = unsafe { Box::from_raw(self_ptr) };
        (boxed_self.on_drop)();
    }
    pub fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex {
        self.obj().index(row, column, parent)
    }
    pub fn parent(&self, child: &QModelIndex) -> QModelIndex {
        self.obj().parent(child)
    }
    pub fn row_count(&self, parent: &QModelIndex) -> i32 {
        self.obj().row_count(parent)
    }
    pub fn column_count(&self, parent: &QModelIndex) -> i32 {
        self.obj().column_count(parent)
    }
    pub fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        self.obj().data(index, role)
    }
    pub fn role_names(&self) -> QHash<i32, QByteArray> {
        self.obj().role_names()
    }
    pub fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
        self.obj_mut().set_data(index, value, role)
    }
    pub fn remove_columns(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool {
        self.obj_mut().remove_columns(first, count, parent)
    }
    pub fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool {
        self.obj_mut().remove_rows(first, count, parent)
    }
    pub fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex {
        self.obj().sibling(row, column, idx)
    }
    pub fn invoke_slot(&self, slot_id: u32, inputs: &[*const u8], outputs: &[*mut u8]) {
        self.obj().invoke_slot(slot_id, inputs, outputs)
    }
    pub fn invoke_slot_mut(&mut self, slot_id: u32, inputs: &[*const u8], outputs: &[*mut u8]) {
        self.obj_mut().invoke_slot_mut(slot_id, inputs, outputs)
    }
    pub fn read_property(&self, prop_id: u32) -> QVariant {
        self.obj().read_property(prop_id)
    }
    pub fn write_property(&mut self, prop_id: u32, value: &QVariant) {
        self.obj_mut().write_property(prop_id, value)
    }
}

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qhash/cpp/qhash_i32_qbytearray.h");
        type QHash_i32_QByteArray = qtbridge_type_lib::QHash_i32_QByteArray;
        include!("qtbridge-type-lib/src/generated/core/qmodelindex/cpp/qmodelindex.h");
        type QModelIndex = qtbridge_type_lib::QModelIndex;
        include!("qtbridge-type-lib/src/generated/core/qvariant/cpp/qvariant.h");
        type QVariant = qtbridge_type_lib::QVariant;
    }
    extern "Rust" {
        type QAIMProxyRust;
        # [Self = QAIMProxyRust]
        # [cxx_name = dropSelf]
        unsafe fn drop_self(self_ptr: *mut QAIMProxyRust);
        # [cxx_name = index]
        fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex;
        # [cxx_name = parent]
        fn parent(&self, child: &QModelIndex) -> QModelIndex;
        # [cxx_name = rowCount]
        fn row_count(&self, parent: &QModelIndex) -> i32;
        # [cxx_name = columnCount]
        fn column_count(&self, parent: &QModelIndex) -> i32;
        # [cxx_name = data]
        fn data(&self, index: &QModelIndex, role: i32) -> QVariant;
        # [cxx_name = roleNames]
        fn role_names(&self) -> QHash_i32_QByteArray;
        # [cxx_name = setData]
        fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool;
        # [cxx_name = removeRows]
        fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool;
        # [cxx_name = sibling]
        fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex;

        # [cxx_name = invokeSlot]
        fn invoke_slot(&self, slot_id: u32, args: &[*const u8], outputs: &[*mut u8]);
        # [cxx_name = invokeSlotMut]
        fn invoke_slot_mut(&mut self, slot_id: u32, args: &[*const u8], outputs: &[*mut u8]);
        # [cxx_name = readProperty]
        fn read_property(&self, prop_id: u32) -> QVariant;
        # [cxx_name = writeProperty]
        fn write_property(&mut self, prop_id: u32, value: &QVariant);

    }
}
unsafe impl cxx::ExternType for QAIMProxyRust {
    type Id = cxx::type_id!(QAIMProxyRust);
    type Kind = cxx::kind::Trivial;
}
