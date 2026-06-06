// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::mem::MaybeUninit;
use crate::{QMetaType, QMetaTypeGet, QObject};

#[qt_gen::bridge]
mod qobjectlist {
    include_in_cpp!(<QObject>);
    include_in_cpp!(<QList>);

    #[derive_cpp(Default, Drop)]
    /// Binding for `QObjectList` = `QList<QObject*>`, used as the metacall wire type
    /// for `Vec<Rc<RefCell<T>>>` signal/slot arguments.
    struct QObjectList {
        _d: MaybeUninit<usize>,
        _ptr: MaybeUninit<usize>,
        _size: MaybeUninit<usize>,
    }

    /// Appends a `QObject` pointer to the list.
    pub fn append(&mut self, item: *mut QObject) {
        let cpp = cpp_fn!(|&mut self, item: *mut QObject| {
            self.append(item);
        });
        unsafe { cpp(self, item) }
    }

    /// Returns the number of elements in the list.
    pub fn size(&self) -> isize {
        cpp_fn!(|&self| -> isize {
            return self.size();
        })(self)
    }

    /// Returns the element at position `idx`.
    pub fn at(&self, idx: isize) -> *mut QObject {
        let cpp = cpp_fn!(|&self, idx: isize| -> *mut QObject {
            return self.at(idx);
        });
        unsafe { cpp(self, idx) }
    }

    pub fn len(&self) -> usize {
        self.size() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    impl QMetaTypeGet for QObjectList {
        fn get_qmetatype() -> QMetaType {
            cpp_fn!(|| -> QMetaType {
                return QMetaType::fromType<QObjectList>();
            })()
        }
    }
}
