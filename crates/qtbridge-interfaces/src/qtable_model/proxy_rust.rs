// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use super::proxy_cpp_bridge::QTableModelProxyCpp;
use crate::{RustObjAccess, call_rust_trait_impl, call_cpp_impl};
use qtbridge_runtime::qproxies::{ConstructionMode, QCppProxy, QRustProxy};
use qtbridge_runtime::{DispatchMetaCall, QObjectHolder, DynamicMetaObjectData};
use qtbridge_runtime::QModelItem;
use qtbridge_type_lib::{QByteArray, QHash, QModelIndex, QVariant};
use std::cell::RefCell;
use std::rc::Rc;

#[doc(hidden)]
pub trait QTableModelAdapter: DispatchMetaCall {
    fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex;
    fn parent(&self, child: &QModelIndex) -> QModelIndex;
    fn row_count(&self, parent: &QModelIndex) -> i32;
    fn column_count(&self, parent: &QModelIndex) -> i32;
    fn data(&self, index: &QModelIndex, role: i32) -> QVariant;
    fn role_names(&self) -> QHash<i32, QByteArray>;
    fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool;
    fn remove_columns(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool;
    fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool;
    fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex;
}

impl<T> QTableModelAdapter for T
where
    T: QTableModel + QObjectHolder<ProxyRust = QTableModelProxyRust> {
    fn index(&self, row: i32, column: i32, _: &QModelIndex) -> QModelIndex {
        let proxy = <Self as QObjectHolder>::get_rust_proxy(self);
        proxy.base_create_index(row, column, 0)
    }
    fn parent(&self, _: &QModelIndex) -> QModelIndex {
        QModelIndex::default()
    }
    fn row_count(&self, _: &QModelIndex) -> i32 {
        self.row_count() as i32
    }
    fn column_count(&self, _: &QModelIndex) -> i32 {
        <Self as QTableModel>::column_count(&self) as i32
    }
    fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        let Some(item) = self.get((index.row() as usize, index.column() as usize))
        else {
            return QVariant::default();
        };
        item.get_role(role)
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
        let Some(mut item) = self.get((index.row() as usize, index.column() as usize))
            .cloned()
        else {
            return false;
        };
        let updated = item.set_role(role, value);
        if updated {
            self.set_unnotified((index.row() as usize, index.column() as usize), item);
            self.get_rust_proxy_mut().base_data_changed(index, index);
        }
        updated
    }
    fn remove_columns(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool {
        let first = first as usize;
        let last = first + count as usize;
        if last > self.column_count() {
            return false;
        }
        self.get_rust_proxy_mut().base_begin_remove_columns(parent, first as i32, (last - 1) as i32);
        for index in (first..last).rev() {
            self.remove_row_unnotified(index);
        }
        self.get_rust_proxy_mut().base_end_remove_columns();
        true
    }
    fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool {
        let first = first as usize;
        let last = first + count as usize;
        if last > self.row_count() {
            return false;
        }
        self.get_rust_proxy_mut().base_begin_remove_rows(parent, first as i32, (last - 1) as i32);
        for index in (first..last).rev() {
            self.remove_row_unnotified(index);
        }
        self.get_rust_proxy_mut().base_end_remove_rows();
        true
    }
    fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex {
        let proxy = self.get_rust_proxy();
        proxy.base_sibling(row, column, idx)
    }
}


/// A trait representing a table-based Qt model.
///
/// [`QTableModel`] provides an interface for table-like data structures
/// that are exposed to Qt through the Model-View concept.
/// <https://doc.qt.io/qt-6/qtquick-modelviewsdata-modelview.html>.
///
/// This trait requires the `qobject` macro to set up the correct Qt proxy.
/// The macro will further generate functionality in the form of the
/// [`QTableModelBase`] trait that supplements the [`QTableModel`] functionality.
///
/// ## Design
///
/// - The model owns items of associated type `Item` that has to implement
///   the [`QModelItem`] trait. Roles are derived from the [`QModelItem`]
///   implementation.
/// - Mutation methods are provided in an **unnotified** form, meaning
///   they modify the underlying data without emitting Qt model signals.
/// - These methods are used by the automatically implemented [`QTableModelBase`]
///   trait to create methods that collaborate the UI about changes in collections.
///
/// As a minimum you have to implement the methods [`QTableModel::row_count`],
/// [`QTableModel::column_count`] and [`QTableModel::get`] to create a readable
/// list model. Further methods can be implemented to make the model fully mutable.
///
/// Methods that do not return an [`Option`] or a boolean value must succeed
/// and perform exactly the operation described in the documentation to avoid
/// invalidating the synchronization between any views and the underlying data.
/// No additional structural changes may occur outside the provided functions.
///
/// **Note, that default implementations may `panic!`** if the corresponding method is
/// not overridden. It is your responsibility to make sure that these functions are
/// not called from QML.
///
/// ## Example
///
/// ``` ignore
/// use qtbridge::qobject;
/// #[qobject(Base = QTableModel)]
/// mod backend {
///     use qtbridge::{QTableModel, QTableModelBase};
///
///     #[derive(Default)]
///     pub struct Backend {
///         string_data: Vec<Vec<String>>,
///     }
///     impl QTableModel for Backend {
///         type Item = String;
///
///         fn len(&self) -> usize {
///             self.string_data.len()
///         }
///         fn column_count(&self) -> usize {
///             self.string_data[0].len
///         }
///         fn get(&self, index: (usize, usize)) -> Option<&Self::Item> {
///             self.string_list.get(index.0)?.get(index.1)
///         }
///     }
/// }
///
/// ```
///
/// The table model can be used in QML views as follows
/// ``` qml, ignore
/// TableView {
///     model: backend
///     delegate: Text {
///         required property string value
///         text: value
///     }
/// }
/// ```
pub trait QTableModel {
    /// The item type stored in the model.
    ///
    /// Items must:
    /// - Implement [`QModelItem`] to integrate with Qt
    /// - Be [`Default`] for creating new items
    /// - Be [`Clone`] for safe data access and copying
    type Item: QModelItem + Default + Clone;

    /// Returns the number of rows in the table.
    fn row_count(&self) -> usize;

    /// Returns the number of columns in the table.
    fn column_count(&self) -> usize;

    /// Returns a reference to the item at `index`, or `None` if the index
    /// is out of bounds.
    fn get(&self, index: (usize, usize)) -> Option<&Self::Item>;

    /// Sets the item at `index`. Reimplement this function but call
    /// [`QTableModelBase::set`] to notify Qt about the modification.
    ///
    /// Returns `true` if the value was successfully set, or `false` if the
    /// operation failed (e.g., index out of bounds or value fails
    /// validation by the business logic).
    ///
    /// The default implementation does nothing and returns `false`.
    fn set_unnotified(&mut self, _index: (usize, usize), _value: Self::Item) -> bool {
        false
    }

    /// Appends a row of items to the end of the model. Reimplement this
    /// function but call [`QTableModelBase::push_row`] to notify Qt about the
    /// modification.
    ///
    /// The function has to accept the value. Validation has to be
    /// done before this function is called.
    ///
    /// The default implementation falls back to [`QTableModel::insert_row_unnotified`],
    /// which in turn panics by default.
    fn push_row_unnotified(&mut self, values: &[Self::Item]) {
        self.insert_row_unnotified(self.row_count(), values);
    }

    /// Appends a column of items to the end of the model. Reimplement this
    /// function but call [`QTableModelBase::push_column`] to notify Qt about the
    /// modification.
    ///
    /// The function has to accept the value. Validation has to be
    /// done before this function is called.
    ///
    /// The default implementation falls back to [`QTableModel::insert_column_unnotified`],
    /// which in turn panics by default.
    fn push_column_unnotified(&mut self, values: &[Self::Item]) {
        self.insert_column_unnotified(self.column_count(), values);
    }

    /// Inserts a row with `value` at `index`. Reimplement this function but
    /// call [`QTableModelBase::insert_row`] to notify Qt about the
    /// modification.
    ///
    /// The function has to accept the value. Validation has to be
    /// done before this function is called.
    ///
    /// Panics by default. Implementors must override this method to support
    /// insertion.
    fn insert_row_unnotified(&mut self, _index: usize, _value: &[Self::Item]) {
        panic!("In order to use insert, implement insert_unnotified")
    }

    /// Inserts a column with `values` at `index`. Reimplement this function but
    /// call [`QTableModelBase::insert_column`] to notify Qt about the
    /// modification.
    ///
    /// The function has to accept the value. Validation has to be
    /// done before this function is called.
    ///
    /// Panics by default. Implementors must override this method to support
    /// insertion.
    fn insert_column_unnotified(&mut self, _index: usize, _value: &[Self::Item]) {
        panic!("In order to use insert, implement insert_unnotified")
    }

    /// Removes and returns the last row in the model. Reimplement this
    /// function but call [`QTableModelBase::pop_row`] to notify Qt
    /// about the modification.
    ///
    /// Returns `None` if the model is empty. If the model is not empty,
    /// the function has to guarantee the success of the operation.
    ///
    /// The default implementation falls back to [`QTableModel::remove_row_unnotified`],
    /// which in turn panics by default.
    fn pop_row_unnotified(&mut self) -> Option<Vec<Self::Item>> {
        (self.row_count() > 0)
            .then(|| self.remove_row_unnotified(self.row_count() - 1))
    }

    /// Removes and returns the last column in the model. Reimplement this
    /// function but call [`QTableModelBase::pop_column`] to notify Qt
    /// about the modification.
    ///
    /// Returns `None` if the model is empty. If the model is not empty,
    /// the function has to guarantee the success of the operation.
    ///
    /// The default implementation falls back to [`QTableModel::remove_column_unnotified`],
    /// which in turn panics by default.
    fn pop_column_unnotified(&mut self) -> Option<Vec<Self::Item>> {
        (self.column_count() > 0)
            .then(|| self.remove_column_unnotified(self.column_count() - 1))
    }

    /// Removes and returns the item at `index`. Reimplement this
    /// function but call [`QTableModelBase::remove_row`] to notify Qt
    /// about the modification.
    ///
    /// The index must be valid and the model has to guarantee the success of
    /// the operation.
    ///
    /// Panics by default. Implementors must override this method to support
    /// removal.
    fn remove_row_unnotified(&mut self, _index: usize) -> Vec<Self::Item> {
        panic!("In order to use remove, implement remove_unnotified")
    }

    /// Removes and returns the column at `index`. Reimplement this
    /// function but call [`QTableModelBase::remove_column`] to notify Qt
    /// about the modification.
    ///
    /// The index must be valid and the model has to guarantee the success of
    /// the operation.
    ///
    /// Panics by default. Implementors must override this method to support
    /// removal.
    fn remove_column_unnotified(&mut self, _index: usize) -> Vec<Self::Item> {
        panic!("In order to use remove, implement remove_unnotified")
    }

    /// Resets the model’s internal storage. Reimplement this function but
    /// call [`QTableModelBase::reset`] to notify Qt about the modification.
    ///
    /// Panics by default. Implementors must override this method to support
    /// a model reset.
    ///
    /// After [`QTableModel::reset_unnotified`] returns, the internal storage
    /// must reflect the new model state: [`QTableModel::row_count`] and
    /// [`QTableModel::get`] must be consistent with the updated storage.
    fn reset_unnotified(&mut self) {
        panic!("In order to use reset, implement reset_unnotified")
    }

}

/// A data-change signaling extension of [`QTableModel`].
///
/// `QTableModelBase` provides the signaling mutation API for list models.
/// The methods defined in this trait wrap the corresponding
/// `*_unnotified` methods from [`QTableModel`] and automatically emit the
/// required Qt model signals (such as `beginInsertRows`, `endInsertRows`,
/// `dataChanged`, etc.). This allows the UI to react to changes in the
/// underlying data.
///
/// This trait is automatically implemented by the [`qobject`] macro and
/// should not be implemented manually.
///
/// ## Usage
///
/// When modifying data that you made accessible with [`QTableModel`], you
/// have to use the functions provided by this trait. Do **not** call the
/// `*_unnotified` methods from [`QTableModel`] directly unless you are
/// manually handling Qt model notifications.
///
/// The correctness of this trait depends on implementors of [`QTableModel`]
/// ensuring that:
///
/// * The `*_unnotified` methods perform the exact mutation corresponding
///   to the emitted Qt signals.
/// * No additional structural changes occur.
///
/// Violating this contract may result in undefined behavior in Qt views.
pub trait QTableModelBase : QTableModel + QObjectHolder<ProxyRust = QTableModelProxyRust> {
    /// Sets the item at `index` and notifies any attached views about
    /// the change, if the operation is successful.
    ///
    /// This method calls [`QTableModel::set_unnotified`].
    ///
    /// Returns `true` if the value was successfully updated,
    /// or `false` if the operation failed (for example, if the index
    /// was out of bounds or validation failed).
    fn set(&mut self, index: (usize, usize), value: <Self as QTableModel>::Item) -> bool {
        if self.set_unnotified(index, value) {
            let model_index = self.get_rust_proxy().index(index.0 as i32, index.1 as i32 , &QModelIndex::default());
            self.get_rust_proxy_mut().base_data_changed(&model_index, &model_index);
            true
        } else {
            false
        }
    }

    /// Appends a row of `values` to the end of the model and notifies any attached views about
    /// the change.
    ///
    /// This method calls [`QTableModel::push_row_unnotified`].
    fn push_row(&mut self, values: &[Self::Item]) {
        self.get_rust_proxy_mut().base_begin_insert_rows(&QModelIndex::default(), self.row_count() as i32, self.row_count() as i32);
        self.push_row_unnotified(values);
        self.get_rust_proxy_mut().base_end_insert_rows();
    }

    /// Appends a column of `value` to the end of the model and notifies any attached views about
    /// the change.
    ///
    /// This method calls [`QTableModel::push_column_unnotified`].
    fn push_column(&mut self, values: &[Self::Item]) {
        self.get_rust_proxy_mut().base_begin_insert_columns(&QModelIndex::default(), self.column_count() as i32, self.column_count() as i32);
        self.push_column_unnotified(values);
        self.get_rust_proxy_mut().base_end_insert_columns();
    }

    /// Inserts a row with `values` at `index` and notifies any attached views about
    /// the change.
    ///
    /// This method calls [`QTableModel::insert_row_unnotified`].
    fn insert_row(&mut self, index: usize, values: &[Self::Item]) {
        self.get_rust_proxy_mut().base_begin_insert_rows(&QModelIndex::default(), index as i32, index as i32);
        self.insert_row_unnotified(index, values);
        self.get_rust_proxy_mut().base_end_insert_rows();
    }

    /// Inserts a column with `values` at `index` and notifies any attached views about
    /// the change.
    ///
    /// This method calls [`QTableModel::insert_column_unnotified`].
    fn insert_column(&mut self, index: usize, values: &[Self::Item]) {
        self.get_rust_proxy_mut().base_begin_insert_columns(&QModelIndex::default(), index as i32, index as i32);
        self.insert_column_unnotified(index, values);
        self.get_rust_proxy_mut().base_end_insert_columns();
    }

    /// Removes and returns the last row in the model and notifies any attached views about
    /// the change.
    ///
    /// This method calls [`QTableModel::pop_row_unnotified`].
    ///
    /// Returns `None` if the model is empty. If the model is not empty,
    /// the function has to guarantee the success of the operation.
    fn pop_row(&mut self) -> Option<Vec<Self::Item>> {
        if self.row_count() == 0 {
            return None;
        }
        self.get_rust_proxy_mut().base_begin_remove_rows(&QModelIndex::default(), self.row_count() as i32 - 1, self.row_count() as i32 - 1);
        let values = self.pop_row_unnotified();
        self.get_rust_proxy_mut().base_end_remove_rows();
        values
    }

    /// Removes and returns the last column in the model and notifies any attached views about
    /// the change.
    ///
    /// This method calls [`QTableModel::pop_column_unnotified`].
    ///
    /// Returns `None` if the model is empty. If the model is not empty,
    /// the function has to guarantee the success of the operation.
    fn pop_column(&mut self) -> Option<Vec<Self::Item>> {
        if self.column_count() == 0 {
            return None;
        }
        self.get_rust_proxy_mut().base_begin_remove_columns(&QModelIndex::default(), self.column_count() as i32 - 1, self.column_count() as i32 - 1);
        let values = self.pop_column_unnotified();
        self.get_rust_proxy_mut().base_end_remove_columns();
        values
    }

    /// Removes and returns the row at `index` and notifies any attached views about
    /// the change.
    ///
    /// This method calls [`QTableModel::remove_row_unnotified`].
    fn remove_row(&mut self, index: usize) -> Vec<Self::Item> {
        self.get_rust_proxy_mut().base_begin_remove_rows(&QModelIndex::default(), index as i32, index as i32);
        let values = self.remove_row_unnotified(index);
        self.get_rust_proxy_mut().base_end_remove_rows();
        values
    }

    /// Removes and returns the column at `index` and notifies any attached views about
    /// the change.
    ///
    /// This method calls [`QTableModel::remove_column_unnotified`].
    fn remove_column(&mut self, index: usize) -> Vec<Self::Item> {
        self.get_rust_proxy_mut().base_begin_remove_columns(&QModelIndex::default(), index as i32, index as i32);
        let values = self.remove_column_unnotified(index);
        self.get_rust_proxy_mut().base_end_remove_columns();
        values
    }

    /// Resets the entire model and notifies any attached views to resyncronize all data.
    ///
    /// This method calls [`QTableModel::reset_unnotified`].
    fn reset(&mut self) {
        self.get_rust_proxy_mut().base_begin_reset_model();
        self.reset_unnotified();
        self.get_rust_proxy_mut().base_end_reset_model();
    }
}

impl<T> QTableModelBase for T
where T: QTableModel + QObjectHolder<ProxyRust = QTableModelProxyRust> { }

pub struct QTableModelProxyRust {
    cpp_proxy: *mut QTableModelProxyCpp,
    rust_obj: RustObjAccess<dyn QTableModelAdapter>,
    on_drop: Box<dyn FnOnce()>,
}

impl QRustProxy for QTableModelProxyRust {

    type ProxyCppType = QTableModelProxyCpp;
    type AdapterType = dyn QTableModelAdapter;

    fn new(rust_obj: &Rc<RefCell<dyn QTableModelAdapter>>, metaobject: &'static DynamicMetaObjectData, construct: ConstructionMode, on_drop: Box<dyn FnOnce() + 'static>) -> *mut Self {
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
                QTableModelProxyCpp::create_at(raw_self, metaobject, addr)
            }
            ConstructionMode::Strong | ConstructionMode::Weak => {
                QTableModelProxyCpp::create(raw_self, metaobject)
            }
        }};
        raw_self
    }
    fn get_cpp_proxy(&self) -> *const QTableModelProxyCpp {
        self.cpp_proxy as *const _
    }
    fn get_cpp_proxy_mut(&self) -> *mut QTableModelProxyCpp {
        self.cpp_proxy
    }
}

impl QTableModelProxyRust {
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
    pub fn remove_columns(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool {
        call_rust_trait_impl!(mut self, remove_columns(first, count, parent))
    }
    pub fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool {
        call_rust_trait_impl!(mut self, remove_rows(first, count, parent))
    }
    pub fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex {
        call_rust_trait_impl!(self, sibling(row, column, idx))
    }
    pub fn base_role_names(&self) -> QHash<i32, QByteArray> {
        call_cpp_impl!(self, base_role_names())
    }
    pub fn base_set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
        call_cpp_impl!(mut self, base_set_data(index, value, role))
    }
    pub fn base_remove_columns(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool {
        call_cpp_impl!(mut self, base_remove_columns(first, count, parent))
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
    pub fn base_create_index(&self, row: i32, column: i32, ptr: usize) -> QModelIndex {
        call_cpp_impl!(self, base_create_index(row, column, ptr))
    }
}
