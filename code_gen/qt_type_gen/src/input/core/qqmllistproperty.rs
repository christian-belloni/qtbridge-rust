// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::{QMetaType, QObject, QVariant};

#[qt_gen::bridge]
mod qqmllistproperty {
    include_in_cpp!(<QObject>);
    include_in_cpp!(<QVariant>);
    include_in_cpp!(<QtQml/QQmlListProperty>);

    /// Build a `QVariant` holding a `QQmlListProperty<QObject>` stamped with the per-type list metatype.
    pub unsafe fn list_property_to_qvariant(
        meta_type: &QMetaType,
        object: *mut QObject,
        data: *mut u8,
        append_fn: usize,
        count_fn: usize,
        at_fn: usize,
        clear_fn: usize,
    ) -> QVariant {
        let cpp = cpp_fn!(|meta_type: &QMetaType, object: *mut QObject, data: *mut u8, append_fn: usize, count_fn: usize, at_fn: usize, clear_fn: usize| -> QVariant {
            auto appendFn = reinterpret_cast<QQmlListProperty<QObject>::AppendFunction>(append_fn);
            auto countFn  = reinterpret_cast<QQmlListProperty<QObject>::CountFunction>(count_fn);
            auto atFn     = reinterpret_cast<QQmlListProperty<QObject>::AtFunction>(at_fn);
            auto clearFn  = reinterpret_cast<QQmlListProperty<QObject>::ClearFunction>(clear_fn);

            QQmlListProperty<QObject> prop(object, static_cast<void*>(data), appendFn, countFn, atFn, clearFn);
            return QVariant(meta_type, &prop);
        });
        unsafe { cpp(meta_type, object, data, append_fn, count_fn, at_fn, clear_fn) }
    }
}
