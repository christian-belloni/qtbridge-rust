// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge::{qobject, QVec, QApp};

use std::cell::RefCell;
use std::rc::Rc;

#[qobject]
mod vector_changer {
    use super::Rc;
    use super::RefCell;
    use super::QVec;

    #[derive(Default)]
    pub struct VectorChanger {
        data: Rc<RefCell<QVec<(String, i32)>>>,
    }

    impl VectorChanger {
        pub fn new(shared: &Rc<RefCell<QVec<(String, i32)>>>) -> Self {
            Self { data: shared.clone() }
        }
    }

    impl VectorChanger {
        #[qslot]
        fn append(&mut self) {
            self.data.borrow_mut().push(("50".to_string(), 50));
        }

        #[qslot]
        fn remove_last(&mut self) {
            if self.data.borrow().len() > 0 {
                self.data.borrow_mut().pop();
            }
        }

        #[qslot]
        fn change_all(&mut self) {
            let len = self.data.borrow().len();
            for i in 0..len {
                self.data.borrow_mut().set(i, ("60".to_string(), 60));
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

    qv.borrow_mut().push(("10".to_string(), 10));
    qv.borrow_mut().push(("20".to_string(), 20));
    qv.borrow_mut().push(("30".to_string(), 30));

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

    qv.borrow_mut().push(("10".to_string(), 10));
    qv.borrow_mut().push(("20".to_string(), 20));
    qv.borrow_mut().push(("30".to_string(), 30));

    let initial_properties = [
        ("listmodel", qv.borrow().as_qvariant()),
        ("listchanger", vchanger.borrow().as_qvariant()),
    ];

    QApp::new()
        .with_initial_properties(&initial_properties)
        .load_qml(include_bytes!("main.qml"))
        .run();
}
