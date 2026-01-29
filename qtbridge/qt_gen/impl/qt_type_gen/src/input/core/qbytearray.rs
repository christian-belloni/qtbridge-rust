// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::mem::MaybeUninit;

#[qt_gen::bridge]
mod qbytearray {
    include_in_cpp!(<QByteArray>);
    include_in_cpp!("rustconv.h");

    #[derive_cpp(Default, Clone, Drop)]
    #[derive(Debug)]
    #[qmetatype = 12]
    /// The QByteArray struct provides an array of bytes.
    ///
    /// See also: [QByteArray documentation](https://doc.qt.io/qt-6/qbytearray.html).
    struct QByteArray {
        _d: MaybeUninit<[usize; 3]>,
    }

    /// Converts a string slice to a byte slice.
    /// # Examples
    /// ```
    /// # use qt_type_lib::QByteArray;
    /// let ba = QByteArray::from("abc");
    /// assert_eq!(ba.as_bytes(), b"abc");
    /// ```
    pub fn as_bytes(&self) -> &[u8] {
        let mut ptr = std::ptr::null();
        let mut size = 0isize;
        cpp_fn!(|&self, ptr: &mut *const u8, size: &mut isize| {
            ptr = reinterpret_cast<const uint8_t*>(self.data());
            size = self.size();
        })(self, &mut ptr, &mut size);
        unsafe { std::slice::from_raw_parts(ptr, size as usize) }
    }

    /// Returns the number of bytes in this byte array.
    /// The last byte in the byte array is at position size() - 1.
    /// In addition, QByteArray ensures that the byte at position size() is always '\0',
    /// /// # Examples
    /// ```
    /// # use qt_type_lib::QByteArray;
    /// let ba = QByteArray::from("Hello");
    /// assert_eq!(ba[0], b'H');
    /// assert_eq!(ba[4], b'o');
    /// assert_eq!(ba[5], b'\0');
    /// ```
    pub fn size(&self) -> usize {
        let cpp = cpp_fn!(|&self| -> isize {
            return self.size();
        });
        cpp(self) as usize
    }

    impl From<&str> for QByteArray {
        fn from(value: &str) -> Self {
            cpp_fn!(|value: &str| -> Self {
                return RustStrToQByteArray(value);
            })(value)
        }
    }

    impl From<&String> for QByteArray {
        fn from(value: &String) -> Self {
            QByteArray::from(value.as_str())
        }
    }

    // TODO: implement TryFrom instead?
    impl From<&QByteArray> for String {
        fn from(value: &QByteArray) -> Self {
            let str;
            unsafe {
                str = str::from_utf8_unchecked(value.as_bytes())
            }
            String::from(str)
        }
    }

    impl From<&[u8]> for QByteArray {
        fn from(value: &[u8]) -> Self {
            cpp_fn!(|value: &[u8]| -> Self {
                return RustByteSliceToQByteArray(value);
            })(value)
        }
    }

    impl std::ops::Index<usize> for QByteArray {
        type Output = u8;

        fn index(&self, index: usize) -> &u8 {
            let cpp = cpp_fn!(|&self, index: usize| -> * const u8 {
                return reinterpret_cast<const uint8_t*>(self.data() + index);
            });
            unsafe {
                &*cpp(self, index)
            }
        }
    }

    impl PartialEq for QByteArray {
        fn eq(&self, other: &Self) -> bool {
            cpp_fn!(|lhs: &Self, rhs: &Self| -> bool {
                return lhs == rhs;
            })(self, other)
        }
    }

    impl PartialEq<&str> for QByteArray {
        fn eq(&self, other: &&str) -> bool {
            let lhs_bytes = self.as_bytes();
            let rhs_bytes = other.as_bytes();
            lhs_bytes == rhs_bytes
        }
    }
}
