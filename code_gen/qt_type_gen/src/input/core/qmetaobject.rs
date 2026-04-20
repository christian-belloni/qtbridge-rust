// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::QMetaType;

#[qt_gen::bridge]
mod qmetaobject {
    include_in_cpp!(<QMetaObject>);

    /// The QMetaObject struct contains meta-information about Qt objects.
    ///
    /// See also: [QMetaObject documentation](https://doc.qt.io/qt-6/qmetaobject.html).
    struct QMetaObject;

    /// Returns the metatype corresponding to this metaobject.
    pub fn meta_type(&self) -> QMetaType {
        let cpp = cpp_fn!(|&self| -> QMetaType {
            return self.metaType();
        });
        cpp(self)
    }
}
