// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]

use qtbridge_type_lib::{QGuiApplication, QVariantList};
use qtbridge::{qobject, QObjectHolder};
use qtbridge::qtbridge_type_lib::{QSignalSpy};
use qtbridge::invoke_method;
#[qobject]
pub mod test_object {
    #[derive(Default)]
    pub struct TestObject {
        pub mutable_slot_called: bool,
        pub int_value: i32,
    }

    impl TestObject {
        #[qsignal]
        pub fn signal_no_args(&self);

        #[qslot]
        fn mutable_slot(&mut self) {
            self.mutable_slot_called = true;
        }

        #[qslot]
        fn immutable_slot(&self) {
            self.signal_no_args();
        }

        #[qslot]
        pub fn set_int(&mut self, value: i32) {
            self.int_value = value;
        }

        #[qslot]
        pub fn add_ints(&mut self, a: i32, b: i32) {
            self.int_value = a + b;
        }
    }
}

pub use test_object::TestObject;

#[test]
fn qml_method_invoker_tests() {
    let app = QGuiApplication::new();

    // invoke_method returns true when object is alive
    let qobject_holder = TestObject::default_with_attached_qobject();
    assert!(qobject_holder.borrow().get_qml_method_invoker().invoke_method("signalNoArgs"));

    // invoke_method returns false after object is destroyed
    let qobject_holder = TestObject::default_with_attached_qobject();
    let qml_method_invoker = qobject_holder.borrow().get_qml_method_invoker();
    qobject_holder.borrow().detach_qobject();
    assert!(!qml_method_invoker.invoke_method("signalNoArgs"));

    // signal is emitted when invoke_method is called
    let qobject_holder = TestObject::default_with_attached_qobject();
    let spy = QSignalSpy::new(qobject_holder.borrow().get_qobject(), "signalNoArgs");
    qobject_holder.borrow().get_qml_method_invoker().invoke_method("signalNoArgs");
    app.process_events();
    app.process_events();
    assert_eq!(spy.count(), 1);

    // mutable slot is called via invoke_method
    let qobject_holder = TestObject::default_with_attached_qobject();
    let qml_method_invoker = qobject_holder.borrow().get_qml_method_invoker();
    qml_method_invoker.invoke_method("mutableSlot");
    app.process_events();
    app.process_events();
    assert!(qobject_holder.borrow().mutable_slot_called);

    // immutable slot is called via invoke_method
    let qobject_holder = TestObject::default_with_attached_qobject();
    let spy = QSignalSpy::new(qobject_holder.borrow().get_qobject(), "signalNoArgs");
    let qml_method_invoker = qobject_holder.borrow().get_qml_method_invoker();
    qml_method_invoker.invoke_method("immutableSlot");
    app.process_events();
    app.process_events();
    assert_eq!(spy.count(), 1);

    // slot with parameters
    let qobject_holder = TestObject::default_with_attached_qobject();
    let invoker = qobject_holder.borrow().get_qml_method_invoker();
    assert!(invoker.invoke_method_with_args("addInts", &QVariantList::from([15.into(), 17.into()])));
    app.process_events();
    app.process_events();
    assert_eq!(qobject_holder.borrow().int_value, 32);

    // immutable slot is called via macro
    let qobject_holder = TestObject::default_with_attached_qobject();
    let spy = QSignalSpy::new(qobject_holder.borrow().get_qobject(), "signalNoArgs");
    let invoker = qobject_holder.borrow().get_qml_method_invoker();
    invoke_method!(invoker, "immutableSlot");
    app.process_events();
    app.process_events();
    assert_eq!(spy.count(), 1);

    // slot with parameters via macro
    let qobject_holder = TestObject::default_with_attached_qobject();
    let invoker = qobject_holder.borrow().get_qml_method_invoker();
    invoke_method!(invoker, "addInts", 15, 17);
    app.process_events();
    app.process_events();
    assert_eq!(qobject_holder.borrow().int_value, 32);
}
