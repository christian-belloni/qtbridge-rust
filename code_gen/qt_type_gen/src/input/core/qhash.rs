// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::QList;

#[qt_gen::bridge]
mod qhash {
    include_in_cpp!(<QHash>);

    #[instantiate_for[
        (i32, QByteArray),
        (QByteArray, QVariant),
        ((QString, QVariant), alias = QVariantHash, qmetatype = 28),
    ]]
    #[derive(Debug)]
    #[derive_cpp(Default, Clone, Drop)]
    /// The QHash is a generic struct that provides a hash-table-based dictionary.
    ///
    /// The following types are currently supported as entries in a QHash:
    /// * [(i32, QByteArray)][crate::QHash_i32_QByteArray]
    /// * [(QByteArray, QVariant)][crate::QHash_QByteArray_QVariant]
    /// * [(QString, QVariant)][crate::QHash_QString_QVariant]
    ///
    /// See also: [QHash documentation](https://doc.qt.io/qt-6/qhash.html).
    struct QHash<K, V> {
        _d_ptr: std::mem::MaybeUninit<usize>,
    }

    /// Removes all items from the QHash and frees up all memory used by it.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::QHash;
    /// let mut qhash = QHash::from([
    ///     (1, "One"),
    ///     (2, "Two"),
    /// ]);
    /// assert!(!qhash.is_empty());
    /// qhash.clear();
    /// assert!(qhash.is_empty());
    /// ```
    pub fn clear(&mut self) {
        cpp_fn!(|&mut self| {
            self.clear();
        })(self)
    }

    /// Returns true if the QHash object contains an item with the key; otherwise returns false.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QByteArray, QHash};
    /// let qhash = QHash::from([
    ///     (10, "ten"),
    ///     (20, "twenty"),
    ///     (30, "thirty"),
    /// ]);
    /// assert!(qhash.contains(&20));
    /// assert!(!qhash.contains(&40));
    /// ```
    pub fn contains(&self, key: &K) -> bool {
        cpp_fn!(|&self, key: &K| -> bool {
            return self.contains(key);
        })(self, key)
    }

    /// Inserts a key-value pair into the map.
    /// If the map has this key present, the value is updated with the one from the argument.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QByteArray, QHash};
    /// let mut qhash = QHash::<i32, QByteArray>::default();
    /// qhash.insert(&42, &"abc".into());
    /// assert_eq!(qhash[&42], "abc");
    /// ```
    pub fn insert(&mut self, key: &K, value: &V) {
        let cpp = cpp_fn!(|&mut self, key: &K, value: &V| {
            self.insert(key, value);
        });
        cpp(self, key, value)
    }

    /// # Returns true if the QHash object contains no items; otherwise returns false.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QByteArray, QHash};
    /// let mut qhash = QHash::<i32, QByteArray>::default();
    /// assert!(qhash.is_empty());
    /// qhash.insert(&93, &"c".into());
    /// assert!(!qhash.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        cpp_fn!(|&self| -> bool {
            return self.isEmpty();
        })(self)
    }

    /// Removes the entry that has specified key from the QHash object.
    /// Returns true if the key exists in the QHash object and the item has been removed, and false otherwise.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QByteArray, QHash};
    /// let mut qhash = QHash::<i32, QByteArray>::from([
    ///     (1, "One"),
    ///     (2, "Two"),
    ///     (3, "Three"),
    /// ]);
    /// assert!(qhash.remove(&2));
    /// assert!(qhash.contains(&1));
    /// assert!(!qhash.contains(&2));
    /// assert!(qhash.contains(&3));
    /// assert!(!qhash.remove(&5));
    /// ```
    pub fn remove(&mut self, key: &K) -> bool {
        let cpp = cpp_fn!(|&mut self, key: &K| -> bool {
            return self.remove(key);
        });
        cpp(self, key)
    }

    /// Returns the number of items in the QHash object as isize.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QByteArray, QHash};
    /// let mut qhash = QHash::<i32, QByteArray>::default();
    /// assert_eq!(qhash.len(), 0);
    /// qhash.insert(&42, &"Forty two".into());
    /// assert_eq!(qhash.len(), 1);
    /// ```
    pub fn size(&self) -> isize {
        let cpp = cpp_fn!(|&self| -> isize {
            return self.size();
        });
        cpp(self)
    }

    /// Returns the number of items in the QHash object as usize.
    pub fn len(&self) -> usize {
        self.size() as usize
    }

    /// Returns a list containing all the keys in the QHash object, in an arbitrary order.
    /// The order is guaranteed to be the same as that used by values().
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QByteArray, QHash};
    /// let qhash = QHash::<i32, QByteArray>::from([
    ///     (2, "Two"),
    ///     (3, "Three"),
    ///     (1, "One"),
    /// ]);
    /// let keys = qhash.keys();
    /// assert!(keys.contains(&1));
    /// assert!(keys.contains(&2));
    /// assert!(keys.contains(&3));
    /// ```
    pub fn keys(&self) -> QList<K> {
        let cpp = cpp_fn!(|&self| -> QList<K> {
            return self.keys();
        });
        cpp(self)
    }

    /// Returns a list containing all the values in the QHash object, in an arbitrary order.
    /// The order is guaranteed to be the same as that used by keys().
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QByteArray, QHash};
    /// let qhash = QHash::<i32, QByteArray>::from([
    ///     (3, "Three"),
    ///     (1, "One"),
    ///     (2, "Two"),
    /// ]);
    /// let values = qhash.values();
    /// assert!(values.contains(&"One".into()));
    /// assert!(values.contains(&"Two".into()));
    /// assert!(values.contains(&"Three".into()));
    /// ```
    pub fn values(&self) -> QList<V> {
        let cpp = cpp_fn!(|&self| -> QList<V> {
            return self.values();
        });
        cpp(self)
    }

    /// Returns the value associated with the key.
    /// If the QHash object contains no item with the key, the function returns default-initialized value.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QByteArray, QHash};
    /// let mut qhash = QHash::<i32, QByteArray>::from([
    ///     (1, "a"),
    ///     (2, "b"),
    ///     (3, "c"),
    /// ]);
    /// assert_eq!(qhash.value(&3), "c");
    /// ```
    pub fn value(&self, key: &K) -> V {
        let cpp = cpp_fn!(|&self, key: &K| -> V {
            return self.value(key);
        });
        cpp(self, key)
    }

    impl From<&[(K, V)]> for QHash<K, V> {
        fn from(src: &[(K, V)]) -> Self {
            let mut result = Self::default();
            src.iter()
                .for_each(|(k, v)| result.insert(k, v));
            result
        }
    }

    impl<const N: usize> From<[(K, V); N]> for QHash<K, V> {
        fn from(src: [(K, V); N]) -> Self {
            let mut result = Self::default();
            src.iter()
                .for_each(|(k, v)| result.insert(k, v));
            result
        }
    }

    // TODO: add doc comment to that code when doc comments for trait implementations are supported by qt_type_gen.
    // Something like:
    // Create an instance of QHash initialized with array of (i32, &str) pairs.
    #[include_if_struct_instantiation[(i32, QByteArray)]]
    impl<const N: usize> From<[(K, &str); N]> for QHash<K, V> {
        fn from(src: [(K, &str); N]) -> Self {
            let mut result = Self::default();
            src.iter()
                .for_each(|(k, v)| result.insert(k, &(*v).into()));
            result
        }
    }

    impl std::ops::Index<&K> for QHash<K, V> {
        type Output = V;

        fn index(&self, index: &K) -> &Self::Output {
            let cpp = cpp_fn!(|&self, key: &K| -> *const V {
                auto findIt = self.find(key);
                if (findIt == self.end())
                    return nullptr;

                return &*findIt;
            });

            unsafe {
                cpp(self, index).as_ref()
            }
            .expect("Given key does not exist in QHash")
        }
    }
}
