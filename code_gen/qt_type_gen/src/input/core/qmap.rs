// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::QList;

#[qt_gen::bridge]
mod qmap {
    include_in_cpp!(<QMap>);

    #[instantiate_for[
        (i32, QString),
        ((QString, QVariant), alias = QVariantMap, qmetatype = 8),
    ]]
    #[derive(Debug)]
    #[derive_cpp(Default, Clone, Drop)]
    /// The QMap is a generic struct that provides an associative array.
    ///
    /// The following types are currently supported as entries in a QMap:
    /// * [(i32, QString)][crate::QMap_i32_QString]
    /// * [(QString, QVariant)][crate::QMap_QString_QVariant] (also known as [QVariantMap][crate::QVariantMap])
    ///
    /// See also: [QMap documentation](https://doc.qt.io/qt-6/qmap.html).
    struct QMap<K, V> {
        // QExplicitlySharedDataPointerV2
        _d: std::mem::MaybeUninit<usize>,
    }

    /// Removes all the items from the map.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::QMap;
    /// let mut map = QMap::from([
    ///     (1, "One"),
    ///     (2, "Two"),
    /// ]);
    /// assert!(!map.is_empty());
    /// map.clear();
    /// assert!(map.is_empty());
    /// ```
    pub fn clear(&mut self) {
        cpp_fn!(|&mut self| {
            self.clear();
        })(self)
    }

    /// Inserts a new entry with the given key and value into the map.
    /// If an entry with the same key already exists, its value is replaced with the one provided in the argument.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QMap, QString};
    /// let mut map = QMap::<i32, QString>::default();
    /// map.insert(&91, &"a".into());
    /// assert_eq!(map[&91], "a");
    /// ```
    pub fn insert(&mut self, key: &K, value: &V) {
        let cpp = cpp_fn!(|&mut self, key: &K, value: &V| {
            self.insert(key, value);
        });
        cpp(self, key, value)
    }

    /// Returns true if the map contains no items; otherwise returns false.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QMap, QString};
    /// let mut map = QMap::<i32, QString>::default();
    /// assert!(map.is_empty());
    /// map.insert(&92, &"b".into());
    /// assert!(!map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        cpp_fn!(|&self| -> bool {
            return self.isEmpty();
        })(self)
    }

    /// Remove the entry with the given key from the map.
    /// Returns 1 if the key existed in the map, and 0 otherwise.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QMap, QString};
    /// let mut map = QMap::<i32, QString>::from([
    ///     (1, "One"),
    ///     (2, "Two"),
    ///     (3, "Three"),
    ///     (4, "Four"),
    /// ]);
    /// assert_eq!(map.remove(&2), 1);
    /// assert_eq!(map.keys(), [1, 3, 4])
    /// ```
    pub fn remove(&mut self, key: &K) -> i32 {
        let cpp = cpp_fn!(|&mut self, key: &K| -> i32 {
            return self.remove(key);
        });
        cpp(self, key)
    }

    /// Returns the number of (key, value) pairs in the map as i32.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QMap, QString};
    /// let mut map = QMap::<i32, QString>::default();
    /// assert_eq!(map.len(), 0);
    /// map.insert(&42, &"Forty two".into());
    /// assert_eq!(map.len(), 1);
    /// ```
    pub fn size(&self) -> i32 {
        cpp_fn!(|&self| -> i32 {
            return self.size();
        })(self)
    }

    /// Returns the number of (key, value) pairs in the map as usize.
    /// See example of QMap<K, V>::size().
    pub fn len(&self) -> usize {
        self.size() as usize
    }

    /// Returns a list containing all the keys in the map in ascending order.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QMap, QString};
    /// let map = QMap::<i32, QString>::from([
    ///     (2, "Two"),
    ///     (3, "Three"),
    ///     (1, "One"),
    /// ]);
    /// assert_eq!(map.keys(), [1, 2, 3]);
    /// ```
    pub fn keys(&self) -> QList<K> {
        let cpp = cpp_fn!(|&self| -> QList<K> {
            return self.keys();
        });
        cpp(self)
    }

    /// Returns a list containing all the values in the map, in ascending order of their keys.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QMap, QString};
    /// let map = QMap::<i32, QString>::from([
    ///     (3, "Three"),
    ///     (1, "One"),
    ///     (2, "Two"),
    /// ]);
    /// assert_eq!(map.values(), [QString::from("One"), QString::from("Two"), QString::from("Three")]);
    /// ```
    pub fn values(&self) -> QList<V> {
        let cpp = cpp_fn!(|&self| -> QList<V> {
            return self.values();
        });
        cpp(self)
    }

    /// Returns the value associated with the specified key.
    /// # Examples
    /// ```
    /// # use qtbridge_type_lib::{QMap, QString};
    /// let mut map = QMap::<i32, QString>::from([
    ///     (1, "a"),
    ///     (2, "b"),
    ///     (3, "c"),
    /// ]);
    /// assert_eq!(map.value(&3), "c");
    /// ```
    pub fn value(&self, key: &K) -> V {
        let cpp = cpp_fn!(|&self, key: &K| -> V {
            return self.value(key);
        });
        cpp(self, key)
    }

    impl From<(K, V)> for QMap<K, V> {
        fn from(src: (K, V)) -> Self {
            let mut result = Self::default();
            result.insert(&src.0, &src.1);
            result
        }
    }

    impl From<&[(K, V)]> for QMap<K, V> {
        fn from(src: &[(K, V)]) -> Self {
            let mut result = Self::default();
            src.iter()
                .for_each(|(k, v)| result.insert(k, v));
            result
        }
    }

    #[include_if_struct_instantiation[(QString, QVariant)]]
    impl From<(&str, V)> for QMap<K, V> {
        fn from(src: (&str, V)) -> Self {
            let mut result = Self::default();
            result.insert(&QString::from(src.0), &src.1);
            result
        }
    }

    #[include_if_struct_instantiation[(QString, QVariant)]]
    impl From<&[(&str, V)]> for QMap<K, V> {
        fn from(src: &[(&str, V)]) -> Self {
            let mut result = Self::default();
            src.iter()
                .for_each(|(k, v)| result.insert(&QString::from(*k), v));
            result
        }
    }

    #[include_if_struct_instantiation[(QString, QVariant)]]
    impl<const N: usize> From<[(&str, V); N]> for QMap<K, V> {
        fn from(src: [(&str, V); N]) -> Self {
            let mut result = Self::default();
            src.iter()
                .for_each(|(k, v)| result.insert(&QString::from(*k), v));
            result
        }
    }

    #[include_if_struct_instantiation[(i32, QString)]]
    impl<const N: usize> From<[(i32, &str); N]> for QMap<K, V> {
        fn from(src: [(i32, &str); N]) -> Self {
            let mut result = Self::default();
            src.into_iter()
                .for_each(|(k, v)| result.insert(&k, &QString::from(v)));
            result
        }
    }

    impl<const N: usize> From<[(K, V); N]> for QMap<K, V> {
        fn from(src: [(K, V); N]) -> Self {
            let mut result = Self::default();
            src.iter()
                .for_each(|(k, v)| result.insert(k, v));
            result
        }
    }

    impl std::ops::Index<&K> for QMap<K, V> {
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
            .expect("Given key does not exist in QMap")
        }
    }
}
