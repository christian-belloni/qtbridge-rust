// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::QModelIndex;

#[doc(hidden)]
pub enum QMetaTypeId {
    Bool = 1,
    Int = 2,
    Uint = 3,
    Double  = 6,
    QString = 10,
    //Long = 32,
    LongLong = 4,
    Short = 33,
    //ULong = 35,
    ULongLong = 5,
    UShort = 36,
    SChar = 40,
    UChar = 37,
    Float = 38,
    QStringList = 11,
    QModelIndex = 42,
}

#[doc(hidden)]
pub trait QMetaTypeIdConst {
    const ID: QMetaTypeId;
}

impl QMetaTypeIdConst for bool {
    const ID: QMetaTypeId = QMetaTypeId::Bool;
}
impl QMetaTypeIdConst for i8 {
    const ID: QMetaTypeId = QMetaTypeId::SChar;
}
impl QMetaTypeIdConst for u8 {
    const ID: QMetaTypeId = QMetaTypeId::UChar;
}
impl QMetaTypeIdConst for i16 {
    const ID: QMetaTypeId = QMetaTypeId::Short;
}
impl QMetaTypeIdConst for u16 {
    const ID: QMetaTypeId = QMetaTypeId::UShort;
}
impl QMetaTypeIdConst for i32 {
    const ID: QMetaTypeId = QMetaTypeId::Int;
}
impl QMetaTypeIdConst for u32 {
    const ID: QMetaTypeId = QMetaTypeId::Uint;
}
impl QMetaTypeIdConst for i64 {
    const ID: QMetaTypeId = QMetaTypeId::LongLong;
}
impl QMetaTypeIdConst for u64 {
    const ID: QMetaTypeId = QMetaTypeId::ULongLong;
}
impl QMetaTypeIdConst for f32 {
    const ID: QMetaTypeId = QMetaTypeId::Float;
}
impl QMetaTypeIdConst for f64 {
    const ID: QMetaTypeId = QMetaTypeId::Double;
}
impl QMetaTypeIdConst for String {
    const ID: QMetaTypeId = QMetaTypeId::QString;
}
impl QMetaTypeIdConst for &str {
    const ID: QMetaTypeId = QMetaTypeId::QString;
}
impl QMetaTypeIdConst for Vec<String> {
    const ID: QMetaTypeId = QMetaTypeId::QStringList;
}
impl QMetaTypeIdConst for QModelIndex {
    const ID: QMetaTypeId = QMetaTypeId::QModelIndex;
}
