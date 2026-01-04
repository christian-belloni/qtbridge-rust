// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge::{QApp, qobject};

#[qobject(Base = QAbstractListModel)]
mod backend {

    use qtbridge::qml_element;
    use qtbridge::qt_type_lib::{QModelIndex, QVariant};

    #[derive(Default)]
    #[qml_element]
    pub struct Backend {
        string_list: Vec<String>,
    }

    impl Backend {
        #[overridden]
        fn row_count(&self, _index: &QModelIndex) -> i32 {
            self.string_list.len() as i32
        }

        #[overridden]
        fn data(&self, index: &QModelIndex, _role: i32) -> QVariant {
            QVariant::from(&self.string_list[index.row() as usize])
        }

        #[overridden]
        fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool {
            let first = first as usize;
            let last = first + count as usize;

            if last > self.string_list.len() {
                return false;
            }
            self.begin_remove_rows(parent, first as i32, (last - 1) as i32);
            self.string_list.drain(first..last);
            self.end_remove_rows();
            true
        }

        #[overridden]
        fn set_data(&mut self, index: &QModelIndex, value: &QVariant, _role: i32) -> bool {
            if let Ok(value_str) = String::try_from(value) {
                if !self.string_list.contains(&value_str) {
                    self.string_list[index.row() as usize] = value_str;
                    self.data_changed(index, index);
                    return true;
                }
                self.duplicate_found(&value_str);
            }
            return false;
        }

        #[qslot]
        fn add_string(&mut self, value: &str) {
            match self.string_list.contains(&value.to_string()) {
                true => self.duplicate_found(&value),
                false => self.append_prechecked_string(value),
            }
        }

        fn append_prechecked_string(&mut self, new_value: &str) {
            let len = self.string_list.len() as i32;
            self.begin_insert_rows(&QModelIndex::default(), len, len);
            self.string_list.push(new_value.to_string());
            self.end_insert_rows();
        }

        #[qsignal]
        fn duplicate_found(&self, duplicate: &str);
    }
}

fn main() {
    QApp::new()
        .load_qml(include_bytes!("Main.qml"))
        .run();
}
