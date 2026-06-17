// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::mem::MaybeUninit;
use crate::{QJsonValue, QString};

#[qt_gen::bridge]
mod qjsonobject {
    include_in_cpp!(<QJsonObject>);
    include_in_cpp!(<QJsonValue>);
    include_in_cpp!("rustconv.h");

    #[derive_cpp(Default, Drop, Clone)]
    #[derive(Debug)]
    #[qmetatype = 46]
    /// A JSON object, bridging Qt's `QJsonObject` to Rust.
    ///
    /// See also: [QJsonObject documentation](https://doc.qt.io/qt-6/qjsonobject.html).
    struct QJsonObject {
        _content: MaybeUninit<[usize; 1]>,
    }

    /// Returns the number of key-value pairs in the object.
    pub fn size(&self) -> isize {
        cpp_fn!(|&self| -> isize {
            return self.size();
        })(self)
    }

    /// Returns all keys as a `Vec<String>`.
    pub fn keys(&self) -> Vec<String> {
        cpp_fn!(|&self| -> Vec<String> {
            rust::Vec<rust::String> result;
            result.reserve(self.size());
            for (auto it = self.constBegin(); it != self.constEnd(); ++it)
                result.push_back(QStringToRustString(it.key()));
            return result;
        })(self)
    }

    /// Returns the value for `key`, or a null `QJsonValue` if the key is absent.
    pub fn value(&self, key: &QString) -> QJsonValue {
        cpp_fn!(|&self, key: &QString| -> QJsonValue {
            return self.value(key);
        })(self, key)
    }

    /// Inserts or replaces the value for `key`.
    pub fn insert(&mut self, key: &QString, value: &QJsonValue) {
        cpp_fn!(|&mut self, key: &QString, value: &QJsonValue| {
            self.insert(key, value);
        })(self, key, value)
    }

    impl PartialEq for QJsonObject {
        fn eq(&self, other: &Self) -> bool {
            cpp_fn!(|lhs: &Self, rhs: &Self| -> bool {
                return lhs == rhs;
            })(self, other)
        }
    }

}
