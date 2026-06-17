// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::rc::Rc;
use qtbridge::{QApp, QObjectHolder, QmlRegister, qobject};
use qtbridge::qtbridge_type_lib::QSignalSpy;

#[qobject]
mod test_object {
    #[derive(Default)]
    pub struct TestObject {
    }

    impl TestObject {
    }
}

use test_object::TestObject;

fn default_with_attached_qobject_creates_rc_with_a_single_strong_reference() {
    // Some very simple qml
    let dummy_qml = r#"
        import QtQuick
        import tst_ownership

        Item {
            required property TestObject testObject;
        }
    "#;

    TestObject::register();

    let obj_strong = TestObject::default_with_attached_qobject();
    let obj_weak = Rc::downgrade(&obj_strong);
    let obj_var = obj_strong.borrow().as_qvariant();

    assert_eq!(1, Rc::strong_count(&obj_strong));

    // Load app with dummy qml to make sure that qml does not hold strong reference
    let mut qapp = QApp::new();
    qapp.add_initial_property("testObject", &obj_var)
        .load_qml(dummy_qml.as_bytes());

    drop(obj_strong);
    assert!(obj_weak.upgrade().is_none());
}

fn drop_test_object_call_qobject_destroy() {
    let obj = TestObject::default_with_attached_qobject();
    let spy = QSignalSpy::new(
        unsafe { &*obj.borrow().get_qobject_ptr() },
        "destroyed"
    );

    assert_eq!(spy.count(), 0);
    drop(obj);
    assert_eq!(spy.count(), 1);
}

fn main() {
    default_with_attached_qobject_creates_rc_with_a_single_strong_reference();
    drop_test_object_call_qobject_destroy();
}
