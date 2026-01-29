// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::mem::MaybeUninit;
use crate::QVariant;

#[qt_gen::bridge]
mod qmodelindex {
    include_in_cpp!(<QModelIndex>);

    #[derive_cpp(Default, Drop, Clone)]
    #[qmetatype = 42]
    /// The QModelIndex struct is used to locate data in a data model.
    ///
    /// This struct is used as an index into item models derived from QAbstractItemModel.
    /// The index is used by item views, delegates, and selection models to locate an item in the model.
    ///
    /// See also: [QModelIndex documentation](https://doc.qt.io/qt-6/qmodelindex.html).
    struct QModelIndex {
        _r: MaybeUninit<i32>,
        _c: MaybeUninit<i32>,
        _i: MaybeUninit<usize>,
        _m: MaybeUninit<usize>,
    }

    /// Returns the column this model index refers to.
    pub fn column(&self) -> i32 {
        cpp_fn!(|&self| -> i32 {
            return self.column();
        })(self)
    }

    /// Returns the row this model index refers to.
    pub fn row(&self) -> i32 {
        cpp_fn!(|&self| -> i32 {
            return self.row();
        })(self)
    }

    /// Returns a pointer casted to usize used by the model to associate the index with the internal data structure.
    pub fn internal_pointer(&self) -> usize {
        cpp_fn!(|&self| -> usize {
            return reinterpret_cast<size_t>(self.internalPointer());
        })(self)
    }

    /// Returns the data for the given role for the item referred to by the index,
    /// or a default-constructed QVariant if this model index is invalid.
    pub fn data(&self) -> QVariant {
        cpp_fn!(|&self| -> QVariant {
            return self.data();
        })(self)
    }

    /// Returns true if this model index is valid; otherwise returns false.
    /// A valid index belongs to a model, and has non-negative row and column numbers.
    pub fn is_valid(&self) -> bool {
        cpp_fn!(|&self| -> bool {
            return self.isValid();
        })(self)
    }
}
