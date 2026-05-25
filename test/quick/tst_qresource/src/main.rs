// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge::{include_bytes_qml, QApp};

fn main() {

    // Images from https://rustacean.net/ by Karen Rustad Tölva.
    // Public domain.
    include_bytes_qml!("rustacean-orig-noshadow.png", "data");
    include_bytes_qml!("rustacean-flat-happy.png", "data");
    include_bytes_qml!("rustacean-orig-noshadow.png", "data2");
    include_bytes_qml!("text/example.txt", "data");
    include_bytes_qml!("example.txt", "text/data");

    QApp::new()
        .load_qml(include_bytes!("main.qml"))
        .run();
}
