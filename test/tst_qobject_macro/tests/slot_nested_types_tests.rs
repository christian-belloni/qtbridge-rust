// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]
use qtbridge::{QApp, QObjectHolder};
use qtbridge::{QmlRegister, qobject};

#[qobject]
pub mod value {
    use std::rc::Rc;
    use std::cell::RefCell;

    #[derive(Default)]
    pub struct Value {
        pub internal: i32
    }
    impl Value {
        qproperty!("internal", Member = internal);

        #[qslot]
        fn copy_internal(&mut self, new: Rc<RefCell<Value>>) {
            self.internal = new.borrow().internal;
        }
    }
}
pub use value::Value;

fn slot_with_nested_type_can_be_called_from_qml_when_nested_type_is_self() {
    let qml = r#"
        import QtQuick
        import tst_qobject_macro

        Item {
            required property Value target
            required property Value source

            Component.onCompleted: target.copy_internal(source)
        }
    "#;

    Value::register();

    let target = Value::default_with_attached_qobject();
    target.borrow_mut().internal = 6;

    let source = Value::default_with_attached_qobject();
    source.borrow_mut().internal = 42;

    let target_var = target.borrow().as_qvariant();
    let source_var = source.borrow().as_qvariant();

     QApp::new()
        .add_initial_property("target", &target_var)
        .add_initial_property("source", &source_var)
        .load_qml(qml.as_bytes());

    // The slot should have set target.internal = source.internal = 42.
    // This also confirms the Rc<RefCell<Value>> can still be borrowed here.
    assert_eq!(target.borrow().internal, 42);
}


#[qobject(Singleton)]
pub mod backend {
    #[derive(Default)]
    pub struct Backend {
        pub value_internal: i32,
    }
    impl Backend {
        #[qslot]
        fn copy_value_internal(&mut self, new: &std::rc::Rc<std::cell::RefCell<super::Value>>) {
            self.value_internal = new.borrow().internal
        }
    }
}

pub use backend::Backend;

fn slot_with_nested_type_can_be_called_from_qml_when_nested_type_is_not_self() {
    let qml = r#"
        import QtQuick
        import tst_qobject_macro

        Item {
            id: root
            required property var backend

            Component.onCompleted: {
                let val = Qt.createQmlObject(
                   `import tst_qobject_macro
                    Value {
                        internal: 42
                    }`,
                    root);
                backend.copy_value_internal(val)
            }
        }
    "#;

    Value::register();

    let backend = Backend::default_with_attached_qobject();
    let backend_var = backend.borrow().as_qvariant();

    QApp::new()
        .add_initial_property("backend", &backend_var)
        .load_qml(qml.as_bytes());
    assert_eq!(backend.borrow().value_internal, 42);

}

fn main() {
    slot_with_nested_type_can_be_called_from_qml_when_nested_type_is_self();
    slot_with_nested_type_can_be_called_from_qml_when_nested_type_is_not_self();
}
