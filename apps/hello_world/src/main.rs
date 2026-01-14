// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge::{QApp, qobject_impl, qml_element};

#[derive(Default)]
pub struct Backend {
}

#[qobject_impl]
#[qml_element]
impl Backend {
    #[qslot]
    fn say_hello(&self) {
        println!("Hello World!")
    }
}

impl Drop for Backend {
    fn drop(&mut self) {
        self.detach_qobject();
    }
}

fn main() {
    QApp::new()
        .load_qml(include_bytes!("Main.qml"))
        .run();
}
