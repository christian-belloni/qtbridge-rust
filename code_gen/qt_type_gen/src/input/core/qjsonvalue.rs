// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::mem::MaybeUninit;
use crate::{QJsonArray, QJsonObject, QString};

#[qt_gen::bridge]
mod qjsonvalue {
    include_in_cpp!(<QJsonValue>);
    include_in_cpp!(<QJsonArray>);
    include_in_cpp!(<QJsonObject>);

    #[derive_cpp(Default, Drop, Clone)]
    #[derive(Debug)]
    #[qmetatype = 45]
    /// A single JSON value of any type, bridging Qt's `QJsonValue` to Rust.
    ///
    /// The default-constructed value is `QJsonValue::Null`.
    ///
    /// See also: [QJsonValue documentation](https://doc.qt.io/qt-6/qjsonvalue.html).
    struct QJsonValue {
        _content: MaybeUninit<[usize; 3]>,
    }

    /// Returns `true` if the value is null.
    pub fn is_null(&self) -> bool {
        cpp_fn!(|&self| -> bool {
            return self.isNull();
        })(self)
    }

    /// Returns `true` if the value is undefined.
    pub fn is_undefined(&self) -> bool {
        cpp_fn!(|&self| -> bool {
            return self.isUndefined();
        })(self)
    }

    /// Returns `true` if the value is a boolean.
    pub fn is_bool(&self) -> bool {
        cpp_fn!(|&self| -> bool {
            return self.isBool();
        })(self)
    }

    /// Returns `true` if the value is a number (double).
    pub fn is_double(&self) -> bool {
        cpp_fn!(|&self| -> bool {
            return self.isDouble();
        })(self)
    }

    /// Returns `true` if the value is a string.
    pub fn is_string(&self) -> bool {
        cpp_fn!(|&self| -> bool {
            return self.isString();
        })(self)
    }

    /// Returns `true` if the value is an array.
    pub fn is_array(&self) -> bool {
        cpp_fn!(|&self| -> bool {
            return self.isArray();
        })(self)
    }

    /// Returns `true` if the value is an object.
    pub fn is_object(&self) -> bool {
        cpp_fn!(|&self| -> bool {
            return self.isObject();
        })(self)
    }

    /// Extracts the boolean value. Returns `false` if not a boolean.
    pub fn to_bool(&self) -> bool {
        cpp_fn!(|&self| -> bool {
            return self.toBool();
        })(self)
    }

    /// Extracts the numeric value as `f64`. Returns `0.0` if not a number.
    pub fn to_double(&self) -> f64 {
        cpp_fn!(|&self| -> f64 {
            return self.toDouble();
        })(self)
    }

    /// Extracts the string value. Returns an empty string if not a string.
    pub fn to_string(&self) -> QString {
        cpp_fn!(|&self| -> QString {
            return self.toString();
        })(self)
    }

    /// Extracts the array value. Returns an empty array if not an array.
    pub fn to_array(&self) -> QJsonArray {
        cpp_fn!(|&self| -> QJsonArray {
            return self.toArray();
        })(self)
    }

    /// Extracts the object value. Returns an empty object if not an object.
    pub fn to_object(&self) -> QJsonObject {
        cpp_fn!(|&self| -> QJsonObject {
            return self.toObject();
        })(self)
    }

    impl PartialEq for QJsonValue {
        fn eq(&self, other: &Self) -> bool {
            cpp_fn!(|lhs: &Self, rhs: &Self| -> bool {
                return lhs == rhs;
            })(self, other)
        }
    }

    impl From<i64> for QJsonValue {
        fn from(value: i64) -> Self {
            cpp_fn!(|value: i64| -> Self {
                return QJsonValue(static_cast<qint64>(value));
            })(value)
        }
    }

    impl From<&i64> for QJsonValue {
        fn from(value: &i64) -> Self {
            cpp_fn!(|value: &i64| -> Self {
                return QJsonValue(static_cast<qint64>(value));
            })(value)
        }
    }

    #[instantiate_for[bool, f64]]
    impl<T> From<T> for QJsonValue {
        fn from(value: T) -> Self {
            Self::from(&value)
        }
    }

    #[instantiate_for[bool, f64, QString, QJsonArray, QJsonObject]]
    impl<T> From<&T> for QJsonValue {
        fn from(value: &T) -> Self {
            cpp_fn!(|value: &T| -> Self {
                return QJsonValue(value);
            })(value)
        }
    }
}
