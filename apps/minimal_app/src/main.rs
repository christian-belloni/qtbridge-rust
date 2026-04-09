// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge::{QApp, qobject};

#[qobject(Base = QListModel)]
mod backend {

    use qtbridge::{QListModel, QListModelBase};

    #[derive(Default)]
    pub struct Backend {
        string_list: Vec<String>,
    }

    impl QListModel for Backend {
        type Item = String;

        fn len(&self) -> usize {
            self.string_list.len()
        }
        fn get(&self, index: usize) -> Option<&String> {
            self.string_list.get(index)
        }
        fn set_unnotified(&mut self, index: usize, value: String) -> bool {
            match self.string_list.contains(&value) {
                true => { self.duplicate_found(&value); return false },
                false => { self.string_list[index] = value; return true },
            }
        }
        fn push_unnotified(&mut self, value: String) {
            self.string_list.push(value);
        }
        fn remove_unnotified(&mut self, index: usize) -> String {
            self.string_list.remove(index)
        }
    }

    impl Backend {
        #[qslot]
        fn add_string(&mut self, value: &str) {
            match self.string_list.contains(&value.to_string()) {
                true => self.duplicate_found(&value),
                false => self.push(value.to_string()),
            }
        }
        #[qsignal]
        fn duplicate_found(&self, duplicate: &str);
    }
}

fn main() {
    QApp::new()
        .register::<backend::Backend>()
        .load_qml(include_bytes!("Main.qml"))
        .run();
}
