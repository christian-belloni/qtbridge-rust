// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::cell::RefCell;
use std::rc::Rc;

use qtbridge_type_lib::{
    QMetaType, QMetaTypeGet, QObject, QString, QVariant,
    QList_bool, QList_i8, QList_u8, QList_i16, QList_u16,
    QList_i32, QList_u32, QList_i64, QList_u64, QList_isize, QList_usize,
    QList_f32, QList_f64, QList_QString,
};

use crate::{QObjectHolder, QmlRegister};

/// Trait for types that can be used as member-based Qt properties.
///
/// Implemented for primitive numeric types, `bool`, `String`, `Vec` of those types,
/// [`Rc<RefCell<T>>`] where `T: QObjectHolder`, and [`Vec<Rc<RefCell<T>>>`] where
/// `T: QmlRegister`.
pub trait QPropertyMember: Sized {
    fn qmetatype() -> QMetaType;

    /// `Owner` is the QObject that holds this property; required by the
    /// `Vec<Rc<RefCell<T>>>` impl which passes it to QQmlListProperty.
    fn to_qvariant<Owner: QObjectHolder>(&self, owner: &Owner) -> QVariant;

    /// This is required for types that transform into a writable view and
    /// need to emit a notify signal on change. Right now only QQmlListProperty
    /// takes advantage of this. Other types are just defaulting to to_qvariant,
    /// returning a value. `Owner` is as above and `Notify` is the signal
    /// to emitted on change.
    fn to_qvariant_view<Owner, Notify>(&self, owner: &Owner, notify: Notify) -> QVariant
    where
        Owner: QObjectHolder,
        Notify: Fn(&mut Owner) + 'static,
    {
        let _ = notify;
        self.to_qvariant(owner)
    }

    fn from_qvariant(value: &QVariant) -> Result<Self, ()>;

    fn property_eq(&self, other: &Self) -> bool;
}

macro_rules! impl_primitive {
    ($($t:ty),*) => {
        $(impl QPropertyMember for $t {
            fn qmetatype() -> QMetaType { <$t as QMetaTypeGet>::get_qmetatype() }
            fn to_qvariant<Owner: QObjectHolder>(&self, _owner: &Owner) -> QVariant { QVariant::from(self) }
            fn from_qvariant(value: &QVariant) -> Result<Self, ()> { Self::try_from(value) }
            fn property_eq(&self, other: &Self) -> bool { self == other }
        })*
    }
}

impl_primitive!(bool, i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64);

macro_rules! impl_vec {
    ($($t:ty => $qlist:ty),*) => {
        $(impl QPropertyMember for Vec<$t> {
            fn qmetatype() -> QMetaType { <$qlist as QMetaTypeGet>::get_qmetatype() }
            fn to_qvariant<Owner: QObjectHolder>(&self, _owner: &Owner) -> QVariant { QVariant::from(self) }
            fn from_qvariant(value: &QVariant) -> Result<Self, ()> { Self::try_from(value) }
            fn property_eq(&self, other: &Self) -> bool { self == other }
        })*
    }
}

impl_vec!(
    bool   => QList_bool,
    i8     => QList_i8,
    u8     => QList_u8,
    i16    => QList_i16,
    u16    => QList_u16,
    i32    => QList_i32,
    u32    => QList_u32,
    i64    => QList_i64,
    u64    => QList_u64,
    isize  => QList_isize,
    usize  => QList_usize,
    f32    => QList_f32,
    f64    => QList_f64,
    String => QList_QString
);

impl QPropertyMember for String {
    fn qmetatype() -> QMetaType { <QString as QMetaTypeGet>::get_qmetatype() }
    fn to_qvariant<Owner: QObjectHolder>(&self, _owner: &Owner) -> QVariant { QVariant::from(self) }
    fn from_qvariant(value: &QVariant) -> Result<Self, ()> { Self::try_from(value) }
    fn property_eq(&self, other: &Self) -> bool { self == other }
}

impl<T: QObjectHolder> QPropertyMember for Rc<RefCell<T>> {
    fn qmetatype() -> QMetaType {
        <*mut QObject as QMetaTypeGet>::get_qmetatype()
    }

    fn to_qvariant<Owner: QObjectHolder>(&self, _owner: &Owner) -> QVariant {
        T::rc_ref_cell_to_qobject(self).cast_mut().into()
    }

    fn from_qvariant(value: &QVariant) -> Result<Self, ()> {
        let ptr = <*mut QObject>::try_from(value)?;
        Ok(unsafe { T::qobject_to_rc_ref_cell(ptr) })
    }

    fn property_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(self, other)
    }
}

impl<T: QmlRegister> QPropertyMember for Vec<Rc<RefCell<T>>> {
    fn qmetatype() -> QMetaType {
        T::get_list_qmetatype()
    }

    fn to_qvariant<Owner: QObjectHolder>(&self, owner: &Owner) -> QVariant {
        T::list_to_qvariant(owner, self, |_: &mut Owner| {})
    }

    fn to_qvariant_view<Owner, Notify>(&self, owner: &Owner, notify: Notify) -> QVariant
    where
        Owner: QObjectHolder,
        Notify: Fn(&mut Owner) + 'static,
    {
        T::list_to_qvariant(owner, self, notify)
    }

    fn from_qvariant(_value: &QVariant) -> Result<Self, ()> {
        // Vec<Rc<RefCell<T>>> is exposed as writeable view and no write operation will ever happen
        Err(())
    }

    fn property_eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(a, b)| Rc::ptr_eq(a, b))
    }
}

/// Returns the [`QMetaType`] of the return value of a `FnOnce`. Given a function
/// |this: &Self| { &this.member } this allows to infer the metatype of a member
/// field without requiring the type to be known at macro expansion time. The
/// closure is never called.
#[doc(hidden)]
pub fn get_meta_type_of_fn_return_value<F, This, R>(_f: F) -> QMetaType
where
    F: FnOnce(&This) -> &R,
    R: QPropertyMember,
{
    R::qmetatype()
}
