// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge::{QApp, qobject};

#[qobject(Singleton)]
mod backend {

    #[derive(Default)]
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
    <backend::Backend as qtbridge::QmlRegister>::register();
    QApp::new()
        .load_qml(include_bytes!("Main.qml"))
        .run();
}
