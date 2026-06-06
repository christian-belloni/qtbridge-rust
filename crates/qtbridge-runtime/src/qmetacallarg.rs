// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::cell::RefCell;
use std::rc::Rc;

use qtbridge_type_lib::{
    QMetaType, QMetaTypeGet, QObject, QString, QObjectList,
    QList_bool, QList_i8, QList_u8, QList_i16, QList_u16,
    QList_i32, QList_u32, QList_i64, QList_u64, QList_isize, QList_usize,
    QList_f32, QList_f64, QList_QString,
};

use crate::{QObjectHolder, QmlRegister};

/// Describes how a Rust type is marshalled through Qt's metacall machinery.
///
/// The associated `WireType` is what actually appears in the `argv` array during
/// a metacall (signal emission or slot invocation). For types that are already
/// metacall-compatible (primitives, `bool`) `WireType = Self`. For Rust-specific
/// types an intermediate representation is used:
///
/// | Rust type              | Wire type        |
/// |------------------------|------------------|
/// | primitives / `bool`    | self             |
/// | `String`               | `QString`        |
/// | `Rc<RefCell<T>>`       | `*mut QObject`   |
/// | `Vec<Rc<RefCell<T>>>`  | `QObjectList`    |
pub trait QMetaCallArg: Sized {
    type WireType: Sized;

    fn to_wire(&self) -> Self::WireType;
    fn from_wire(wire: &Self::WireType) -> Self;
    fn wire_metatype() -> QMetaType;
}

macro_rules! impl_direct {
    ($($t:ty),*) => {
        $(impl QMetaCallArg for $t {
            type WireType = $t;
            fn to_wire(&self) -> $t { *self }
            fn from_wire(wire: &$t) -> $t { *wire }
            fn wire_metatype() -> QMetaType { <$t as QMetaTypeGet>::get_qmetatype() }
        })*
    }
}

impl_direct!(bool, i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64);

macro_rules! impl_vec_direct {
    ($($t:ty => $qlist:ty),*) => {
        $(impl QMetaCallArg for Vec<$t> {
            type WireType = $qlist;
            fn to_wire(&self) -> $qlist { self.into() }
            fn from_wire(wire: &$qlist) -> Vec<$t> { wire.into() }
            fn wire_metatype() -> QMetaType { <$qlist as QMetaTypeGet>::get_qmetatype() }
        })*
    }
}

impl_vec_direct!(
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

impl QMetaCallArg for String {
    type WireType = QString;
    fn to_wire(&self) -> QString { self.into() }
    fn from_wire(wire: &QString) -> String { wire.into() }
    fn wire_metatype() -> QMetaType { <QString as QMetaTypeGet>::get_qmetatype() }
}

impl<T: QObjectHolder> QMetaCallArg for Rc<RefCell<T>> {
    type WireType = *mut QObject;

    fn to_wire(&self) -> *mut QObject {
        T::rc_ref_cell_to_qobject(self).cast_mut()
    }

    fn from_wire(wire: &*mut QObject) -> Self {
        unsafe { T::qobject_to_rc_ref_cell(*wire) }
    }

    fn wire_metatype() -> QMetaType {
        <*mut QObject as QMetaTypeGet>::get_qmetatype()
    }
}

impl<T: QmlRegister> QMetaCallArg for Vec<Rc<RefCell<T>>> {
    type WireType = QObjectList;

    fn to_wire(&self) -> QObjectList {
        let mut list = QObjectList::default();
        for rc in self {
            list.append(T::rc_ref_cell_to_qobject(rc).cast_mut());
        }
        list
    }

    fn from_wire(wire: &QObjectList) -> Self {
        (0..wire.size())
            .map(|i| unsafe { T::qobject_to_rc_ref_cell(wire.at(i)) })
            .collect()
    }

    fn wire_metatype() -> QMetaType {
        QObjectList::get_qmetatype()
    }
}
