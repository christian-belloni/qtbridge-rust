// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge_type_lib::{QMetaObject, QMetaType};

/// `QCppProxy` defines what a C++ proxy to a QObject (C++) must implement.
///
/// This includes access to the C++ static meta-object, which is then extended
/// on the Rust side to create a dynamic meta-object.
pub trait QCppProxy {
    fn get_static_meta_object() -> &'static QMetaObject;
    fn get_size() -> usize;
    fn get_align() -> usize;
    fn get_qmetatype_list() -> QMetaType;
}
