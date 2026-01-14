// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::mem::MaybeUninit;
use crate::QObject;

#[qt_gen::bridge]
mod qvariant {

    include_in_cpp!(<QVariant>);
    include_in_cpp!("rustconv.h");

    #[derive_cpp(Default, Drop, Clone)]
    #[qmetatype(id=41)]
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
    /// # use qt_type_lib::QVariant;
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

    impl ToString for QVariant {
        fn to_string(&self) -> String {
            let conv_fn = cpp_fn!(|&self| -> String {
                return QStringToRustString(self.toString());
            });
            conv_fn(self)
        }
    }

    impl From<&str> for QVariant {
        fn from(value: &str) -> Self {
            let conv_fn = cpp_fn!(|from: &str| -> Self {
                return QVariant(RustStrToQString(from));
            });
            conv_fn(value)
        }
    }

    impl From<&String> for QVariant {
        fn from(value: &String) -> Self {
            QVariant::from(value.as_str())
        }
    }

    impl From<String> for QVariant {
        fn from(value: String) -> Self {
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

                return QVariant(std::move(sl));
            });
            conv_fn(value)
        }
    }

    impl From<*mut QObject> for QVariant {
        fn from(value: *mut QObject) -> Self {
            let conv_fn = cpp_fn!(|from: *mut QObject| -> Self {
                return QVariant::fromValue(from);
            });
            unsafe { conv_fn(value) }
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
    #[instantiate_for[bool, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64]]
    impl<T> From<&T> for QVariant {
        fn from(value: &T) -> Self {
            cpp_fn!(|value: &T| -> Self {
                return QVariant::fromValue(value);
            })(value)
        }
    }

    #[instantiate_for[bool, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64]]
    impl<T> From<T> for QVariant {
        fn from(value: T) -> Self {
            QVariant::from(&value)
        }
    }

    #[instantiate_for[bool, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64]]
    impl<T> TryFrom<&QVariant> for T {
        type Error = ();

        fn try_from(value: &QVariant) -> Result<Self, ()> {
            let convert_fn = cpp_fn!(|from: &QVariant, result: &mut T| -> bool {
                if (!from.canConvert<T>())
                    return false;

                result = from.value<T>();
                return true;
            });

            let mut x = T::default();
            match convert_fn(value, &mut x) {
                true => Ok(x),
                false => Err(()),
            }
        }
    }

    #[instantiate_for[bool, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64]]
    impl<T> TryFrom<QVariant> for T {
        type Error = ();

        fn try_from(value: QVariant) -> Result<Self, ()> {
            Self::try_from(&value)
        }
    }
}
