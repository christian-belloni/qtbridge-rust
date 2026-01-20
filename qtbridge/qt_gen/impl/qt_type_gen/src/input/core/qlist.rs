// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#[qt_gen::bridge]
mod qlist {
    include_in_cpp!(<QList>);

    #[instantiate_for[
        i8, u8, i16, u16, i32, u32, i64, u64, f32, f64,
        ((QByteArray), alias = QByteArrayList, qmetatype = 49),
        ((QString), alias = QStringList, qmetatype = 11),
        ((QVariant), alias = QVariantList, qmetatype = 9)
        ]]
    #[derive(Debug)]
    #[derive_cpp(Default, Clone, Drop)]
    /// The QList is a generic struct that provides a dynamic array.
    ///
    /// QList is one of Qt's generic container structs. It stores its items in adjacent memory locations and provides fast index-based access.
    ///
    /// The following types are currently supported as items in a QList:
    /// * [i8][crate::QList_i8]
    /// * [u8][crate::QList_u8]
    /// * [i16][crate::QList_i16]
    /// * [u16][crate::QList_u16]
    /// * [i32][crate::QList_i32]
    /// * [u32][crate::QList_u32]
    /// * [i64][crate::QList_i64]
    /// * [u64][crate::QList_u64]
    /// * [f32][crate::QList_f32]
    /// * [f64][crate::QList_f64]
    /// * [QByteArray][crate::QList_QByteArray] (also known as [QByteArrayList][crate::QByteArrayList])
    /// * [QString][crate::QList_QString] (also known as [QStringList][crate::QStringList])
    /// * [QVariant][crate::QList_QVariant] (also known as [QVariantList][crate::QVariantList])
    ///
    /// See also [QList documentation](https://doc.qt.io/qt-6/qlist.html).
    struct QList<T> {
        // fields of QArrayDataPointer<T>:
        _d: std::mem::MaybeUninit<usize>,
        _ptr: std::mem::MaybeUninit<usize>,
        _size: std::mem::MaybeUninit<usize>,
    }

    /// Inserts value at the end of the list.
    /// # Examples
    /// ```
    /// # use qt_type_lib::QList;
    /// let mut list = QList::default();
    /// list.append(1);
    /// list.append(2);
    /// let three: i32 = 3;
    /// list.append(three);
    /// assert_eq!(list, [1, 2, 3]);
    /// ```
    pub fn append(&mut self, value: T) {
        let cpp = cpp_fn!(|&mut self, value: T| {
            self.append(value);
        });
        cpp(self, value);
    }

    /// Returns the maximum number of items that can be stored in the list without forcing a reallocation.
    /// # Examples
    /// ```
    /// # use qt_type_lib::QList;
    /// let mut list = QList::default();
    /// assert_eq!(0, list.capacity());
    /// list.append(1);
    /// assert!(1 <= list.capacity());
    /// list.reserve(100);
    /// assert_eq!(list.capacity(), 100);
    /// ```
    pub fn capacity(&self) -> usize {
        let cpp = cpp_fn!(|&self| -> usize {
            return self.capacity();
        });
        cpp(self)
    }

    /// Removes all the elements from the list.
    /// # Examples
    /// ```
    /// # use qt_type_lib::QList;
    /// let mut list = QList::from([1, 2, 3]);
    /// assert!(!list.is_empty());
    /// list.clear();
    /// assert!(list.is_empty());
    /// ```
    pub fn clear(&mut self) {
        let cpp = cpp_fn!(|&mut self| {
            self.clear();
        });
        cpp(self);
    }

    /// Returns true if the list contains an occurrence of value; otherwise returns false.
    /// # Examples
    /// ```
    /// # use qt_type_lib::QList;
    /// let list = QList::from([10, 20, 30]);
    /// assert!(list.contains(&20));
    /// assert!(!list.contains(&40));
    /// ```
    pub fn contains(&self, value: &T) -> bool {
        cpp_fn!(|&self, value: &T| -> bool {
            return self.contains(value);
        })(self, value)
    }

    /// Returns true if the list has size 0; otherwise returns false.
    /// # Examples
    /// ```
    /// # use qt_type_lib::QList;
    /// let mut list = QList::default();
    /// assert!(list.is_empty());
    ///
    /// list.append(1);
    /// assert!(!list.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    /// Inserts value at the end of the list.
    /// # Examples
    /// ```
    /// # use qt_type_lib::QList;
    /// let mut list = QList::from([1, 2, 3]);
    /// list.push_back(4);
    /// assert_eq!(list, [1, 2, 3, 4]);
    /// ```
    pub fn push_back(&mut self, value: T) {
        let cpp = cpp_fn!(|&mut self, value: T| {
            self.push_back(value);
        });
        cpp(self, value);
    }

    /// Removes n elements from the list, starting at index position i.
    /// # Examples
    /// ```
    /// # use qt_type_lib::QList;
    /// let mut list = QList::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// list.remove(3, 4);
    /// assert_eq!(list, [1, 2, 3, 8, 9]);
    /// ```
    pub fn remove(&mut self, i: isize, n: isize) {
        let cpp = cpp_fn!(|&mut self, i: isize, n: isize| {
            self.remove(i, n);
        });
        cpp(self, i, n);
    }

    /// Attempts to allocate memory for at least size elements.
    /// # Examples
    /// ```
    /// # use qt_type_lib::QList;
    /// let mut list = QList::<i32>::default();
    /// list.reserve(100);
    /// assert_eq!(list.capacity(), 100);
    /// ```
    pub fn reserve(&mut self, size: usize) {
        let cpp = cpp_fn!(|&mut self, size: usize| {
            self.reserve(static_cast<qsizetype>(size));
        });
        cpp(self, size);
    }

    /// Returns the number of items in the list as isize.
    /// # Examples
    /// ```
    /// # use qt_type_lib::QList;
    /// let list = QList::from([1, 2, 3, 4, 5, 6, 7]);
    /// assert_eq!(list.len(), 7);
    /// ```
    pub fn size(&self) -> isize {
        let cpp = cpp_fn!(|&self| -> isize {
            return self.size();
        });
        cpp(self)
    }

    /// Returns the number of items in the list as usize.
    /// # Examples
    /// ```
    /// # use qt_type_lib::QList;
    /// let list = QList::from([1, 2, 3, 4, 5, 6, 7, 8]);
    /// assert_eq!(list.len(), 8);
    /// ```
    pub fn len(&self) -> usize {
        self.size() as usize
    }

    /// Returns a const reference to the first item in the list. This function assumes that the list isn't empty.
    /// # Examples
    /// ```
    /// # use qt_type_lib::QList;
    /// let list = QList::from([1, 2, 3]);
    /// assert_eq!(*list.first(), 1);
    /// ```
    pub fn first(&self) -> &T {
        let cpp = cpp_fn!(|&self| -> &T {
            return self.constFirst();
        });
        cpp(self)
    }

    /// Returns a const reference to the last item in the list. This function assumes that the list isn't empty.
    /// # Examples
    /// ```
    /// # use qt_type_lib::QList;
    /// let list = QList::from([1, 2, 3]);
    /// assert_eq!(*list.last(), 3);
    pub fn last(&self) -> &T {
        let cpp = cpp_fn!(|&self| -> &T {
            return self.constLast();
        });
        cpp(self)
    }

    #[exclude_if_struct_instantiation[QByteArray, QString, QVariant]]
    impl From<&[T]> for QList<T> {
        fn from(value: &[T]) -> Self {
            let mut result = Self::default();
            result.reserve(value.len());
            for item in value.iter() {
                result.append(*item);
            }
            result
        }
    }

    #[include_if_struct_instantiation[QByteArray, QString, QVariant]]
    impl From<&[T]> for QList<T> {
        fn from(value: &[T]) -> Self {
            let mut result = Self::default();
            result.reserve(value.len());
            for item in value.iter() {
                result.append(item.clone());
            }
            result
        }
    }

    impl<const N: usize> From<[T; N]> for QList<T> {
        fn from(value: [T; N]) -> Self {
            let mut result = Self::default();
            result.reserve(N);
            for item in value.into_iter() {
                result.append(item);
            }
            result
        }
    }

    #[exclude_if_struct_instantiation[QByteArray, QString, QVariant]]
    impl From<&Vec<T>> for QList<T> {
        fn from(value: &Vec<T>) -> Self {
            let cpp = cpp_fn!(|src: &Vec<T>| -> Self {
                return QList<T>(src.cbegin(), src.cend());
            });
            cpp(value)
        }
    }

    #[exclude_if_struct_instantiation[QByteArray, QString, QVariant]]
    impl From<&QList<T>> for Vec<T> {
        fn from(value: &QList<T>) -> Self {
            let cpp = cpp_fn!(|src: &QList<T>| -> Self {
                rust::Vec<T> result;
                result.reserve(static_cast<size_t>(src.size()));
                for (T item: src)
                    result.push_back(item);
                return result;
            });
            cpp(value)
        }
    }

    impl std::ops::Index<usize> for QList<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            let cpp = cpp_fn!(|&self, index: usize| -> *const T {
                return index < static_cast<size_t>(self.size()) ? &self[index] : nullptr;
            });

            unsafe {
                cpp(self, index).as_ref()
            }
            .expect("Out of bounds access to QList")
        }
    }

    impl PartialEq for QList<T> {
        fn eq(&self, other: &Self) -> bool {
            let cpp = cpp_fn!(|lhs: &Self, rhs: &Self| -> bool {
                return lhs == rhs;
            });
            cpp(self, other)
        }
    }

    impl<const N: usize> PartialEq<[T; N]> for QList<T> {
        fn eq(&self, other: &[T; N]) -> bool {
            if self.len() != N {
                return false;
            }

            let cpp = cpp_fn!(|&self, rhs: &[T]| -> bool {
                for (size_t i = 0; i < rhs.size(); ++i) {
                    if (self[i] != rhs[i])
                        return false;
                }
                return true;
            });
            cpp(self, other)
        }
    }
}
