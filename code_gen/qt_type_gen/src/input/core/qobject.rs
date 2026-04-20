// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::mem::MaybeUninit;
use crate::{QMetaObject, QVariant};

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

    /// Returns a pointer to the meta-object of this object.
    pub fn get_qmeta_object(&self) -> *const QMetaObject {
        let cpp = cpp_fn!(|&self| -> *const QMetaObject {
            return self.metaObject();
        });
        unsafe { cpp(self) }
    }

    /// Returns a [QVariant][crate::QVariant] containing the value of the object's property with the given name.
    ///
    /// If this property does not exist, the returned `QVariant` is invalid.
    pub fn property(&self, name: &str) -> QVariant {
        let cpp = cpp_fn!(|&self, name: &[u8]| -> QVariant {
            return self.property(reinterpret_cast<const char*>(name.data()));
        });
        let cstr = std::ffi::CString::new(name)
            .expect("CString::new() failed");
        cpp(self, cstr.as_bytes())
    }

    /// Sets the value of the object's property with the given name.
    ///
    /// If the property is defined in the object,
    /// the function returns `true` on success and `false` otherwise.
    ///
    /// If the property is not defined,
    /// and therefore is not listed in the meta-object,
    /// it is added as a dynamic property and `false` is returned.
    pub fn set_property(&mut self, name: &str, value: QVariant) -> bool {
        let cpp = cpp_fn!(|&mut self, name: &[u8], value: QVariant| -> bool {
            return self.setProperty(reinterpret_cast<const char*>(name.data()), value);
        });
        let cstr = std::ffi::CString::new(name)
            .expect("CString::new() failed");
        cpp(self, cstr.as_bytes(), value)
    }
}
