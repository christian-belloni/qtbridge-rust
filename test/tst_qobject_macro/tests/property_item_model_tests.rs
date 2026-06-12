// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]

use std::cell::RefCell;
use std::rc::Rc;
use qtbridge::{QApp, QListModel, QObjectHolder, qobject};

#[qobject(Base = QListModel)]
pub mod model {
    use super::QListModel;

    #[derive(Default)]
    pub struct Model {
        lines: Vec<String>,
    }

    impl Model {
        #[qslot]
        pub fn populate_items(&mut self, count: usize) {
            self.lines = (0..count)
                .map(|n| n.to_string())
                .collect();
        }
    }

    impl QListModel for Model {
        type Item = String;

        fn len(&self) -> usize {
            self.lines.len()
        }

        fn get(&self, index: usize) -> Option<&String> {
            self.lines.get(index)
        }
    }
}
pub use model::Model;

#[qobject]
pub mod dual_model {
    use super::{Model, Rc, RefCell};

    #[derive(Default)]
    pub struct DualModel {
        left: Rc<RefCell<Model>>,
        right: Rc<RefCell<Model>>,
    }

    impl DualModel {
        qproperty!("left", Read = get_left, Write = set_left);
        qproperty!("right", Read = get_right, Write = set_right);

        #[qsignal]
        fn left_changed(&mut self);

        #[qsignal]
        fn right_changed(&mut self);

        pub fn get_left(&self) -> Rc<RefCell<Model>> {
            self.left.clone()
        }

        pub fn get_right(&self) -> Rc<RefCell<Model>> {
            self.right.clone()
        }

        pub fn set_left(&mut self, new: Rc<RefCell<Model>>) {
            self.left = new;
            self.left_changed();
        }

        pub fn set_right(&mut self, new: Rc<RefCell<Model>>) {
            self.right = new;
            self.right_changed();
        }
    }
}
pub use dual_model::DualModel;

fn item_model_can_be_assigned_to_property() {
    let qml = r#"
        import QtQuick
        import tst_qobject_macro

        Item {
            id: root
            required property DualModel dualModel

            function createModel(numItems) {
                let model = Qt.createQmlObject(
                    `import tst_qobject_macro
                     Model { }`,
                    root

                )
                model.populate_items(numItems)
                return model
            }

            Component.onCompleted: {
                dualModel.left = createModel(3);
                dualModel.right = createModel(4);
            }
        }
    "#;

    let dual_model = DualModel::default_with_attached_qobject();
    let dual_model_var = dual_model.borrow().as_qvariant();

    QApp::new()
        .add_initial_property("dualModel", &dual_model_var)
        .register::<DualModel>()
        .register::<Model>()
        .load_qml(qml.as_bytes());

    assert_eq!(dual_model.borrow().get_left().borrow().len(), 3);
    assert_eq!(dual_model.borrow().get_right().borrow().len(), 4);
}

#[qobject(Singleton)]
mod test_values {
    #[derive(Default)]
    pub struct TestValues {
        pub left_size: usize,
        pub right_size: usize,
        pub view_count: usize,
    }

    impl TestValues {
        qproperty!("leftSize", Member = left_size);
        qproperty!("rightSize", Member = right_size);
        qproperty!("viewCount", Member = view_count);
    }
}
use test_values::TestValues;

fn item_model_can_be_read_from_property() {
    let dual_model = DualModel::default_with_attached_qobject();
    let dual_model_var = dual_model.borrow().as_qvariant();

    let test_values = TestValues::default_with_attached_qobject();
    let test_values_var = test_values.borrow().as_qvariant();


    let qml = r#"
        import QtQuick
        import tst_qobject_macro

        Item {
            id: root
            required property var dualModel
            required property var testValues

            Connections {
                target: dualModel
                function onLeft_changed() {
                    testValues.leftSize = dualModel.left.rowCount()
                }

                function onRight_changed() {
                    testValues.rightSize = dualModel.right.rowCount()
                }
            }
        }
    "#;

    let mut app = QApp::new();
    app.add_initial_property("dualModel", &dual_model_var)
        .add_initial_property("testValues", &test_values_var)
        .register::<DualModel>()
        .register::<TestValues>()
        .load_qml(qml.as_bytes());

    let new_left = Model::default_with_attached_qobject();
    new_left.borrow_mut().populate_items(5);
    dual_model.borrow_mut().set_left(new_left);

    let new_right = Model::default_with_attached_qobject();
    new_right.borrow_mut().populate_items(7);
    dual_model.borrow_mut().set_right(new_right);

    assert_eq!(test_values.borrow().left_size, 5);
    assert_eq!(test_values.borrow().right_size, 7);
}

fn item_model_is_compatible_with_view() {
    let qml = r#"
        import QtQuick
        import tst_qobject_macro

        Item {
            id: root
            required property Model model
            required property TestValues testValues

            ListView {
                id: view
                model: root.model
                delegate: Item {}
            }

            Component.onCompleted: {
                testValues.viewCount = view.count
            }
        }
    "#;

    let model = Model::default_with_attached_qobject();
    model.borrow_mut().populate_items(6);
    let model_var = model.borrow().as_qvariant();

    let test_values = TestValues::default_with_attached_qobject();
    let test_values_var = test_values.borrow().as_qvariant();

    QApp::new()
        .add_initial_property("model", &model_var)
        .add_initial_property("testValues", &test_values_var)
        .register::<Model>()
        .register::<TestValues>()
        .load_qml(qml.as_bytes());

    assert_eq!(test_values.borrow().view_count, 6);
}

fn main() {
    item_model_can_be_assigned_to_property();
    item_model_can_be_read_from_property();
    item_model_is_compatible_with_view();
}
