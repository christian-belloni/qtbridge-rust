// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]
use qtbridge::{QmlRegister, qobject};
use quicktest::quick_test_main;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[qobject]
pub mod data {
    #[derive(Default)]
    pub struct Data {
        pub int_value: i32,
        pub float_value: f32,
        pub string_value: String,
    }

    impl Data {
        qproperty!("intValue", Member = int_value);
        qproperty!("floatValue", Member = float_value);
        qproperty!("stringValue", Member = string_value);
    }
}

use data::Data;

#[qobject(Singleton)]
pub mod backend {
    use std::cell::RefCell;
    use std::rc::Rc;
    use qtbridge::QObjectHolder;
    use super::Data;

    #[derive(Default)]
    pub struct Backend {
    }

    impl Backend {
        #[qsignal]
        fn data_changed(&mut self, data: Rc<RefCell<Data>>);

        #[qslot]
        fn emit_data_changed(&mut self) {
            let data = Rc::new(RefCell::new(Data {
                int_value:42,
                float_value: 0.25,
                string_value: "Some string".into()
            }));
            Data::attach_qobject(&data);

            self.data_changed(data);
        }
    }
}

use backend::Backend;

fn signal_with_nested_type_can_be_emitted() {
    Data::register();
    Backend::register();

    let args = vec![
        file!().into(),
        "-input".into(),
        format!("{MANIFEST_DIR}/tests/qml/signal_nested_type.qml")
    ];

    let result = quick_test_main(&args, &"signal_nested_types".into());
    assert_eq!(result, 0, "quick test failed");
}

fn main() {
    signal_with_nested_type_can_be_emitted();
}
