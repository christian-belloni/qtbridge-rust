// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::mem::MaybeUninit;
use crate::QJsonValue;

#[qt_gen::bridge]
mod qjsonarray {
    include_in_cpp!(<QJsonArray>);
    include_in_cpp!(<QJsonValue>);

    #[derive_cpp(Default, Drop, Clone)]
    #[derive(Debug)]
    #[qmetatype = 47]
    /// A JSON array, bridging Qt's `QJsonArray` to Rust.
    ///
    /// See also: [QJsonArray documentation](https://doc.qt.io/qt-6/qjsonarray.html).
    struct QJsonArray {
        _content: MaybeUninit<[usize; 1]>,
    }

    /// Returns the number of elements in the array.
    pub fn size(&self) -> isize {
        cpp_fn!(|&self| -> isize {
            return self.size();
        })(self)
    }

    /// Returns the element at position `i`.
    pub fn at(&self, i: isize) -> QJsonValue {
        cpp_fn!(|&self, i: isize| -> QJsonValue {
            return self.at(i);
        })(self, i)
    }

    /// Appends `value` to the end of the array.
    pub fn append(&mut self, value: &QJsonValue) {
        cpp_fn!(|&mut self, value: &QJsonValue| {
            self.append(value);
        })(self, value)
    }

    impl PartialEq for QJsonArray {
        fn eq(&self, other: &Self) -> bool {
            cpp_fn!(|lhs: &Self, rhs: &Self| -> bool {
                return lhs == rhs;
            })(self, other)
        }
    }
}
