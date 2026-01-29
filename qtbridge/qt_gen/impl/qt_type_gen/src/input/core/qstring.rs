// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::mem::MaybeUninit;
use crate::QByteArray;

#[qt_gen::bridge]
mod qstring {
    include_in_cpp!(<QString>);
    include_in_cpp!("rustconv.h");

    #[derive_cpp(Default, Drop, Clone)]
    #[derive(Debug)]
    #[qmetatype = 10]
    /// The QString struct provides a Unicode character string.
    ///
    /// QString stores a string of 16-bit char items, where each item corresponds to one UTF-16 code unit.
    ///
    /// See also: [QString documentation](https://doc.qt.io/qt-6/qstring.html).
    struct QString {
        // DataPointer d; (2 pointers and size)
        _content: MaybeUninit<[usize; 3]>,
    }

    /// Create new string from the input &str.
    /// # Examples
    /// ```
    /// # use qt_type_lib::QString;
    /// let st = QString::new("abc");
    /// assert_eq!(st, "abc");
    /// ```
    pub fn new(src: &str) -> Self {
        let cpp = cpp_fn!(|src: &str| -> Self {
            return RustStrToQString(src);
        });
        cpp(src)
    }

    /// Returns a UTF-8 representation of the string as a [QByteArray].
    /// # Examples
    /// ```
    /// # use qt_type_lib::QString;
    /// let st = QString::new("Hello world!");
    /// let utf8 = st.to_utf8();
    /// assert_eq!(utf8[6], b'w');
    /// assert_eq!(utf8[8], b'r');
    /// assert_eq!(utf8[10], b'd');
    /// ```
    pub fn to_utf8(&self) -> QByteArray {
        cpp_fn!(|&self| -> QByteArray {
            return self.toUtf8();
        })(self)
    }

    impl PartialEq for QString {
        fn eq(&self, other: &Self) -> bool {
            cpp_fn!(|lhs: &Self, rhs: &Self| -> bool {
                return lhs == rhs;
            })(self, other)
        }
    }
}

impl From<&str> for QString {
    fn from(value: &str) -> Self {
        QString::new(value)
    }
}

impl From<&String> for QString {
    fn from(value: &String) -> Self {
        QString::new(&value)
    }
}

impl From<&QString> for String {
    fn from(value: &QString) -> Self {
        String::from(&value.to_utf8())
    }
}

impl PartialEq<&str> for QString {
    fn eq(&self, other: &&str) -> bool {
        // TODO: compare without allocations?
        let rhs = QString::new(other);
        *self == rhs
    }
}
