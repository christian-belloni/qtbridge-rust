// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use super::proxy_cpp_bridge::{QListModelProxyCpp, ffi};
use crate::{RustObjAccess, call_rust_trait_impl, call_cpp_impl};
use qt_type_lib::{QByteArray, QHash, QMetaObject, QMetaType, QModelIndex, QVariant};
use std::cell::RefCell;
use std::rc::Rc;
use qt_traits::QModelItem;

pub trait QListModelProxyGet {
    fn get_rust_proxy(&self) -> &QListModelProxyRust;
    fn get_rust_proxy_mut(&self) -> &mut QListModelProxyRust;
    fn get_trait(&self) -> &dyn QListModelAdapter;
    fn get_trait_mut(&mut self) ->&mut dyn QListModelAdapter;
}

#[doc(hidden)]
pub trait QListModelAdapter {
    fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex;
    fn row_count(&self, parent: &QModelIndex) -> i32;
    fn data(&self, index: &QModelIndex, role: i32) -> QVariant;
    fn role_names(&self) -> QHash<i32, QByteArray>;
    fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool;
    fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool;
    fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex;
}

fn map_role_to_element_id(role: i32) -> i32 {
    if role > 0x6 {
        return role - 0x100;
    }
    role
}

impl<T> QListModelAdapter for T
where
    T: QListModelProxyGet + QListModel {
    fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex {
        let proxy = <Self as QListModelProxyGet>::get_rust_proxy(self);
        proxy.base_index(row, column, parent)
    }
    fn row_count(&self, _parent: &QModelIndex) -> i32 {
        return self.len() as i32;
    }
    fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        let Some(item) = self.get(index.row() as usize)
        else {
            return QVariant::default();
        };
        let element_id = map_role_to_element_id(role);
        return match element_id {
            0  => item.elem0(),
            1  => item.elem1(),
            2  => item.elem2(),
            3  => item.elem3(),
            4  => item.elem4(),
            5  => item.elem5(),
            6  => item.elem6(),
            7  => item.elem7(),
            8  => item.elem8(),
            9  => item.elem9(),
            10 => item.elem10(),
            11 => item.elem11(),
            12 => item.elem12(),
            13 => item.elem13(),
            14 => item.elem14(),
            _  => QVariant::default(),
        };
    }
    fn role_names(&self) -> QHash<i32, QByteArray> {
        let names = T::Item::role_names();
        let mut result = QHash::default();
        names.iter()
            .for_each(|(k, v)| result.insert(k, &QByteArray::from(v)));
        result
    }
    fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
        if !index.is_valid() {
            return false;
        }
        let Some(mut item) = self.get(index.row() as usize)
            .cloned()
        else {
            return false;
        };
        let element_id = map_role_to_element_id(role);
        let updated = match element_id {
            0  => item.set_elem0(value),
            1  => item.set_elem1(value),
            2  => item.set_elem2(value),
            3  => item.set_elem3(value),
            4  => item.set_elem4(value),
            5  => item.set_elem5(value),
            6  => item.set_elem6(value),
            7  => item.set_elem7(value),
            8  => item.set_elem8(value),
            9  => item.set_elem9(value),
            10 => item.set_elem10(value),
            11 => item.set_elem11(value),
            12 => item.set_elem12(value),
            13 => item.set_elem13(value),
            14 => item.set_elem14(value),
            _  => false,
        };
        if updated {
            self.set_unnotified(index.row() as usize, item);
            self.get_rust_proxy_mut().base_data_changed(index, index);
        }
        updated
    }
    fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool {
        let first = first as usize;
        let last = first + count as usize;
        if last > self.len() {
            return false;
        }
        self.get_rust_proxy_mut().base_begin_remove_rows(parent, first as i32, (last - 1) as i32);
        for index in (first..last).rev() {
            self.remove_unnotified(index);
        }
        self.get_rust_proxy_mut().base_end_remove_rows();
        true
    }
    fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex {
        let proxy = <Self as QListModelProxyGet>::get_rust_proxy(self);
        proxy.base_sibling(row, column, idx)
    }
}

pub trait QListModel : QListModelProxyGet {
    type Item: QModelItem + Default + Clone;

    fn len(&self) -> usize;

    fn get(&self, index: usize) -> Option<&Self::Item>;

    fn reset_unnotified(&mut self) {}

    fn set_unnotified(&mut self, _index: usize, _value: Self::Item) -> bool {
        false
    }

    fn push_unnotified(&mut self, value: Self::Item) {
        self.insert_unnotified(self.len(), value);
    }

    fn insert_unnotified(&mut self, _index: usize, _value: Self::Item) {
        panic!("In order to use insert, implement insert_unnotified")
    }

    fn pop_unnotified(&mut self) -> Option<Self::Item> {
        (self.len() > 0)
            .then(|| self.remove_unnotified(self.len() - 1))
    }

    fn remove_unnotified(&mut self, _index: usize) -> Self::Item {
        panic!("In order to use remove, implement remove_unnotified")
    }
}

pub trait QListModelBase : QListModelProxyGet + QListModel {
    fn set(&mut self, index: usize, value: Self::Item) -> bool {
        if self.set_unnotified(index, value) {
            let model_index = <Self as QListModelProxyGet>::get_rust_proxy(self).base_index(index as i32, 0 , &QModelIndex::default());
            self.get_rust_proxy_mut().base_data_changed(&model_index, &model_index);
            true
        } else {
            false
        }
    }
    fn push(&mut self, value: Self::Item) {
        self.get_rust_proxy_mut().base_begin_insert_rows(&QModelIndex::default(), self.len() as i32, self.len() as i32);
        self.push_unnotified(value);
        self.get_rust_proxy_mut().base_end_insert_rows();
    }
    fn insert(&mut self, index: usize, value: Self::Item) {
        self.get_rust_proxy_mut().base_begin_insert_rows(&QModelIndex::default(), index as i32, index as i32);
        self.insert_unnotified(index, value);
        self.get_rust_proxy_mut().base_end_insert_rows();
    }
    fn pop(&mut self) -> Option<Self::Item> {
        if self.len() == 0 {
            return None;
        }
        self.get_rust_proxy_mut().base_begin_remove_rows(&QModelIndex::default(), self.len() as i32 - 1, self.len() as i32 - 1);
        let value = self.pop_unnotified();
        self.get_rust_proxy_mut().base_end_remove_rows();
        value
    }
    fn remove(&mut self, index: usize) -> Self::Item {
        self.get_rust_proxy_mut().base_begin_remove_rows(&QModelIndex::default(), index as i32, index as i32);
        let value = self.remove_unnotified(index);
        self.get_rust_proxy_mut().base_end_remove_rows();
        value
    }
    fn reset(&mut self) {
        self.get_rust_proxy_mut().base_begin_reset_model();
        self.reset_unnotified();
        self.get_rust_proxy_mut().base_end_reset_model();
    }
}

pub struct QListModelProxyRust {
    cpp_proxy: *mut QListModelProxyCpp,
    #[allow(dead_code)]
    rust_obj: RustObjAccess<dyn QListModelProxyGet>,
    on_drop: fn(rust_obj: *const u8),
}
impl QListModelProxyRust {
    pub fn new(rust_obj: &Rc<RefCell<dyn QListModelProxyGet>>, register_strong: bool, on_drop: fn(rust_obj: *const u8)) -> *mut Self {
        let raw_rust_obj = rust_obj.as_ptr();
        let boxed_self = Box::new(Self {
            cpp_proxy: std::ptr::null_mut(),
            rust_obj: match register_strong {
                true => RustObjAccess::new_strong(rust_obj.clone()),
                false => RustObjAccess::new_weak(Rc::downgrade(rust_obj)),
            },
            on_drop,
        });
        let raw_self = Box::into_raw(boxed_self);
        unsafe { (*raw_self).cpp_proxy = ffi::create_qlist_model_proxy_cpp(raw_rust_obj.cast(), raw_self) };
        raw_self
    }
    pub fn new_with_cpp_proxy_at(addr: *mut u8, rust_obj: &Rc<RefCell<dyn QListModelProxyGet>>, on_drop: fn(rust_obj: *const u8)) -> *mut Self {
        let raw_rust_obj = rust_obj.as_ptr();
        let boxed_self = Box::new(Self {
            cpp_proxy: std::ptr::null_mut(),
            rust_obj: RustObjAccess::new_strong(rust_obj.clone()),
            on_drop,
        });
        let raw_self = Box::into_raw(boxed_self);
        unsafe { (*raw_self).cpp_proxy = ffi::create_qlist_model_proxy_cpp_at(addr, raw_rust_obj.cast(), raw_self) };
        raw_self
    }
    pub fn drop_self(raw_self: *mut Self, rust_obj_ptr: *const u8) {
        let boxed_self = unsafe { Box::from_raw(raw_self) };
        (boxed_self.on_drop)(rust_obj_ptr);
    }
    pub fn get_static_meta_object() -> &'static QMetaObject {
        ffi::static_qmeta_object_of_qlist_model_proxy_cpp()
    }
    pub fn get_size_of_cpp_proxy() -> usize {
        ffi::size_of_qlist_model_proxy_cpp()
    }
    pub fn get_align_of_cpp_proxy() -> usize {
        ffi::align_of_qlist_model_proxy_cpp()
    }
    pub fn get_qmetatype_list_of_cpp_proxy() -> QMetaType {
        ffi::qmetatype_list_of_qlist_model_proxy_cpp()
    }
    pub fn get_cpp_proxy(&self) -> *const QListModelProxyCpp {
        self.cpp_proxy as *const _
    }
    pub fn get_cpp_proxy_mut(&self) -> *mut QListModelProxyCpp {
        self.cpp_proxy
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
