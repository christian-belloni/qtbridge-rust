// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::mem::MaybeUninit;
use crate::{QByteArray, QJsonArray, QJsonObject, QJsonValue, QList, QMetaType, QMetaTypeGet, QObject, QString};

#[qt_gen::bridge]
mod qvariant {

    include_in_cpp!(<QJsonValue>);
    include_in_cpp!(<QList>);
    include_in_cpp!(<QVariant>);
    include_in_cpp!("rustconv.h");

    #[derive_cpp(Default, Drop, Clone)]
    #[qmetatype = 41]
    /// The QVariant struct acts like an enum for the most common Qt data types.
    ///
    /// QVariant represents dynamically typed value container.
    /// It holds a value of an arbitrary supported type and allows
    /// runtime type inspection and safe conversion from/to compatible types.
    ///
    /// In the context of qtbridge, `QVariant` is primarily used for data exchange between the Rust backend and the Qml Engine.
    /// This kind of data transfer occur in the following areas of implementation internals:
    /// * item models
    /// * exposing user-defined structures to Qml via signals/slots/properties
    ///
    /// In the user code, you may encounter `QVariant` when using [QQmlApplicationEngine::set_initial_properties][crate::QQmlApplicationEngine::set_initial_properties].
    ///
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::QVariant;
    /// let var = QVariant::from("123");
    /// let converted: i32 = var.try_into()
    ///     .expect("Conversion failed");
    /// assert_eq!(converted, 123);
    /// ```
    ///
    /// See also: [QVariant documentation](https://doc.qt.io/qt-6/qvariant.html).
    struct QVariant {
        _content: MaybeUninit<[u8; 32]>,
    }

    /// Returns `true` if this object holds some value or false otherwise.
    ///
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::QVariant;
    /// let var_default = QVariant::default();
    /// assert!(!var_default.is_valid());
    /// let var_int = QVariant::from(42);
    /// assert!(var_int.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        let cpp = cpp_fn!(|&self| -> bool {
            return self.isValid();
        });
        cpp(self)
    }

    /// Returns the [QMetaType] of the value stored in the variant.
    ///
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QMetaTypeGet, QVariant};
    /// let var = QVariant::from(0.5f64);
    /// assert_eq!(var.meta_type().id(), f64::get_qmetatype().id());
    /// ```
    pub fn meta_type(&self) -> QMetaType {
        let cpp = cpp_fn!(|&self| -> QMetaType {
            return self.metaType();
        });
        cpp(self)
    }

    impl ToString for QVariant {
        fn to_string(&self) -> String {
            let conv_fn = cpp_fn!(|&self| -> String {
                return QStringToRustString(self.toString());
            });
            conv_fn(self)
        }
    }

    impl From<&()> for QVariant {
        fn from(_: &()) -> Self {
            QVariant::default()
        }
    }

    impl From<&str> for QVariant {
        fn from(value: &str) -> Self {
            let conv_fn = cpp_fn!(|from: &str| -> Self {
                return QVariant::fromValue(RustStrToQString(from));
            });
            conv_fn(value)
        }
    }

    impl From<&String> for QVariant {
        fn from(value: &String) -> Self {
            QVariant::from(value.as_str())
        }
    }

    impl From<&Vec<String>> for QVariant {
        fn from(value: &Vec<String>) -> Self {
            let conv_fn = cpp_fn!(|from: &Vec<String>| -> Self {
                QStringList sl;
                sl.reserve(from.size());
                for (const auto& s : from)
                    sl.push_back(RustStrToQString(s));

                return QVariant::fromValue(std::move(sl));
            });
            conv_fn(value)
        }
    }

    impl TryFrom<&QVariant> for () {
        type Error = ();
        fn try_from(value: &QVariant) -> Result<Self, Self::Error> {
            match value.is_valid() {
                true => Err(()),
                false => Ok(()),
            }
        }
    }

    impl TryFrom<&QVariant> for String {
        type Error = ();

        fn try_from(value: &QVariant) -> Result<Self, Self::Error> {
            let conv_fn = cpp_fn!(|from: &QVariant, result: &mut String| -> bool {
                if (!from.canConvert<QString>())
                    return false;

                QString s = from.value<QString>();
                result = QStringToRustString(s);
                return true;
            });

            let mut result = String::new();
            match conv_fn(value, &mut result) {
                true => Ok(result),
                false => Err(()),
            }
        }
    }

    impl TryFrom<&QVariant> for Vec<String> {
        type Error = ();

        fn try_from(value: &QVariant) -> Result<Self, Self::Error> {
            let conv_fn = cpp_fn!(|from: &QVariant, result: &mut Vec<String>| -> bool {
                if (!from.canConvert<QStringList>())
                    return false;

                QStringList sl = from.value<QStringList>();
                result = QStringListToRustStringList(sl);
                return true;
            });

            let mut result = Vec::<String>::new();
            match conv_fn(value, &mut result) {
                true => Ok(result),
                false => Err(()),
            }
        }
    }

    // TODO: consider reusing QMetaType for conversions instead of this
    #[instantiate_for[bool, i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64, *mut QObject]]
    impl<T> From<&T> for QVariant {
        fn from(value: &T) -> Self {
            cpp_fn!(|value: &T| -> Self {
                return QVariant::fromValue(value);
            })(value)
        }
    }

    #[instantiate_for[bool, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, isize, usize, String, Vec<String>, *mut QObject]]
    impl<T> From<T> for QVariant {
        fn from(value: T) -> Self {
            QVariant::from(&value)
        }
    }

    #[instantiate_for[bool, i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64]]
    impl<T> From<&Vec<T>> for QVariant {
        fn from(value: &Vec<T>) -> Self {
            cpp_fn!(|value: &Vec<T>| -> Self {
                QList<T> qlist(value.cbegin(), value.cend());
                return QVariant::fromValue(qlist);
            })(value)
        }
    }

    #[instantiate_for[QByteArray, QString]]
    impl<T> From<&Vec<T>> for QVariant {
        fn from(value: &Vec<T>) -> Self {
            let cpp = cpp_fn!(|list: QList<T>| -> Self {
                return QVariant::fromValue(std::move(list));
            });
            cpp(QList::from(value))
        }
    }

    #[instantiate_for[bool, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, isize, usize, QByteArray, QString]]
    impl<T> From<Vec<T>> for QVariant {
        fn from(value: Vec<T>) -> Self {
            QVariant::from(&value)
        }
    }

    #[instantiate_for[bool, i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64, *mut QObject]]
    impl<T> TryFrom<&QVariant> for T {
        type Error = ();

        fn try_from(value: &QVariant) -> Result<Self, ()> {
            let convert_fn = cpp_fn!(|from: &QVariant, result: &mut T| -> bool {
                if (!from.canConvert<T>())
                    return false;

                result = from.value<T>();
                return true;
            });

            let mut x = <T>::default();
            match convert_fn(value, &mut x) {
                true => Ok(x),
                false => Err(()),
            }
        }
    }

    #[instantiate_for[bool, i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64]]
    impl<T> TryFrom<&QVariant> for Vec<T> {
        type Error = ();

        fn try_from(value: &QVariant) -> Result<Self, ()> {
            let convert_fn = cpp_fn!(|from: &QVariant, result: &mut QList<T>| -> bool {
                if (!from.canConvert<QList<T>>())
                    return false;

                result = from.value<QList<T>>();
                return true;

            });

            let mut x = QList::default();
            match convert_fn(value, &mut x) {
                true => Ok(x.into()),
                false => Err(())
            }
        }
    }

    #[instantiate_for[bool, i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64, String, *mut QObject]]
    impl<T> TryFrom<QVariant> for T {
        type Error = ();
        fn try_from(value: QVariant) -> Result<Self, ()> {
            Self::try_from(&value)
        }
    }

    #[instantiate_for[bool, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, isize, usize, String]]
    impl<T> TryFrom<QVariant> for Vec<T> {
        type Error = ();
        fn try_from(value: QVariant) -> Result<Self, ()> {
            Self::try_from(&value)
        }
    }

    impl From<&QJsonArray> for QVariant {
        fn from(value: &QJsonArray) -> QVariant {
            cpp_fn!(|value: &QJsonArray| -> QVariant {
                return QVariant::fromValue(value);
            })(value)
        }
    }

    impl TryFrom<&QVariant> for QJsonArray {
        type Error = ();
        fn try_from(value: &QVariant) -> Result<Self, ()> {
            let conv_fn = cpp_fn!(|from: &QVariant, result: &mut QJsonArray| -> bool {
                if (from.metaType() != QMetaType::fromType<QJsonArray>())
                    return false;
                result = from.value<QJsonArray>();
                return true;
            });
            let mut result = QJsonArray::default();
            match conv_fn(value, &mut result) {
                true => Ok(result),
                false => Err(()),
            }
        }
    }

    impl From<&QJsonObject> for QVariant {
        fn from(value: &QJsonObject) -> QVariant {
            cpp_fn!(|value: &QJsonObject| -> QVariant {
                return QVariant::fromValue(value);
            })(value)
        }
    }

    impl TryFrom<&QVariant> for QJsonObject {
        type Error = ();
        fn try_from(value: &QVariant) -> Result<Self, ()> {
            let conv_fn = cpp_fn!(|from: &QVariant, result: &mut QJsonObject| -> bool {
                if (from.metaType() != QMetaType::fromType<QJsonObject>())
                    return false;
                result = from.value<QJsonObject>();
                return true;
            });
            let mut result = QJsonObject::default();
            match conv_fn(value, &mut result) {
                true => Ok(result),
                false => Err(()),
            }
        }
    }

    /// Wraps this value in a `QVariant` with metatype `QMetaType::QJsonValue` (id 45).
    impl From<&QJsonValue> for QVariant {
        fn from(value: &QJsonValue) -> QVariant {
            cpp_fn!(|value: &QJsonValue| -> QVariant {
                return QVariant::fromValue(value);
            })(value)
        }
    }

    /// Extracts a `QJsonValue` from a `QVariant` whose metatype is `QMetaType::QJsonValue`.
    impl TryFrom<&QVariant> for QJsonValue {
        type Error = ();
        fn try_from(value: &QVariant) -> Result<Self, ()> {
            let conv_fn = cpp_fn!(|from: &QVariant, result: &mut QJsonValue| -> bool {
                if (from.metaType() != QMetaType::fromType<QJsonValue>())
                    return false;
                result = from.value<QJsonValue>();
                return true;
            });
            let mut result = QJsonValue::default();
            match conv_fn(value, &mut result) {
                true => Ok(result),
                false => Err(()),
            }
        }
    }
}

/// Returns `true` if the variant currently holds a value of Qt metatype `T`.
impl QVariant {
    pub fn is_type<T: QMetaTypeGet>(&self) -> bool {
        self.meta_type() == T::get_qmetatype()
    }
}
