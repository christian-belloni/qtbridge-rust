// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge_type_lib::QVariant;

#[doc(hidden)]
pub fn slot_callback_for<T>(this_callback: fn(&mut T, &[*const u8], &[*mut u8])) -> fn(*mut u8, &[*const u8], &[*mut u8]) {
    unsafe {
        std::mem::transmute::<_, _>(this_callback)
    }
}

//TODO: make self& not mut for read callbacks?
#[doc(hidden)]
pub fn property_read_callback_for<T>(this_callback: fn(&mut T)->QVariant) -> fn(*mut u8)->QVariant {
    unsafe {
        std::mem::transmute::<_, _>(this_callback)
    }
}

#[doc(hidden)]
pub fn property_write_callback_for<T>(this_callback: fn(&mut T, &QVariant)) -> fn(*mut u8, &QVariant) {
    unsafe {
        std::mem::transmute::<_, _>(this_callback)
    }
}
