use qt_gen::qobject_impl_internal;
use qt_type_lib::{QByteArray, QHash, QModelIndex, QVariant};

use std::fmt;
use std::ops::Index;
use std::collections::HashMap;
use std::slice::SliceIndex;

/// Trait representing a single item in a Qt item model.
///
/// This trait is implemented automatically by `#[derive(QModelItem)]`
/// for structs and tuple structs and an implmenetation for primitive types
/// and tuples is provided. It allows a `QVec<T>` to expose fields as Qt
/// to QML views.
///
/// # Typical usage
/// Normally, you should **not implement this manually**. Use
/// `#[derive(QModelItem)]` on your struct:
///
/// ```rust,ignore
/// #[derive(QModelItem)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
/// ```
///
/// # Manual Implementation
/// If your struct contains fields that **cannot be converted to `QVariant`**,
/// you need to implement `QModelItem` manually. Only include fields that
/// can be represented as `QVariant` in the `elemN` and `set_elemN` methods,
/// and adjust the `LEN` constant and `role_names()` accordingly.
///
/// Example:
///
/// ```rust
///
/// use std::collections::HashMap;
/// use qt_type_lib::QVariant;
/// use qt_container::QModelItem;
///
/// struct CustomType {
///     data: Vec<u8>, // cannot be directly converted to QVariant
///     name: String,
/// }
///
/// impl QModelItem for CustomType {
///     const LEN: usize = 1; // only expose one field
///
///     fn elem0(&self) -> QVariant {
///         QVariant::from(&self.name)
///     }
///
///     fn set_elem0(&mut self, value: &QVariant) -> bool {
///         <String>::try_from(value).map(|val| { self.name = val; }).is_ok()
///     }
///
///     fn role_names() -> HashMap<i32, String> {
///         let roles = [(0x100, "name".into())];
///         roles.iter().cloned().collect()
///     }
/// }
/// ```
pub trait QModelItem {
    const LEN: usize;
    fn elem0(&self) -> QVariant { QVariant::default() }
    fn elem1(&self) -> QVariant { QVariant::default() }
    fn elem2(&self) -> QVariant { QVariant::default() }
    fn elem3(&self) -> QVariant { QVariant::default() }
    fn elem4(&self) -> QVariant { QVariant::default() }
    fn elem5(&self) -> QVariant { QVariant::default() }
    fn elem6(&self) -> QVariant { QVariant::default() }
    fn elem7(&self) -> QVariant { QVariant::default() }
    fn elem8(&self) -> QVariant { QVariant::default() }
    fn elem9(&self) -> QVariant { QVariant::default() }
    fn elem10(&self) -> QVariant { QVariant::default() }
    fn elem11(&self) -> QVariant { QVariant::default() }
    fn elem12(&self) -> QVariant { QVariant::default() }
    fn elem13(&self) -> QVariant { QVariant::default() }
    fn elem14(&self) -> QVariant { QVariant::default() }
    fn set_elem0(&mut self, _value: &QVariant) -> bool { false }
    fn set_elem1(&mut self, _value: &QVariant) -> bool { false }
    fn set_elem2(&mut self, _value: &QVariant) -> bool { false }
    fn set_elem3(&mut self, _value: &QVariant) -> bool { false }
    fn set_elem4(&mut self, _value: &QVariant) -> bool { false }
    fn set_elem5(&mut self, _value: &QVariant) -> bool { false }
    fn set_elem6(&mut self, _value: &QVariant) -> bool { false }
    fn set_elem7(&mut self, _value: &QVariant) -> bool { false }
    fn set_elem8(&mut self, _value: &QVariant) -> bool { false }
    fn set_elem9(&mut self, _value: &QVariant) -> bool { false }
    fn set_elem10(&mut self, _value: &QVariant) -> bool { false }
    fn set_elem11(&mut self, _value: &QVariant) -> bool { false }
    fn set_elem12(&mut self, _value: &QVariant) -> bool { false }
    fn set_elem13(&mut self, _value: &QVariant) -> bool { false }
    fn set_elem14(&mut self, _value: &QVariant) -> bool { false }
    fn role_names() -> HashMap<i32, String>;
}

macro_rules! impl_QModelItem_for_primitive {
    ($t:ty) => {
        impl QModelItem for $t {
            const LEN: usize = 1;

            fn elem0(&self) -> QVariant {
                QVariant::from(self)
            }

            fn set_elem0(&mut self, value: &QVariant) -> bool {
                <$t>::try_from(value).map(|val| *self = val).is_ok()
            }

            fn role_names() -> HashMap<i32, String> {
                let roles = [(0x100, "value".into())];
                roles.iter().cloned().collect()
            }
        }
    };
}

impl_QModelItem_for_primitive!(i8);
impl_QModelItem_for_primitive!(i16);
impl_QModelItem_for_primitive!(i32);
impl_QModelItem_for_primitive!(i64);
impl_QModelItem_for_primitive!(u8);
impl_QModelItem_for_primitive!(u16);
impl_QModelItem_for_primitive!(u32);
impl_QModelItem_for_primitive!(u64);
impl_QModelItem_for_primitive!(f32);
impl_QModelItem_for_primitive!(f64);
impl_QModelItem_for_primitive!(bool);
impl_QModelItem_for_primitive!(String);

impl<T0> QModelItem for (T0,)
where
    for<'a> QVariant: From<&'a T0>,
    for<'a> T0: TryFrom<&'a QVariant, Error = ()>,
{
    const LEN: usize = 1;
    fn elem0(&self) -> QVariant { QVariant::from(&self.0) }
    fn set_elem0(&mut self, value: &QVariant) -> bool {
        T0::try_from(value).map(|val| self.0 = val).is_ok()
    }
    fn role_names() -> HashMap<i32, String> {
        let roles = [
            (0x100, "_0".into()),
        ];
        roles.iter().cloned().collect()
    }
}

impl<T0, T1> QModelItem for (T0, T1)
where
    for<'a> QVariant: From<&'a T0>,
    for<'a> T0: TryFrom<&'a QVariant, Error = ()>,
    for<'a> QVariant: From<&'a T1>,
    for<'a> T1: TryFrom<&'a QVariant, Error = ()>,
{
    const LEN: usize = 2;
    fn elem0(&self) -> QVariant { QVariant::from(&self.0) }
    fn elem1(&self) -> QVariant { QVariant::from(&self.1) }
    fn set_elem0(&mut self, value: &QVariant) -> bool {
        T0::try_from(value).map(|val| self.0 = val).is_ok()
    }
    fn set_elem1(&mut self, value: &QVariant) -> bool {
        T1::try_from(value).map(|val| self.1 = val).is_ok()
    }
    fn role_names() -> HashMap<i32, String> {
        let roles= [
            (0x100, "_0".into()),
            (0x101, "_1".into()),
        ];
        roles.iter().cloned().collect()
    }
}

impl<T0, T1, T2> QModelItem for (T0, T1, T2)
where
    for<'a> QVariant: From<&'a T0>,
    for<'a> T0: TryFrom<&'a QVariant, Error = ()>,
    for<'a> QVariant: From<&'a T1>,
    for<'a> T1: TryFrom<&'a QVariant, Error = ()>,
    for<'a> QVariant: From<&'a T2>,
    for<'a> T2: TryFrom<&'a QVariant, Error = ()>,
{
    const LEN: usize = 3;
    fn elem0(&self) -> QVariant { QVariant::from(&self.0) }
    fn elem1(&self) -> QVariant { QVariant::from(&self.1) }
    fn elem2(&self) -> QVariant { QVariant::from(&self.2) }
    fn set_elem0(&mut self, value: &QVariant) -> bool {
        T0::try_from(value).map(|val| self.0 = val).is_ok()
    }
    fn set_elem1(&mut self, value: &QVariant) -> bool {
        T1::try_from(value).map(|val| self.1 = val).is_ok()
    }
    fn set_elem2(&mut self, value: &QVariant) -> bool {
        T2::try_from(value).map(|val| self.2 = val).is_ok()
    }
    fn role_names() -> HashMap<i32, String> {
        let roles= [
            (0x100, "_0".into()),
            (0x101, "_1".into()),
            (0x102, "_2".into()),
        ];
        roles.iter().cloned().collect()
    }
}

impl<T0, T1, T2, T3> QModelItem for (T0, T1, T2, T3)
where
    for<'a> QVariant: From<&'a T0>,
    for<'a> T0: TryFrom<&'a QVariant, Error = ()>,
    for<'a> QVariant: From<&'a T1>,
    for<'a> T1: TryFrom<&'a QVariant, Error = ()>,
    for<'a> QVariant: From<&'a T2>,
    for<'a> T2: TryFrom<&'a QVariant, Error = ()>,
    for<'a> QVariant: From<&'a T3>,
    for<'a> T3: TryFrom<&'a QVariant, Error = ()>,
{
    const LEN: usize = 4;
    fn elem0(&self) -> QVariant { QVariant::from(&self.0) }
    fn elem1(&self) -> QVariant { QVariant::from(&self.1) }
    fn elem2(&self) -> QVariant { QVariant::from(&self.2) }
    fn elem3(&self) -> QVariant { QVariant::from(&self.3) }
    fn set_elem0(&mut self, value: &QVariant) -> bool {
        T0::try_from(value).map(|val| self.0 = val).is_ok()
    }
    fn set_elem1(&mut self, value: &QVariant) -> bool {
        T1::try_from(value).map(|val| self.1 = val).is_ok()
    }
    fn set_elem2(&mut self, value: &QVariant) -> bool {
        T2::try_from(value).map(|val| self.2 = val).is_ok()
    }
    fn set_elem3(&mut self, value: &QVariant) -> bool {
        T3::try_from(value).map(|val| self.3 = val).is_ok()
    }
    fn role_names() -> HashMap<i32, String> {
        let roles= [
            (0x100, "_0".into()),
            (0x101, "_1".into()),
            (0x102, "_2".into()),
            (0x103, "_3".into()),
        ];
        roles.iter().cloned().collect()
    }
}

impl<T0, T1, T2, T3, T4> QModelItem for (T0, T1, T2, T3, T4)
where
    for<'a> QVariant: From<&'a T0>,
    for<'a> T0: TryFrom<&'a QVariant, Error = ()>,
    for<'a> QVariant: From<&'a T1>,
    for<'a> T1: TryFrom<&'a QVariant, Error = ()>,
    for<'a> QVariant: From<&'a T2>,
    for<'a> T2: TryFrom<&'a QVariant, Error = ()>,
    for<'a> QVariant: From<&'a T3>,
    for<'a> T3: TryFrom<&'a QVariant, Error = ()>,
    for<'a> QVariant: From<&'a T4>,
    for<'a> T4: TryFrom<&'a QVariant, Error = ()>,
{
    const LEN: usize = 5;
    fn elem0(&self) -> QVariant { QVariant::from(&self.0) }
    fn elem1(&self) -> QVariant { QVariant::from(&self.1) }
    fn elem2(&self) -> QVariant { QVariant::from(&self.2) }
    fn elem3(&self) -> QVariant { QVariant::from(&self.3) }
    fn elem4(&self) -> QVariant { QVariant::from(&self.4) }
    fn set_elem0(&mut self, value: &QVariant) -> bool {
        T0::try_from(value).map(|val| self.0 = val).is_ok()
    }
    fn set_elem1(&mut self, value: &QVariant) -> bool {
        T1::try_from(value).map(|val| self.1 = val).is_ok()
    }
    fn set_elem2(&mut self, value: &QVariant) -> bool {
        T2::try_from(value).map(|val| self.2 = val).is_ok()
    }
    fn set_elem3(&mut self, value: &QVariant) -> bool {
        T3::try_from(value).map(|val| self.3 = val).is_ok()
    }
    fn set_elem4(&mut self, value: &QVariant) -> bool {
        T4::try_from(value).map(|val| self.4 = val).is_ok()
    }
    fn role_names() -> HashMap<i32, String> {
        let roles= [
            (0x100, "_0".into()),
            (0x101, "_1".into()),
            (0x102, "_2".into()),
            (0x103, "_3".into()),
            (0x104, "_4".into()),
        ];
        roles.iter().cloned().collect()
    }
}

impl<T0, T1, T2, T3, T4, T5> QModelItem for (T0, T1, T2, T3, T4, T5)
where
    for<'a> QVariant: From<&'a T0>,
    for<'a> T0: TryFrom<&'a QVariant, Error = ()>,
    for<'a> QVariant: From<&'a T1>,
    for<'a> T1: TryFrom<&'a QVariant, Error = ()>,
    for<'a> QVariant: From<&'a T2>,
    for<'a> T2: TryFrom<&'a QVariant, Error = ()>,
    for<'a> QVariant: From<&'a T3>,
    for<'a> T3: TryFrom<&'a QVariant, Error = ()>,
    for<'a> QVariant: From<&'a T4>,
    for<'a> T4: TryFrom<&'a QVariant, Error = ()>,
    for<'a> QVariant: From<&'a T5>,
    for<'a> T5: TryFrom<&'a QVariant, Error = ()>,
{
    const LEN: usize = 6;
    fn elem0(&self) -> QVariant { QVariant::from(&self.0) }
    fn elem1(&self) -> QVariant { QVariant::from(&self.1) }
    fn elem2(&self) -> QVariant { QVariant::from(&self.2) }
    fn elem3(&self) -> QVariant { QVariant::from(&self.3) }
    fn elem4(&self) -> QVariant { QVariant::from(&self.4) }
    fn elem5(&self) -> QVariant { QVariant::from(&self.5) }
    fn set_elem0(&mut self, value: &QVariant) -> bool {
        T0::try_from(value).map(|val| self.0 = val).is_ok()
    }
    fn set_elem1(&mut self, value: &QVariant) -> bool {
        T1::try_from(value).map(|val| self.1 = val).is_ok()
    }
    fn set_elem2(&mut self, value: &QVariant) -> bool {
        T2::try_from(value).map(|val| self.2 = val).is_ok()
    }
    fn set_elem3(&mut self, value: &QVariant) -> bool {
        T3::try_from(value).map(|val| self.3 = val).is_ok()
    }
    fn set_elem4(&mut self, value: &QVariant) -> bool {
        T4::try_from(value).map(|val| self.4 = val).is_ok()
    }
    fn set_elem5(&mut self, value: &QVariant) -> bool {
        T5::try_from(value).map(|val| self.5 = val).is_ok()
    }
    fn role_names() -> HashMap<i32, String> {
        let roles= [
            (0x100, "_0".into()),
            (0x101, "_1".into()),
            (0x102, "_2".into()),
            (0x103, "_3".into()),
            (0x104, "_4".into()),
            (0x105, "_5".into()),
        ];
        roles.iter().cloned().collect()
    }
}

/// A Qt-aware vector that acts both as a container and a `QAbstractItemModel`.
///
/// `QVec<T>` mirrors the API of `Vec<T>` where possible (`push`, `pop`, `len`,
/// indexing, iteration) but emits the necessary Qt model-change signals so QML
/// views update automatically.
///
/// # Supported item types and respective role names
///
/// * **Primitive types** (e.g. `QVec<i32>`): exposed under the `"value"` role.
/// * **Tuples** (e.g. `QVec<(i32, String)>`): roles are `"_0"`, `"_1"`, ...
/// * **Structs implementing `QModelItem`**: roles come from the trait;
///   `#[derive(QModelItem)]` generates roles from field names.
///
/// Up to 15 roles are supported.
///
/// # No mutable references
///
/// Methods like `get_mut`, `index_mut`, and `iter_mut` are intentionally
/// unavailable because modifying items through direct references would bypass
/// Qt’s notification system. Use `set` to update elements so QML views are
/// notified correctly.
///
/// # QML integration
///
/// A model created with `QVec<T>` can be exposed as a QAbstractItemModel to QML.
/// In QML, the model can be used to fill ListViews, Repeaters, and similar items.
///
pub struct QVec<T>
{
    data: Vec<T>,
}

impl<T: fmt::Debug> fmt::Debug for QVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("QVec")
            .field("data", &self.data)
            .finish()
    }
}

impl<T> Default for QVec<T>
where
    T: crate::QModelItem + Default + 'static
{
    /// Creates an empty `QVec`, requiring `T: Default + QModelItem`.
    fn default() -> Self {
        Self::new(Vec::default())
    }
}

impl<T> QVec<T>
where
    T: crate::QModelItem + Default + 'static
{
    /// Creates a new `QVec` from the given vector.
    ///
    /// Since any modification of QVec emits signals to Qt, it is most
    /// efficiently created with this function.
    pub fn new(data: Vec<T>) -> Self {
        QVec {
            data: data,
        }
    }

    /// Appends an element to the end of the vector and notifies QML views.
    pub fn push(&mut self, value: T) {
        self.begin_insert_rows(&QModelIndex::default(),
            self.len() as i32,
            self.len() as i32);
        self.data.push(value);
        self.end_insert_rows();
    }

    /// Removes the last element and notifies QML views.
    ///
    /// Returns [`None`] if the vector is empty.
    pub fn pop(&mut self) -> Option<T> {
        self.begin_remove_rows(&QModelIndex::default(),
            self.len() as i32 - 1,
            self.len() as i32 - 1);
        let result = self.data.pop();
        self.end_remove_rows();
        result
    }

    /// Returns the number of elements in the vector.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the vector contains no elements.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns a reference to an element or subslice depending on the type of index.
    ///
    /// * If given a position, returns a reference to the element at that position or None if out of bounds.
    /// * If given a range, returns the subslice corresponding to that range, or None if out of bounds.
    ///
    /// Note: No mutable access is provided because it would bypass Qt change
    /// notifications.
    ///
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[T]>,
    {
        self.data.get(index)
    }

    /// Replaces the element at `index` and emits a `dataChanged` signal.
    ///
    /// Use this instead of mutating elements in place so QML views update correctly.
    pub fn set(&mut self, index: usize, value: T) {
        self.data[index] = value;
        let index_ref = self.create_index(index as i32,0, 0);
        self.data_changed(&index_ref, &index_ref);
    }

    /// Clears all items and resets the model.
    ///
    /// Emits a full model reset; views will completely refresh.
    pub fn clear(&mut self) {
        self.begin_reset_model();
        self.data.clear();
        self.end_reset_model();
    }
}

impl<T, I: SliceIndex<[T]>> Index<I> for QVec<T> {
    type Output = I::Output;

    /// Indexing operator (`qvec[i]`). Panics if out of bounds.
    ///
    /// Only shared indexing is supported; mutable indexing is intentionally omitted.
    fn index(&self, index: I) -> &Self::Output {
        &self.data.index(index)
    }
}

impl<T> QVec<T> {
    /// Returns an iterator over the slice.
    ///
    /// The iterator yields all items from start to end.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }
}

#[qobject_impl_internal(Base = QAbstractItemModel)]
impl<T> QVec<T>
where
    T: crate::QModelItem + Default + 'static
{
    #[overridden]
    fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex {
        self.create_index(row, column, parent.internal_pointer())
    }

    #[overridden]
    fn parent(&self, _child: &QModelIndex) -> QModelIndex {
        QModelIndex::default()
    }

    #[overridden]
    fn row_count(&self, parent: &QModelIndex) -> i32 {
        if parent.is_valid() {
            return 0;
        }
        self.data.len() as i32
    }

    #[overridden]
    fn column_count(&self, _parent: &QModelIndex) -> i32 {
        1
    }

    fn map_role_to_element_id(&self, role: i32) -> i32 {
        if role > 0x6 {
            return role - 0x100;
        }
        role
    }

    #[overridden]
    fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        if !index.is_valid() {
            return QVariant::default()
        }

        let role = self.map_role_to_element_id(role);
        let row = index.row() as usize;

        match role {
            0 => self.data[row].elem0(),
            1 => self.data[row].elem1(),
            2 => self.data[row].elem2(),
            3 => self.data[row].elem3(),
            4 => self.data[row].elem4(),
            5 => self.data[row].elem5(),
            6 => self.data[row].elem6(),
            7 => self.data[row].elem7(),
            8 => self.data[row].elem8(),
            9 => self.data[row].elem9(),
            10 => self.data[row].elem10(),
            11 => self.data[row].elem11(),
            12 => self.data[row].elem12(),
            13 => self.data[row].elem13(),
            14 => self.data[row].elem14(),
            _ => QVariant::default()
        }
    }

    #[overridden]
    fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool {
        if !index.is_valid() {
            return false;
        }

        let role = self.map_role_to_element_id(role);
        let indexed_value = &mut self.data[index.row() as usize];

        match role {
            0 => indexed_value.set_elem0(value),
            1 => indexed_value.set_elem1(value),
            2 => indexed_value.set_elem2(value),
            3 => indexed_value.set_elem3(value),
            4 => indexed_value.set_elem4(value),
            5 => indexed_value.set_elem5(value),
            6 => indexed_value.set_elem6(value),
            7 => indexed_value.set_elem7(value),
            8 => indexed_value.set_elem8(value),
            9 => indexed_value.set_elem9(value),
            10 => indexed_value.set_elem10(value),
            11 => indexed_value.set_elem11(value),
            12 => indexed_value.set_elem12(value),
            13 => indexed_value.set_elem13(value),
            14 => indexed_value.set_elem14(value),
            _ => false,
        }
    }

    #[overridden]
    fn role_names(&self)-> QHash<i32, QByteArray> {
        let names = T::role_names();
        if names.is_empty() {
            let proxy = qvec_impl_details::get_rust_proxy(self);
            proxy.base_role_names()
        } else {
            let mut result = QHash::default();
            names.iter()
                .for_each(|(k, v)| result.insert(k, &QByteArray::from(v)));
            result
        }
    }
}
