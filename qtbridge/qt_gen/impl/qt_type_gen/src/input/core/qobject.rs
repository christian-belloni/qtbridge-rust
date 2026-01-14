// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::mem::MaybeUninit;

#[qt_gen::bridge]
mod qobject {
    include_in_cpp!(<QObject>);

    #[doc(hidden)]
    struct QObject {
        // QScopedPointer<QObjectData> d_ptr;
        _d_ptr: MaybeUninit<usize>,
        _maybe_vtable: MaybeUninit<usize>,
    }

    /// Call C++ 'delete' operator on pointer given in the argument.
    pub fn delete(obj: *mut QObject) {
        let cpp = cpp_fn!(|obj: *mut QObject| {
            delete obj;
        });
        unsafe { cpp(obj) }
    }

    /// Call QObject (virtual) destructor.
    pub fn destruct(obj: *mut QObject) {
        let cpp = cpp_fn!(|obj: *mut QObject| {
            obj->~QObject();
        });
        unsafe { cpp(obj) }
    }
}
