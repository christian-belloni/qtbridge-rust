// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qt_type_lib::QVariant;
use crate::metamethodparams::ffi::MetaMethodIncomingParams;

pub fn slot_callback_for<T>(this_callback: fn(&mut T, &MetaMethodIncomingParams)) -> fn(*mut u8, &MetaMethodIncomingParams) {
    unsafe {
        std::mem::transmute::<
            fn(&mut T,  &MetaMethodIncomingParams),
            fn(*mut u8, &MetaMethodIncomingParams)
        >(this_callback)
    }
}

//TODO: make self& not mut for read callbacks?
pub fn property_read_callback_for<T>(this_callback: fn(&mut T)->QVariant) -> fn(*mut u8)->QVariant {
    unsafe {
        std::mem::transmute::<
            fn(&mut T)->QVariant,
            fn(*mut u8)->QVariant,
        >(this_callback)
    }
}

pub fn property_write_callback_for<T>(this_callback: fn(&mut T, &QVariant)) -> fn(*mut u8, &QVariant) {
    unsafe {
        std::mem::transmute::<
            fn(&mut T, &QVariant),
            fn(*mut u8, &QVariant),
        >(this_callback)
    }
}
