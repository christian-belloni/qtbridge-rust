// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge::QApp;
use qtbridge::QObjectHolder;

use tst_qabstractitemmodel::Backend;

fn main() {
    let backend = Backend::default_with_attached_qobject();

    let properties = [("rustmodel", backend.borrow().as_qvariant())];
    QApp::new()
        .with_initial_properties(&properties)
        .load_qml(include_bytes!("main.qml"))
        .run();
}
