// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use super::proxy_cpp_bridge::{QAbstractListModelProxyCpp, ffi};
use crate::{RustObjAccess, call_rust_trait_impl, call_cpp_impl};
use qtbridge_runtime::qrustproxy::{QRustProxy, ConstructionMode};
use qtbridge_runtime::{DispatchMetaCall, QObjectHolder};
use qtbridge_type_lib::{QByteArray, QHash, QMetaObject, QMetaType, QModelIndex, QVariant};
use std::cell::RefCell;
use std::rc::Rc;

pub trait QAbstractListModel : QObjectHolder<ProxyRust = QAbstractListModelProxyRust> {
    fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex {
        let proxy = self.get_rust_proxy();
        proxy.base_index(row, column, parent)
    }
    fn row_count(&self, parent: &QModelIndex) -> i32;
    fn data(&self, index: &QModelIndex, role: i32) -> QVariant;
    fn role_names(&self) -> QHash<i32, QByteArray> {
        let proxy = self.get_rust_proxy();
        proxy.base_role_names()
    }
    fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_set_data(index, value, role)
    }
    fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_remove_rows(first, count, parent)
    }
    fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex {
        let proxy = self.get_rust_proxy();
        proxy.base_sibling(row, column, idx)
    }
}

pub trait QAbstractListModelAdapter: DispatchMetaCall {
    fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex;
    fn row_count(&self, parent: &QModelIndex) -> i32;
    fn data(&self, index: &QModelIndex, role: i32) -> QVariant;
    fn role_names(&self) -> QHash<i32, QByteArray>;
    fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool;
    fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool;
    fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex;
}

impl<T> QAbstractListModelAdapter for T
where
    T : QAbstractListModel +
        QObjectHolder<ProxyRust = QAbstractListModelProxyRust>
{
    fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex {
         <Self as QAbstractListModel>::index(self, row, column, parent)
    }
    fn row_count(&self, parent: &QModelIndex) -> i32 {
         <Self as QAbstractListModel>::row_count(self, parent)
    }
    fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
         <Self as QAbstractListModel>::data(self, index, role)
    }
    fn role_names(&self) -> QHash<i32, QByteArray> {
         <Self as QAbstractListModel>::role_names(self)
    }
    fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
         <Self as QAbstractListModel>::set_data(self, index, value, role)
    }
    fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool {
         <Self as QAbstractListModel>::remove_rows(self, first, count, parent)
    }
    fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex {
         <Self as QAbstractListModel>::sibling(self, row, column, idx)
    }
}

pub trait QAbstractListModelBase : QObjectHolder<ProxyRust = QAbstractListModelProxyRust> {
    fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex {
        let proxy = self.get_rust_proxy();
        proxy.base_index(row, column, parent)
    }
    fn role_names(&self) -> QHash<i32, QByteArray> {
        let proxy = self.get_rust_proxy();
        proxy.base_role_names()
    }
    fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_set_data(index, value, role)
    }
    fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_remove_rows(first, count, parent)
    }
    fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex {
        let proxy = self.get_rust_proxy();
        proxy.base_sibling(row, column, idx)
    }
    fn data_changed(&mut self, top_left: &QModelIndex, bottom_right: &QModelIndex) {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_data_changed(top_left, bottom_right);
    }
    fn begin_insert_rows(&mut self, parent: &QModelIndex, first: i32, last: i32) {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_begin_insert_rows(parent, first, last);
    }
    fn end_insert_rows(&mut self) {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_end_insert_rows();
    }
    fn begin_move_rows(&mut self, source_parent: &QModelIndex, source_first: i32, source_last: i32,
        destination_parent: &QModelIndex, destination_child: i32) {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_begin_move_rows(source_parent, source_first,
            source_last, destination_parent, destination_child);
    }
    fn end_move_rows(&mut self) {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_end_move_rows();
    }
    fn begin_remove_rows(&mut self, parent: &QModelIndex, first: i32, last: i32) {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_begin_remove_rows(parent, first, last);
    }
    fn end_remove_rows(&mut self) {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_end_remove_rows();
    }
    fn begin_reset_model(&mut self) {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_begin_reset_model();
    }
    fn end_reset_model(&mut self) {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_end_reset_model();
    }
}

impl<T> QAbstractListModelBase for T
where
    T : QObjectHolder<ProxyRust = QAbstractListModelProxyRust> {}

pub struct QAbstractListModelProxyRust {
    cpp_proxy: *mut QAbstractListModelProxyCpp,
    #[allow(dead_code)]
    rust_obj: RustObjAccess<dyn QAbstractListModelAdapter>,
    on_drop: fn(rust_obj: *const u8),
}

impl QRustProxy for QAbstractListModelProxyRust {
    type ProxyCppType = QAbstractListModelProxyCpp;
    type AdapterType = dyn QAbstractListModelAdapter;

    fn new(rust_obj: &Rc<RefCell<dyn QAbstractListModelAdapter>>, construct: ConstructionMode, on_drop: fn(rust_obj: *const u8)) -> *mut Self {
        let raw_rust_obj = rust_obj.as_ptr();
        let boxed_self = Box::new(Self {
            cpp_proxy: std::ptr::null_mut(),
            rust_obj: match construct {
                ConstructionMode::Strong | ConstructionMode::AtAddress(_) => RustObjAccess::new_strong(rust_obj.clone()),
                ConstructionMode::Weak => RustObjAccess::new_weak(Rc::downgrade(rust_obj)),
            },
            on_drop,
        });
        let raw_self = Box::into_raw(boxed_self);


        unsafe{ (*raw_self).cpp_proxy = match construct {
            ConstructionMode::AtAddress(addr) => {
                ffi::create_qabstract_list_model_proxy_cpp_at( addr, raw_rust_obj.cast(), raw_self)
            }
            ConstructionMode::Strong | ConstructionMode::Weak => {
                ffi::create_qabstract_list_model_proxy_cpp(raw_rust_obj.cast(), raw_self)
            }
        }};
        raw_self
    }
    fn get_static_meta_object() -> &'static QMetaObject {
        ffi::static_qmeta_object_of_qabstract_list_model_proxy_cpp()
    }
    fn get_size_of_cpp_proxy() -> usize {
        ffi::size_of_qabstract_list_model_proxy_cpp()
    }
    fn get_align_of_cpp_proxy() -> usize {
        ffi::align_of_qabstract_list_model_proxy_cpp()
    }
    fn get_qmetatype_list_of_cpp_proxy() -> QMetaType {
        ffi::qmetatype_list_of_qabstract_list_model_proxy_cpp()
    }
    fn get_cpp_proxy(&self) -> *const QAbstractListModelProxyCpp {
        self.cpp_proxy as *const _
    }
    fn get_cpp_proxy_mut(&self) -> *mut QAbstractListModelProxyCpp {
        self.cpp_proxy
    }
}

impl QAbstractListModelProxyRust {
    pub fn drop_self(raw_self: *mut Self, rust_obj_ptr: *const u8) {
        let boxed_self = unsafe { Box::from_raw(raw_self) };
        (boxed_self.on_drop)(rust_obj_ptr);
    }
    pub fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex {
        call_rust_trait_impl!(self, index(row, column, parent))
    }
    pub fn row_count(&self, parent: &QModelIndex) -> i32 {
        call_rust_trait_impl!(self, row_count(parent))
    }
    pub fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        call_rust_trait_impl!(self, data(index, role))
    }
    pub fn role_names(&self) -> QHash<i32, QByteArray> {
        call_rust_trait_impl!(self, role_names())
    }
    pub fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
        call_rust_trait_impl!(mut self, set_data(index, value, role))
    }
    pub fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool {
        call_rust_trait_impl!(mut self, remove_rows(first, count, parent))
    }
    pub fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex {
        call_rust_trait_impl!(self, sibling(row, column, idx))
    }

    pub fn invoke_slot(&mut self, slot_id: u32, inputs: &[*const u8], outputs: &[*mut u8]) {
        call_rust_trait_impl!(mut self, invoke_slot(slot_id, inputs, outputs))
    }
    pub fn read_property(&self, prop_id: u32) -> QVariant {
        call_rust_trait_impl!(self, read_property(prop_id))
    }
    pub fn write_property(&mut self, prop_id: u32, value: &QVariant) {
        call_rust_trait_impl!(mut self, write_property(prop_id, value))
    }

    pub fn base_index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex {
        call_cpp_impl!(self, base_index(row, column, parent))
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
