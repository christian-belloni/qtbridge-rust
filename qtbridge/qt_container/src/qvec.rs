// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qt_gen::qobject_internal;

#[qobject_internal(Base = QAbstractItemModel)]
mod qvec
{
    use std::fmt;
    use std::slice::SliceIndex;
    use qt_type_lib::{QByteArray, QHash, QModelIndex, QVariant};
    use qt_traits::QModelItem;
    use qt_ifaces::{QAbstractItemModel, QAbstractItemModelBase};

    /// A Qt-aware vector that acts both as a container and a `QAbstractItemModel`.
    ///
    /// `QVec<T>` mirrors the API of `Vec<T>` where possible (`push`, `pop`, `len`,
    /// indexing, iteration) but emits the necessary Qt model-change signals so QML
    /// views update automatically.
    ///
    /// # Supported item types and respective role names
    ///
    /// * **Primitive types** (e.g. `QVec<i32>`): exposed under the `"value"` role.
    /// * **Tuples** (e.g. `QVec<(i32, String)>`): roles are `"_0"`, `"_1"`, ...
    /// * **Structs implementing `QModelItem`**: roles come from the trait;
    ///   `#[derive(QModelItem)]` generates roles from field names.
    ///
    /// Up to 15 roles are supported.
    ///
    /// # No mutable references
    ///
    /// Methods like `get_mut`, `index_mut`, and `iter_mut` are intentionally
    /// unavailable because modifying items through direct references would bypass
    /// Qt’s notification system. Use `set` to update elements so QML views are
    /// notified correctly.
    ///
    /// # QML integration
    ///
    /// A model created with `QVec<T>` can be exposed as a QAbstractItemModel to QML.
    /// In QML, the model can be used to fill ListViews, Repeaters, and similar items.
    ///
    pub struct QVec<T>
    where
        T: QModelItem + Default + 'static
    {
        data: Vec<T>,
    }

    impl<T: fmt::Debug> fmt::Debug for QVec<T>
    where
        T: QModelItem + fmt::Debug + Default + 'static
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("QVec")
                .field("data", &self.data)
                .finish()
        }
    }

    impl<T> Default for QVec<T>
    where
        T: QModelItem + Default + 'static
    {
        /// Creates an empty `QVec`, requiring `T: Default + QModelItem`.
        fn default() -> Self {
            Self::new(Vec::default())
        }
    }

    impl<T> QVec<T>
    where
        T: QModelItem + Default + 'static
    {
        /// Creates a new `QVec` from the given vector.
        ///
        /// Since any modification of QVec emits signals to Qt, it is most
        /// efficiently created with this function.
        pub fn new(data: Vec<T>) -> Self {
            QVec {
                data: data,
            }
        }

        /// Appends an element to the end of the vector and notifies QML views.
        pub fn push(&mut self, value: T) {
            self.begin_insert_rows(&QModelIndex::default(),
                self.len() as i32,
                self.len() as i32);
            self.data.push(value);
            self.end_insert_rows();
        }

        /// Removes the last element and notifies QML views.
        ///
        /// Returns [`None`] if the vector is empty.
        pub fn pop(&mut self) -> Option<T> {
            self.begin_remove_rows(&QModelIndex::default(),
                self.len() as i32 - 1,
                self.len() as i32 - 1);
            let result = self.data.pop();
            self.end_remove_rows();
            result
        }

        /// Returns the number of elements in the vector.
        pub fn len(&self) -> usize {
            self.data.len()
        }

        /// Returns `true` if the vector contains no elements.
        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        /// Returns a reference to an element or subslice depending on the type of index.
        ///
        /// * If given a position, returns a reference to the element at that position or None if out of bounds.
        /// * If given a range, returns the subslice corresponding to that range, or None if out of bounds.
        ///
        /// Note: No mutable access is provided because it would bypass Qt change
        /// notifications.
        ///
        pub fn get<I>(&self, index: I) -> Option<&I::Output>
        where
            I: SliceIndex<[T]>,
        {
            self.data.get(index)
        }

        /// Replaces the element at `index` and emits a `dataChanged` signal.
        ///
        /// Use this instead of mutating elements in place so QML views update correctly.
        pub fn set(&mut self, index: usize, value: T) {
            self.data[index] = value;
            let index_ref = self.create_index(index as i32,0, 0);
            self.data_changed(&index_ref, &index_ref);
        }

        /// Clears all items and resets the model.
        ///
        /// Emits a full model reset; views will completely refresh.
        pub fn clear(&mut self) {
            self.begin_reset_model();
            self.data.clear();
            self.end_reset_model();
        }
    }

    impl<T: Default + QModelItem, I: SliceIndex<[T]>> std::ops::Index<I> for QVec<T> {
        type Output = I::Output;

        /// Indexing operator (`qvec[i]`). Panics if out of bounds.
        ///
        /// Only shared indexing is supported; mutable indexing is intentionally omitted.
        fn index(&self, index: I) -> &Self::Output {
            &self.data.index(index)
        }
    }

    impl<T: Default + QModelItem> QVec<T> {
        /// Returns an iterator over the slice.
        ///
        /// The iterator yields all items from start to end.
        pub fn iter(&self) -> std::slice::Iter<'_, T> {
            self.data.iter()
        }
    }

    impl<T> QVec<T>
    where
        T: QModelItem + Default + 'static
    {
        fn map_role_to_element_id(&self, role: i32) -> i32 {
            if role > 0x6 {
                return role - 0x100;
            }
            role
        }
    }

    impl<T> QAbstractItemModel for QVec<T>
    where
        T: QModelItem + Default + 'static
    {
        fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex {
            self.create_index(row, column, parent.internal_pointer())
        }

        fn parent(&self, _child: &QModelIndex) -> QModelIndex {
            QModelIndex::default()
        }

        fn row_count(&self, parent: &QModelIndex) -> i32 {
            if parent.is_valid() {
                return 0;
            }
            self.data.len() as i32
        }

        fn column_count(&self, _parent: &QModelIndex) -> i32 {
            1
        }

        fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
            if !index.is_valid() {
                return QVariant::default()
            }

            let role = self.map_role_to_element_id(role);
            let row = index.row() as usize;

            match role {
                0 => self.data[row].elem0(),
                1 => self.data[row].elem1(),
                2 => self.data[row].elem2(),
                3 => self.data[row].elem3(),
                4 => self.data[row].elem4(),
                5 => self.data[row].elem5(),
                6 => self.data[row].elem6(),
                7 => self.data[row].elem7(),
                8 => self.data[row].elem8(),
                9 => self.data[row].elem9(),
                10 => self.data[row].elem10(),
                11 => self.data[row].elem11(),
                12 => self.data[row].elem12(),
                13 => self.data[row].elem13(),
                14 => self.data[row].elem14(),
                _ => QVariant::default()
            }
        }

        fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
            if !index.is_valid() {
                return false;
            }

            let role = self.map_role_to_element_id(role);
            let indexed_value = &mut self.data[index.row() as usize];

            match role {
                0 => indexed_value.set_elem0(value),
                1 => indexed_value.set_elem1(value),
                2 => indexed_value.set_elem2(value),
                3 => indexed_value.set_elem3(value),
                4 => indexed_value.set_elem4(value),
                5 => indexed_value.set_elem5(value),
                6 => indexed_value.set_elem6(value),
                7 => indexed_value.set_elem7(value),
                8 => indexed_value.set_elem8(value),
                9 => indexed_value.set_elem9(value),
                10 => indexed_value.set_elem10(value),
                11 => indexed_value.set_elem11(value),
                12 => indexed_value.set_elem12(value),
                13 => indexed_value.set_elem13(value),
                14 => indexed_value.set_elem14(value),
                _ => false,
            }
        }

        fn role_names(&self)-> QHash<i32, QByteArray> {
            let names = T::role_names();
            // TODO: If names is empty, we could revert to
            // QAbstractItemModel::role_names() / default role names.
            let mut result = QHash::default();
            names.iter()
                .for_each(|(k, v)| result.insert(k, &QByteArray::from(v)));
            result
        }
    }
}
pub use qvec::QVec;
