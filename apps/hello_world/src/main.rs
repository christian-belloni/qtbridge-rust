// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge::{QApp, qobject};

#[qobject]
mod backend {
    use qtbridge::qml_element;

    #[derive(Default)]
    #[qml_element]
    pub struct Backend {
    }

    impl Backend {
        #[qslot]
        fn say_hello(&self) {
            println!("Hello World!")
        }
    }
}

fn main() {
    QApp::new()
        .load_qml(include_bytes!("Main.qml"))
        .run();
}
