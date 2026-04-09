// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]

use qtbridge::{qobject, QObjectHolder};
use qtbridge::qtbridge_type_lib::{QSignalSpy, QVariantList};

#[qobject]
pub mod test_object {
    #[derive(Default)]
    pub struct TestObject {
        pub mutable_slot_called: bool,
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
    }
}

pub use test_object::TestObject;

#[test]
fn invoke_method_returns_true_when_object_is_alive() {
    let qobject_holder = TestObject::default_with_attached_qobject();
    assert!(qobject_holder.borrow().get_qml_method_invoker().invoke_method("signalNoArgs"));
}

#[test]
fn invoke_method_returns_false_after_object_is_destroyed() {
    let qobject_holder = TestObject::default_with_attached_qobject();
    let qml_method_invoker = qobject_holder.borrow().get_qml_method_invoker();
    qobject_holder.borrow().detach_qobject();
    assert!(!qml_method_invoker.invoke_method("signalNoArgs"));
}

#[test]
fn signal_is_emitted_when_invoke_method_is_called() {
    let qobject_holder = TestObject::default_with_attached_qobject();
    let spy = QSignalSpy::new(qobject_holder.borrow().get_qobject(), "signalNoArgs");
    qobject_holder.borrow().get_qml_method_invoker().invoke_method("signalNoArgs");
    assert_eq!(spy.count(), 1);
}

#[test]
fn mutable_slot_is_called_via_invoke_method() {
    let qobject_holder = TestObject::default_with_attached_qobject();
    let qml_method_invoker = qobject_holder.borrow().get_qml_method_invoker();
    qml_method_invoker.invoke_method("mutableSlot");
    assert!(qobject_holder.borrow().mutable_slot_called);
}

#[test]
fn immutable_slot_is_called_via_invoke_method() {
    let qobject_holder = TestObject::default_with_attached_qobject();
    let spy = QSignalSpy::new(qobject_holder.borrow().get_qobject(), "signalNoArgs");
    let qml_method_invoker = qobject_holder.borrow().get_qml_method_invoker();
    qml_method_invoker.invoke_method("immutableSlot");
    assert_eq!(spy.count(), 1);
}

