// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use super::proxy_cpp_bridge::QAbstractItemModelProxyCpp;
use crate::{RustObjAccess2, call_rust_trait_impl2, call_cpp_impl2};
use qtbridge_runtime::qproxies::{ConstructionMode, QCppProxy, QRustProxy};
use qtbridge_runtime::{DispatchMetaCall, QObjectHolder, DynamicMetaObjectData};
use qtbridge_type_lib::{QByteArray, QHash, QModelIndex, QVariant};
use std::cell::RefCell;
use std::rc::Rc;

pub trait QAbstractItemModelAdapter: DispatchMetaCall + 'static {
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
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &*proxy }.base_role_names(self)
    }
    fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_set_data(&mut *self, index, value, role)
    }
    fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_remove_rows(&mut *self, first, count, parent)
    }
    fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &*proxy }.base_sibling(self, row, column, idx)
    }
}

pub trait QAbstractItemModelBase : QAbstractItemModel {
    fn role_names(&self) -> QHash<i32, QByteArray> {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &*proxy }.base_role_names(self)
    }
    fn set_data(
        &mut self,
        index: &QModelIndex,
        value: &QVariant,
        role: i32,
    ) -> bool {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_set_data(&mut *self, index, value, role)
    }
    fn remove_rows(
        &mut self,
        first: i32,
        count: i32,
        parent: &QModelIndex,
    ) -> bool {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_remove_rows(&mut *self, first, count, parent)
    }
    fn sibling(
        &self,
        row: i32,
        column: i32,
        idx: &QModelIndex,
    ) -> QModelIndex {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &*proxy }.base_sibling(self, row, column, idx)
    }
    fn data_changed(
        &mut self,
        top_left: &QModelIndex,
        bottom_right: &QModelIndex,
    ) {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_data_changed(&mut *self, top_left, bottom_right)
    }
    fn begin_insert_columns(
        &mut self,
        parent: &QModelIndex,
        first: i32,
        last: i32,
    ) {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_begin_insert_columns(&mut *self, parent, first, last)
    }
    fn end_insert_columns(&mut self) {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_end_insert_columns(&mut *self)
    }
    fn begin_insert_rows(
        &mut self,
        parent: &QModelIndex,
        first: i32,
        last: i32,
    ) {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_begin_insert_rows(&mut *self, parent, first, last)
    }
    fn end_insert_rows(&mut self) {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_end_insert_rows(&mut *self)
    }
    fn begin_move_columns(
        &mut self,
        source_parent: &QModelIndex,
        source_first: i32,
        source_last: i32,
        destination_parent: &QModelIndex,
        destination_child: i32,
    ) {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_begin_move_columns(
            &mut *self,
            source_parent,
            source_first,
            source_last,
            destination_parent,
            destination_child,
        )
    }
    fn end_move_columns(&mut self) {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_end_move_columns(&mut *self)
    }
    fn begin_move_rows(
        &mut self,
        source_parent: &QModelIndex,
        source_first: i32,
        source_last: i32,
        destination_parent: &QModelIndex,
        destination_child: i32,
    ) {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_begin_move_rows(
            &mut *self,
            source_parent,
            source_first,
            source_last,
            destination_parent,
            destination_child,
        )
    }
    fn end_move_rows(&mut self) {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_end_move_rows(&mut *self)
    }
    fn begin_remove_columns(
        &mut self,
        parent: &QModelIndex,
        first: i32,
        last: i32,
    ) {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_begin_remove_columns(&mut *self, parent, first, last)
    }
    fn end_remove_columns(&mut self) {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_end_remove_columns(&mut *self)
    }
    fn begin_remove_rows(
        &mut self,
        parent: &QModelIndex,
        first: i32,
        last: i32,
    ) {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_begin_remove_rows(&mut *self, parent, first, last)
    }
    fn end_remove_rows(&mut self) {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_end_remove_rows(&mut *self)
    }
    fn begin_reset_model(&mut self) {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_begin_reset_model(&mut *self)
    }
    fn end_reset_model(&mut self) {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &mut *proxy }.base_end_reset_model(&mut *self)
    }
    fn create_index(
        &self,
        row: i32,
        column: i32,
        ptr: usize,
    ) -> QModelIndex {
        let proxy = self.try_get_rust_proxy_ptr().expect("No proxy");
        unsafe { &*proxy }.base_create_index(self, row, column, ptr)
    }
}

impl<T> QAbstractItemModelBase for T
where T: QAbstractItemModel {}

pub struct QAbstractItemModelProxyRust {
    cpp_proxy: *mut QAbstractItemModelProxyCpp,
    rust_obj: RustObjAccess2<dyn QAbstractItemModelAdapter>,
    on_drop: Box<dyn FnOnce()>,
}

impl QRustProxy for QAbstractItemModelProxyRust {
    type ProxyCppType = QAbstractItemModelProxyCpp;
    type AdapterType = dyn QAbstractItemModelAdapter;

    fn new(rust_obj: &Rc<RefCell<dyn QAbstractItemModelAdapter>>, metaobject: &'static DynamicMetaObjectData, construct: ConstructionMode, on_drop: Box<dyn FnOnce() + 'static>) -> *mut Self {
        let boxed_self = Box::new(Self {
            cpp_proxy: std::ptr::null_mut(),
            rust_obj: match construct {
                ConstructionMode::Strong | ConstructionMode::AtAddress(_) => RustObjAccess2::new_strong(rust_obj.clone()),
                ConstructionMode::Weak => RustObjAccess2::new_weak(Rc::downgrade(rust_obj)),
            },
            on_drop,
        });
        let raw_self = Box::into_raw(boxed_self);

        unsafe{ (*raw_self).cpp_proxy = match construct {
            ConstructionMode::AtAddress(addr) => {
                QAbstractItemModelProxyCpp::create_at(raw_self, metaobject, addr)
            }
            ConstructionMode::Strong | ConstructionMode::Weak => {
                QAbstractItemModelProxyCpp::create(raw_self, metaobject)
            }
        }};
        raw_self
    }
    fn get_cpp_proxy(&self) -> *const QAbstractItemModelProxyCpp {
        self.cpp_proxy as *const _
    }
    fn get_cpp_proxy_mut(&self) -> *mut QAbstractItemModelProxyCpp {
        self.cpp_proxy
    }
    fn emit_signal(&self, reference: &Self::AdapterType, signal_name: &str, argv: &[*const u8]) {
        call_cpp_impl2!(self, reference, emit_signal(signal_name, argv))
    }
    fn emit_signal_mut(&self, mut_ref: &mut Self::AdapterType, signal_name: &str, argv: &[*const u8]) {
        call_cpp_impl2!(mut self, mut_ref, emit_signal_mut(signal_name, argv))
    }
}

impl QAbstractItemModelProxyRust {
    pub fn drop_self(self_ptr: *mut Self) {
        let boxed_self = unsafe { Box::from_raw(self_ptr) };
        (boxed_self.on_drop)();
    }

    pub fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex {
        call_rust_trait_impl2!(self, index(row, column, parent))
    }
    pub fn parent(&self, child: &QModelIndex) -> QModelIndex {
        call_rust_trait_impl2!(self, parent(child))
    }
    pub fn row_count(&self, parent: &QModelIndex) -> i32 {
        call_rust_trait_impl2!(self, row_count(parent))
    }
    pub fn column_count(&self, parent: &QModelIndex) -> i32 {
        call_rust_trait_impl2!(self, column_count(parent))
    }
    pub fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        call_rust_trait_impl2!(self, data(index, role))
    }
    pub fn role_names(&self) -> QHash<i32, QByteArray> {
        call_rust_trait_impl2!(self, role_names())
    }
    pub fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
        call_rust_trait_impl2!(mut self, set_data(index, value, role))
    }
    pub fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool {
        call_rust_trait_impl2!(mut self, remove_rows(first, count, parent))
    }
    pub fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex {
        call_rust_trait_impl2!(self, sibling(row, column, idx))
    }

    pub fn invoke_slot(&self, slot_id: u32, inputs: &[*const u8], outputs: &[*mut u8]) {
        call_rust_trait_impl2!(self, invoke_slot(slot_id, inputs, outputs))
    }
    pub fn invoke_slot_mut(&mut self, slot_id: u32, inputs: &[*const u8], outputs: &[*mut u8]) {
        call_rust_trait_impl2!(mut self, invoke_slot_mut(slot_id, inputs, outputs))
    }
    pub fn read_property(&self, prop_id: u32) -> QVariant {
        call_rust_trait_impl2!(self, read_property(prop_id))
    }
    pub fn write_property(&mut self, prop_id: u32, value: &QVariant) {
        call_rust_trait_impl2!(mut self, write_property(prop_id, value))
    }

    pub fn get_rust_object_rc_ptr(&self) -> *const u8 {
        self.rust_obj.get_rc()
            .map_or(std::ptr::null(), |rc| Rc::into_raw(rc) as *const u8)
    }

    pub fn base_role_names(&self, reference: &dyn QAbstractItemModelAdapter) -> QHash<i32, QByteArray> {
        call_cpp_impl2!(self, reference, base_role_names())
    }
    pub fn base_set_data(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
        call_cpp_impl2!(mut self, mut_ref, base_set_data(index, value, role))
    }
    pub fn base_remove_rows(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter, first: i32, count: i32, parent: &QModelIndex) -> bool {
        call_cpp_impl2!(mut self, mut_ref, base_remove_rows(first, count, parent))
    }
    pub fn base_sibling(&self, reference: &dyn QAbstractItemModelAdapter, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex {
        call_cpp_impl2!(self, reference, base_sibling(row, column, idx))
    }
    pub fn base_data_changed(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter, top_left: &QModelIndex, bottom_right: &QModelIndex) {
        call_cpp_impl2!(mut self, mut_ref, base_data_changed(top_left, bottom_right))
    }
    pub fn base_begin_insert_columns(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter, parent: &QModelIndex, first: i32, last: i32) {
        call_cpp_impl2!(mut self, mut_ref, base_begin_insert_columns(parent, first, last))
    }
    pub fn base_end_insert_columns(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter) {
        call_cpp_impl2!(mut self, mut_ref, base_end_insert_columns())
    }
    pub fn base_begin_insert_rows(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter, parent: &QModelIndex, first: i32, last: i32) {
        call_cpp_impl2!(mut self, mut_ref, base_begin_insert_rows(parent, first, last))
    }
    pub fn base_end_insert_rows(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter) {
        call_cpp_impl2!(mut self, mut_ref, base_end_insert_rows())
    }
    pub fn base_begin_move_columns(
        &mut self,
        mut_ref: &mut dyn QAbstractItemModelAdapter,
        source_parent: &QModelIndex,
        source_first: i32,
        source_last: i32,
        destination_parent: &QModelIndex,
        destination_child: i32,
    ) {
        call_cpp_impl2!(mut self, mut_ref, base_begin_move_columns(source_parent, source_first, source_last, destination_parent, destination_child))
    }
    pub fn base_end_move_columns(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter) {
        call_cpp_impl2!(mut self, mut_ref, base_end_move_columns())
    }
    pub fn base_begin_move_rows(
        &mut self,
        mut_ref: &mut dyn QAbstractItemModelAdapter,
        source_parent: &QModelIndex,
        source_first: i32,
        source_last: i32,
        destination_parent: &QModelIndex,
        destination_child: i32,
    ) {
        call_cpp_impl2!(mut self, mut_ref, base_begin_move_rows(source_parent, source_first, source_last, destination_parent, destination_child))
    }
    pub fn base_end_move_rows(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter) {
        call_cpp_impl2!(mut self, mut_ref, base_end_move_rows())
    }
    pub fn base_begin_remove_columns(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter, parent: &QModelIndex, first: i32, last: i32) {
        call_cpp_impl2!(mut self, mut_ref, base_begin_remove_columns(parent, first, last))
    }
    pub fn base_end_remove_columns(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter) {
        call_cpp_impl2!(mut self, mut_ref, base_end_remove_columns())
    }
    pub fn base_begin_remove_rows(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter, parent: &QModelIndex, first: i32, last: i32) {
        call_cpp_impl2!(mut self, mut_ref, base_begin_remove_rows(parent, first, last))
    }
    pub fn base_end_remove_rows(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter) {
        call_cpp_impl2!(mut self, mut_ref, base_end_remove_rows())
    }
    pub fn base_begin_reset_model(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter) {
        call_cpp_impl2!(mut self, mut_ref, base_begin_reset_model())
    }
    pub fn base_end_reset_model(&mut self, mut_ref: &mut dyn QAbstractItemModelAdapter) {
        call_cpp_impl2!(mut self, mut_ref, base_end_reset_model())
    }
    pub fn base_create_index(&self, reference: &dyn QAbstractItemModelAdapter, row: i32, column: i32, ptr: usize) -> QModelIndex {
        call_cpp_impl2!(self, reference, base_create_index(row, column, ptr))
    }
}
