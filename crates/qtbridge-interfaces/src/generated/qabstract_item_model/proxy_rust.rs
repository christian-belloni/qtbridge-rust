// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use super::proxy_cpp_bridge::{QAbstractItemModelProxyCpp, ffi};
use crate::{RustObjAccess, call_rust_trait_impl, call_cpp_impl};
use qtbridge_runtime::qrustproxy::{QRustProxy, ConstructionMode};
use qtbridge_runtime::{DispatchMetaCall, QObjectHolder};
use qtbridge_type_lib::{QByteArray, QHash, QMetaObject, QMetaType, QModelIndex, QVariant};
use std::cell::RefCell;
use std::rc::Rc;

pub trait QAbstractItemModelAdapter: DispatchMetaCall {
    fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex;
    fn parent(&self, child: &QModelIndex) -> QModelIndex;
    fn row_count(&self, parent: &QModelIndex) -> i32;
    fn column_count(&self, parent: &QModelIndex) -> i32;
    fn data(&self, index: &QModelIndex, role: i32) -> QVariant;
    fn role_names(&self) -> QHash<i32, QByteArray>;
    fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool;
    fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool;
    fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex;
}

impl<T> QAbstractItemModelAdapter for T
where
    T: QAbstractItemModel {
    fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex {
        <Self as QAbstractItemModel>::index(self, row, column, parent)
    }
    fn parent(&self, child: &QModelIndex) -> QModelIndex {
        <Self as QAbstractItemModel>::parent(self, child)
    }
    fn row_count(&self, parent: &QModelIndex) -> i32 {
        <Self as QAbstractItemModel>::row_count(self, parent)
    }
    fn column_count(&self, parent: &QModelIndex) -> i32 {
        <Self as QAbstractItemModel>::column_count(self, parent)
    }
    fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        <Self as QAbstractItemModel>::data(self, index, role)
    }
    fn role_names(&self) -> QHash<i32, QByteArray> {
        <Self as QAbstractItemModel>::role_names(self)
    }
    fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
        <Self as QAbstractItemModel>::set_data(self, index, value, role)
    }
    fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool {
        <Self as QAbstractItemModel>::remove_rows(self, first, count, parent)
    }
    fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex {
        <Self as QAbstractItemModel>::sibling(self, row, column, idx)
    }
}

pub trait QAbstractItemModel : QObjectHolder<ProxyRust = QAbstractItemModelProxyRust> {
    fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex;
    fn parent(&self, child: &QModelIndex) -> QModelIndex;
    fn row_count(&self, parent: &QModelIndex) -> i32;
    fn column_count(&self, parent: &QModelIndex) -> i32;
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

pub trait QAbstractItemModelBase : QObjectHolder<ProxyRust = QAbstractItemModelProxyRust> {
fn role_names(&self) -> qtbridge_type_lib::QHash<i32, qtbridge_type_lib::QByteArray> {
        let proxy = self.get_rust_proxy();
        proxy.base_role_names()
    }
    fn set_data(
        &mut self,
        index: &qtbridge_type_lib::QModelIndex,
        value: &qtbridge_type_lib::QVariant,
        role: i32,
    ) -> bool {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_set_data(index, value, role)
    }
    fn remove_rows(
        &mut self,
        first: i32,
        count: i32,
        parent: &qtbridge_type_lib::QModelIndex,
    ) -> bool {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_remove_rows(first, count, parent)
    }
    fn sibling(
        &self,
        row: i32,
        column: i32,
        idx: &qtbridge_type_lib::QModelIndex,
    ) -> qtbridge_type_lib::QModelIndex {
        let proxy = self.get_rust_proxy();
        proxy.base_sibling(row, column, idx)
    }
    fn data_changed(
        &mut self,
        top_left: &qtbridge_type_lib::QModelIndex,
        bottom_right: &qtbridge_type_lib::QModelIndex,
    ) {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_data_changed(top_left, bottom_right)
    }
    fn begin_insert_columns(
        &mut self,
        parent: &qtbridge_type_lib::QModelIndex,
        first: i32,
        last: i32,
    ) {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_begin_insert_columns(parent, first, last)
    }
    fn end_insert_columns(&mut self) {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_end_insert_columns()
    }
    fn begin_insert_rows(
        &mut self,
        parent: &qtbridge_type_lib::QModelIndex,
        first: i32,
        last: i32,
    ) {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_begin_insert_rows(parent, first, last)
    }
    fn end_insert_rows(&mut self) {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_end_insert_rows()
    }
    fn begin_move_columns(
        &mut self,
        source_parent: &qtbridge_type_lib::QModelIndex,
        source_first: i32,
        source_last: i32,
        destination_parent: &qtbridge_type_lib::QModelIndex,
        destination_child: i32,
    ) {
        let proxy = self.get_rust_proxy_mut();
        proxy
            .base_begin_move_columns(
                source_parent,
                source_first,
                source_last,
                destination_parent,
                destination_child,
            )
    }
    fn end_move_columns(&mut self) {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_end_move_columns()
    }
    fn begin_move_rows(
        &mut self,
        source_parent: &qtbridge_type_lib::QModelIndex,
        source_first: i32,
        source_last: i32,
        destination_parent: &qtbridge_type_lib::QModelIndex,
        destination_child: i32,
    ) {
        let proxy = self.get_rust_proxy_mut();
        proxy
            .base_begin_move_rows(
                source_parent,
                source_first,
                source_last,
                destination_parent,
                destination_child,
            )
    }
    fn end_move_rows(&mut self) {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_end_move_rows()
    }
    fn begin_remove_columns(
        &mut self,
        parent: &qtbridge_type_lib::QModelIndex,
        first: i32,
        last: i32,
    ) {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_begin_remove_columns(parent, first, last)
    }
    fn end_remove_columns(&mut self) {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_end_remove_columns()
    }
    fn begin_remove_rows(
        &mut self,
        parent: &qtbridge_type_lib::QModelIndex,
        first: i32,
        last: i32,
    ) {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_begin_remove_rows(parent, first, last)
    }
    fn end_remove_rows(&mut self) {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_end_remove_rows()
    }
    fn begin_reset_model(&mut self) {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_begin_reset_model()
    }
    fn end_reset_model(&mut self) {
        let proxy = self.get_rust_proxy_mut();
        proxy.base_end_reset_model()
    }
    fn create_index(
        &self,
        row: i32,
        column: i32,
        ptr: usize,
    ) -> qtbridge_type_lib::QModelIndex {
        let proxy = self.get_rust_proxy_mut();
        unsafe { proxy.base_create_index(row, column, ptr) }
    }
}

impl<T> QAbstractItemModelBase for T
where T: QObjectHolder<ProxyRust = QAbstractItemModelProxyRust> {}

pub struct QAbstractItemModelProxyRust {
    cpp_proxy: *mut QAbstractItemModelProxyCpp,
    #[allow(dead_code)]
    rust_obj: RustObjAccess<dyn QAbstractItemModelAdapter>,
    on_drop: Box<dyn FnOnce()>,
}

impl QRustProxy for QAbstractItemModelProxyRust {
    type ProxyCppType = QAbstractItemModelProxyCpp;
    type AdapterType = dyn QAbstractItemModelAdapter;

    fn new<OnDropFn: FnOnce() + 'static>(rust_obj: &Rc<RefCell<dyn QAbstractItemModelAdapter>>, construct: ConstructionMode, on_drop: OnDropFn) -> *mut Self {
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
                ffi::create_qabstract_item_model_proxy_cpp_at(addr, raw_self)
            }
            ConstructionMode::Strong | ConstructionMode::Weak => {
                ffi::create_qabstract_item_model_proxy_cpp(raw_self)
            }
        }};
        raw_self
    }
    fn get_static_meta_object() -> &'static QMetaObject {
        ffi::static_qmeta_object_of_qabstract_item_model_proxy_cpp()
    }
    fn get_size_of_cpp_proxy() -> usize {
        ffi::size_of_qabstract_item_model_proxy_cpp()
    }
    fn get_align_of_cpp_proxy() -> usize {
        ffi::align_of_qabstract_item_model_proxy_cpp()
    }
    fn get_qmetatype_list_of_cpp_proxy() -> QMetaType {
        ffi::qmetatype_list_of_qabstract_item_model_proxy_cpp()
    }
    fn get_cpp_proxy(&self) -> *const QAbstractItemModelProxyCpp {
        self.cpp_proxy as *const _
    }
    fn get_cpp_proxy_mut(&self) -> *mut QAbstractItemModelProxyCpp {
        self.cpp_proxy
    }
}

impl QAbstractItemModelProxyRust {
    pub fn drop_self(self_ptr: *mut Self) {
        let boxed_self = unsafe { Box::from_raw(self_ptr) };
        (boxed_self.on_drop)();
    }
    pub fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex {
        call_rust_trait_impl!(self, index(row, column, parent))
    }
    pub fn parent(&self, child: &QModelIndex) -> QModelIndex {
        call_rust_trait_impl!(self, parent(child))
    }
    pub fn row_count(&self, parent: &QModelIndex) -> i32 {
        call_rust_trait_impl!(self, row_count(parent))
    }
    pub fn column_count(&self, parent: &QModelIndex) -> i32 {
        call_rust_trait_impl!(self, column_count(parent))
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
    pub fn base_begin_insert_columns(&mut self, parent: &QModelIndex, first: i32, last: i32) {
        call_cpp_impl!(mut self, base_begin_insert_columns(parent, first, last))
    }
    pub fn base_end_insert_columns(&mut self) {
        call_cpp_impl!(mut self, base_end_insert_columns())
    }
    pub fn base_begin_insert_rows(&mut self, parent: &QModelIndex, first: i32, last: i32) {
        call_cpp_impl!(mut self, base_begin_insert_rows(parent, first, last))
    }
    pub fn base_end_insert_rows(&mut self) {
        call_cpp_impl!(mut self, base_end_insert_rows())
    }
    pub fn base_begin_move_columns(
        &mut self,
        source_parent: &QModelIndex,
        source_first: i32,
        source_last: i32,
        destination_parent: &QModelIndex,
        destination_child: i32,
    ) {
        call_cpp_impl!(mut self, base_begin_move_columns(source_parent, source_first, source_last, destination_parent, destination_child))
    }
    pub fn base_end_move_columns(&mut self) {
        call_cpp_impl!(mut self, base_end_move_columns())
    }
    pub fn base_begin_move_rows(&mut self, source_parent: &QModelIndex, source_first: i32, source_last: i32, destination_parent: &QModelIndex, destination_child: i32) {
        call_cpp_impl!(mut self, base_begin_move_rows(source_parent, source_first, source_last, destination_parent, destination_child))
    }
    pub fn base_end_move_rows(&mut self) {
        call_cpp_impl!(mut self, base_end_move_rows())
    }
    pub fn base_begin_remove_columns(&mut self, parent: &QModelIndex, first: i32, last: i32) {
        call_cpp_impl!(mut self, base_begin_remove_columns(parent, first, last))
    }
    pub fn base_end_remove_columns(&mut self) {
        call_cpp_impl!(mut self, base_end_remove_columns())
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
    pub unsafe fn base_create_index(&self, row: i32, column: i32, ptr: usize) -> QModelIndex {
        call_cpp_impl!(self, base_create_index(row, column, ptr))
    }
}
