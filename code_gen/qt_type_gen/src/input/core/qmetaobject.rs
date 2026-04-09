// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::QMetaType;
use crate::QObject;

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

    pub fn invoke_method(obj: *mut QObject, name: &str) -> bool {
        let cpp = cpp_fn!(|obj: *mut QObject, name: &str| -> bool {
            QByteArray nameBa = RustStrToQByteArray(name);
            return QMetaObject::invokeMethod(obj, nameBa.constData(), Qt::QueuedConnection);
        });
        unsafe { cpp(obj, name) }
    }
}
