// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::collections::HashMap;

use qt_type_lib::QVariant;

/// Trait representing a single item in a Qt item model.
///
/// This trait is implemented automatically by `#[derive(QModelItem)]`
/// for structs and tuple structs and an implementation for primitive types
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
/// use qt_traits::QModelItem;
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

macro_rules! impl_qmodel_item_for_primitive {
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

impl_qmodel_item_for_primitive!(i8);
impl_qmodel_item_for_primitive!(i16);
impl_qmodel_item_for_primitive!(i32);
impl_qmodel_item_for_primitive!(i64);
impl_qmodel_item_for_primitive!(u8);
impl_qmodel_item_for_primitive!(u16);
impl_qmodel_item_for_primitive!(u32);
impl_qmodel_item_for_primitive!(u64);
impl_qmodel_item_for_primitive!(f32);
impl_qmodel_item_for_primitive!(f64);
impl_qmodel_item_for_primitive!(bool);
impl_qmodel_item_for_primitive!(String);

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
