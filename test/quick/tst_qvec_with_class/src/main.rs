// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge::qobject;
use qtbridge::qt_type_lib::{QVariant};
use qtbridge::{QVec, QModelItem, QApp};

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, Default, QModelItem)]
struct MyClass {
    value: i32,
    decoration: String,
    display: String,

}

impl MyClass {
    pub fn new(value: i32, dec: &str) -> Self {
        MyClass {
            value: value,
            decoration: dec.to_string(),
            display: value.to_string(),
        }
    }
}

#[qobject]
mod vector_changer {

    use super::{RefCell, Rc};
    use super::QVec;
    use super::MyClass;

    #[derive(Default)]
    pub struct VectorChanger {
        data: Rc<RefCell<QVec<MyClass>>>,
    }

    impl VectorChanger {
        pub fn new(shared: &Rc<RefCell<QVec<MyClass>>>) -> Self {
            Self { data: shared.clone() }
        }
    }

    impl VectorChanger {
        #[qslot]
        fn append(&mut self) {
            self.data.borrow_mut().push(
                MyClass {
                    value: 50,
                    decoration: "orange".to_string(),
                    display: "50".to_string(),
                });
        }

        #[qslot]
        fn remove_last(&mut self) {

            if (*self.data.borrow()).len() > 0 {
                (*self.data.borrow_mut()).pop();
            }
        }

        #[qslot]
        fn change_all(&mut self) {
            let mut vec= self.data.borrow_mut();
            let len = vec.len();
            for i in 0..len {
                let mut display = vec.get(i).unwrap().display.clone();
                let decoration = vec.get(i).unwrap().decoration.clone();
                let value = vec.get(i).unwrap().value * 2;
                display = display.repeat(2);
                vec.set(i,
                    MyClass {
                    value: value,
                    decoration: decoration,
                    display: display,
                });
            }
        }
    }
}

use vector_changer::VectorChanger;

#[test]
fn test_qvec() {
    use std::env;
    use std::path::PathBuf;

    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    input_path.push("qml");
    let input_folder = input_path.to_str().unwrap().to_string();

    println!("Running quick test with qml files in \"{}\"", &input_folder);

    let args: Vec<String> = vec![
        file!().to_string(),
        "-input".to_string(),
        input_folder,
    ];

    use qtbridge::qt_type_lib::QVariantMap;
    use qtbridge::quicktest::quick_test_main_with_properties;

    let qv = QVec::default_with_attached_qobject();

    qv.borrow_mut().push(MyClass::new(10, "red"));
    qv.borrow_mut().push(MyClass::new(20, "green"));
    qv.borrow_mut().push(MyClass::new(30, "blue"));

    let vchanger = Rc::new(RefCell::new(VectorChanger::new(&qv)));
    VectorChanger::attach_qobject(&vchanger);

    let properties = QVariantMap::from([
        ("listmodel", qv.borrow().as_qvariant()),
        ("listchanger", vchanger.borrow().as_qvariant()),
    ]);

    let result = quick_test_main_with_properties(&args, &"test_qabstractitemmodel".to_string(), &properties);

    assert_eq!(result, 0, "quick_test failed with code {}", result);
}

fn main() {

    let qv = QVec::default_with_attached_qobject();
    let vchanger = Rc::new(RefCell::new(VectorChanger::new(&qv)));
    VectorChanger::attach_qobject(&vchanger);

    qv.borrow_mut().push(MyClass::new(10, "red"));
    qv.borrow_mut().push(MyClass::new(20, "green"));
    qv.borrow_mut().push(MyClass::new(30, "blue"));

    let initial_properties = [
        ("listmodel", qv.borrow().as_qvariant()),
        ("listchanger", vchanger.borrow().as_qvariant()),
    ];

    QApp::new()
        .with_initial_properties(&initial_properties)
        .load_qml(include_bytes!("main.qml"))
        .run();
}
