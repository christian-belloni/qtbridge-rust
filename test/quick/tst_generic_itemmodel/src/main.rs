// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
use std::cell::RefCell;
use std::rc::Rc;

use qtbridge::{qobject, QApp};

#[qobject(Base = QAbstractItemModel)]
mod backend {
    use qtbridge::qt_type_lib::{QVariant, QModelIndex};

    #[derive(Default)]
    pub struct Backend<T>
    where T: 'static + Default,
        for<'a> qtbridge::qt_type_lib::QVariant: From<&'a T>, // TODO: make it work without fully qualified path for QVariant
    {
        data: Vec<T>,
    }
    impl<T> Backend<T>
    where T: 'static + Default,
        for<'a> QVariant: From<&'a T>,
    {
        pub fn new(data: Vec<T>) -> Self {
            Self {
                data: data
            }
        }
    }

    impl<T> Backend<T>
        where
        T: 'static + Default,
        for<'a> qtbridge::qt_type_lib::QVariant: From<&'a T>,
    {
        #[overridden]
        fn index(&self, row: i32, column: i32, _parent: &QModelIndex) -> QModelIndex {
            self.create_index(row, column, 0)
        }
        #[overridden]
        fn parent(&self, _child: &QModelIndex) -> QModelIndex {
            QModelIndex::default()
        }
        #[overridden]
        fn row_count(&self, parent: &QModelIndex) -> i32 {
            if !parent.is_valid() {
                self.data.len() as i32
            } else {
                0
            }
        }
        #[overridden]
        fn column_count(&self, _parent: &QModelIndex) -> i32 {
            1
        }
        #[overridden]
        fn data(&self, index: &QModelIndex, _role: i32) -> QVariant {
            QVariant::from(&self.data[index.row() as usize])
        }
        #[overridden]
        fn set_data(&mut self, _index: &QModelIndex, _value: &QVariant, _role: i32) -> bool {
            false
        }
    }

}

use backend::Backend;

#[test]
fn test_qabstractitemmodel() {
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

    let data = vec![1, 2, 3, 10, 100];
    let test_object = Rc::new(RefCell::new(Backend::<i32>::new(data)));
    Backend::attach_qobject(&test_object);

    let properties = QVariantMap::from(("listmodel", test_object.borrow().as_qvariant()));
    let result = quick_test_main_with_properties(&args, &"test_qabstractitemmodel".to_string(), &properties);

    assert_eq!(result, 0, "quick_test failed with code {}", result);
}

//Manual test
fn main() {
    let data = vec![1, 2, 3, 4, 5, 10, 100, 1000];
    let backend = Rc::new(RefCell::new(Backend::<i32>::new(data)));
    Backend::attach_qobject(&backend);
    let data2 = Vec::from(["one", "two", "three", "ten", "hundred"].map(String::from));
    let backend2 = Rc::new(RefCell::new(Backend::<String>::new(data2)));
    Backend::attach_qobject(&backend2);

    use qtbridge::qt_type_lib::QMetaTypeInterfaceGet;
    let a = <Backend<i32> as QMetaTypeInterfaceGet>::get_qmetatype_interface();
    let b = <Backend<String> as QMetaTypeInterfaceGet>::get_qmetatype_interface();

    assert!(!::core::ptr::eq(a, b), "QMetaTypeInterfaces are not unique");

    let initial_properties = [
        ("rustmodel", backend.borrow().as_qvariant()),
        ("rustmodel2", backend2.borrow().as_qvariant()),
    ];

    QApp::new()
        .with_initial_properties(&initial_properties)
        .load_qml(include_bytes!("main.qml"))
        .run();
}
