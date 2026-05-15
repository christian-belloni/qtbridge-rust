// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]

use qtbridge::{QObjectHolder, qobject};
use qtbridge::qtbridge_runtime::DispatchMetaCall;
use qtbridge::qtbridge_type_lib::QVariant;


#[qobject]
pub mod test_object {
    #[derive(Default)]
    pub struct TestObject {
        prop: i32,
    }

    impl TestObject {
        qproperty!("prop", Member = prop);

        #[qslot]
        fn some_slot(&self) {
        }
    }
}

use test_object::TestObject;

#[test]
#[cfg(not(miri))]
#[should_panic]
fn invoke_slot_panics_on_invalid_slot_id() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().invoke_slot(0, &[], &[]);
}

#[test]
#[cfg(not(miri))]
#[should_panic]
fn read_property_panics_on_invalid_property_id() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow().read_property(0);
}

#[test]
#[cfg(not(miri))]
#[should_panic]
fn write_property_panics_on_invalid_property_id() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().write_property(0, &QVariant::default());
}
