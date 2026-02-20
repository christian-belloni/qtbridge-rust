// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#![cfg(test)]
use qtbridge::qobject;

#[qobject]
mod generic_trivial {

    #[derive(Default)]
    #[allow(dead_code)]
    pub struct Backend<T>
    where T: 'static + Default {
        data: Vec<T>
    }
}

#[qobject]
mod trivial1 {
    #[derive(Default)]
    pub struct Backend1 {}
}

#[qobject]
mod trivial2 {
    #[derive(Default)]
    pub struct Backend2 {}
}

#[test]
fn test_structs_have_unique_metatype() {
    use qtbridge::qt_type_lib::QMetaTypeGet;
    let a = <trivial1::Backend1 as QMetaTypeGet>::get_qmetatype().id();
    let b = <trivial2::Backend2 as QMetaTypeGet>::get_qmetatype().id();
    assert_ne!(a, b, "QMetaTypes are not unique for 2 different types");
}

#[test]
fn test_generics_have_unique_metatype() {
    use qtbridge::qt_type_lib::QMetaTypeGet;
    let a = <generic_trivial::Backend<i32> as QMetaTypeGet>::get_qmetatype().id();
    let b = <generic_trivial::Backend<String> as QMetaTypeGet>::get_qmetatype().id();
    assert_ne!(a, b, "QMetaTypes are not unique for 2 generic instantiations");
}
