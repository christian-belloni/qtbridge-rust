// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#![cfg(test)]
use std::ops::AddAssign;
use std::cell::RefCell;
use std::rc::Rc;
use qt_ifaces::object_access::rust_object_access::{RustObjAccess, RustObjAccessError};

#[test]
fn require_that_new_strong_creates_an_instance() {
    let rc = Rc::new(RefCell::new(0i32));
    let _instance = RustObjAccess::new_strong(rc);
}

#[test]
fn require_that_new_weak_creates_an_instance() {
    let rc = Rc::new(RefCell::new(0i32));
    let weak = Rc::downgrade(&rc);
    let _instance = RustObjAccess::new_weak(weak);
}

#[test]
fn require_that_try_with_borrow_invokes_function_with_value_and_succeeds_when_function_is_not_recursive() {
    let rc = Rc::new(RefCell::new(42));
    let instance = RustObjAccess::new_strong(rc.clone());

    let result = instance.try_with_borrow(|value| value * 2).unwrap();
    assert_eq!(result, 84);
}

#[test]
fn require_that_try_with_borrow_mut_invokes_function_and_succeeds_when_function_is_not_recursive() {
    let rc = Rc::new(RefCell::new(42));
    let instance = RustObjAccess::new_strong(rc.clone());

    assert_eq!(true, instance.try_with_borrow_mut(|value| *value += 1).is_ok());
    assert_eq!(*rc.borrow(), 43);
}


struct RecursionChecker {
    value: RustObjAccess<i32>,
}
impl<'a> RecursionChecker {
    pub fn new(obj: Rc<RefCell<i32>>) -> Self {
        Self {
            value: RustObjAccess::new_strong(obj)
        }
    }

    fn get_value(&self) -> i32 {
        self.value.try_with_borrow(|value| *value)
            .unwrap()
    }

    fn calc_product_recursively_borrow_immut(&self, times: u32, init_value: u32) -> u32 {
        self.value.try_with_borrow(|value| -> u32 {
            let new_value = init_value + *value as u32;
            if times > 1 {
                return self.calc_product_recursively_borrow_immut(times - 1, new_value);
            }
            new_value
        }).unwrap()
    }

    fn increment_recursively_borrow_mut(&self, times: u32) {
        self.value.try_with_borrow_mut(|value| {
            *value += 1;
            if times > 1 {
                self.increment_recursively_borrow_mut(times - 1);
            }
        }).unwrap()
    }

    fn increment_recursively_borrow_mut_and_immut(&self, times: u32) {
        // Function similar to the previous one but get value indirectly via immutable borrowing
        self.value.try_with_borrow_mut(|value| {
            *value = self.get_value() + 1;
            if times > 1 {
                self.increment_recursively_borrow_mut_and_immut(times - 1);
            }
        }).unwrap()
    }
}

#[test]
fn require_that_borrowing_immutably_recursively_succeeds() {
    let rc = Rc::new(RefCell::new(10));
    let checker = RecursionChecker::new(rc.clone());
    let result = checker.calc_product_recursively_borrow_immut(4, 0);
    assert_eq!(result, 40)
}

#[test]
fn require_that_borrowing_mutably_recursively_succeeds() {
    let rc = Rc::new(RefCell::new(10));
    let checker = RecursionChecker::new(rc.clone());
    checker.increment_recursively_borrow_mut(7);
    assert_eq!(*rc.borrow(), 17);
}

#[test]
fn require_that_borrow_mutably_and_immutably_recursively_succeeds_if_initially_borrowed_mutably() {
    let rc = Rc::new(RefCell::new(20));
    let checker = RecursionChecker::new(rc.clone());
    checker.increment_recursively_borrow_mut_and_immut(8);
    assert_eq!(*rc.borrow(), 28);
}

#[test]
fn require_that_try_with_borrow_mut_within_try_with_borrow_fails() {
    let rc = Rc::new(RefCell::new(0));
    let instance = RustObjAccess::new_strong(rc);
    let result = instance.try_with_borrow(|_| {
        instance.try_with_borrow_mut(|_| {})
            .expect_err("Expected to be borrowed")
    });
    let error = result.unwrap();
    assert!(matches!(error, RustObjAccessError::BorrowMutError(_)))
}

#[test]
fn require_that_try_borrow_original_ref_cell_succeeds_when_called_within_try_with_borrow() {
    let rc = Rc::new(RefCell::new(36));
    let instance = RustObjAccess::new_strong(rc.clone());
    instance.try_with_borrow(|_| {
        let nested_borrow_result = rc.try_borrow();
        assert!(nested_borrow_result.is_ok());
        assert_eq!(*nested_borrow_result.unwrap(), 36);
    }).unwrap();
}

#[test]
fn require_that_try_borrow_from_original_ref_cell_fails_when_called_within_try_with_borrow_mut() {
    let rc = Rc::new(RefCell::new(37));
    let instance = RustObjAccess::new_strong(rc.clone());
    instance.try_with_borrow_mut(|_| {
        let nested_borrow_result = rc.try_borrow();
        assert!(nested_borrow_result.is_err());
    }).unwrap();
}

#[test]
fn require_that_try_borrow_mut_from_original_ref_cell_fails_when_called_within_try_with_borrow_mut() {
    let rc = Rc::new(RefCell::new(39));
    let instance = RustObjAccess::new_strong(rc.clone());
    instance.try_with_borrow_mut(|_| {
        let nested_borrow_result = rc.try_borrow_mut();
        assert!(nested_borrow_result.is_err());
    }).unwrap();
}

#[test]
fn require_that_try_borrow_mut_from_original_ref_cell_fails_when_called_within_try_with_borrow() {
    let rc = Rc::new(RefCell::new(38));
    let instance = RustObjAccess::new_strong(rc.clone());
    instance.try_with_borrow(|_| {
        let nested_borrow_result = rc.try_borrow_mut();
        assert!(nested_borrow_result.is_err());
    }).unwrap();
}

#[test]
fn require_that_try_with_borrow_succeeds_when_called_within_scope_of_borrow_of_original_ref_cell() {
    let rc = Rc::new(RefCell::new(40));
    let ref_ = rc.borrow();
    let instance = RustObjAccess::new_strong(rc.clone());
    instance.try_with_borrow(|value| {
        assert_eq!(*value, 40);
    }).unwrap();
    assert_eq!(*ref_, 40);
}

#[test]
fn require_that_try_with_borrow_fails_when_called_within_scope_of_borrow_mut_of_original_ref_cell() {
    let rc = Rc::new(RefCell::new(41));
    let mut ref_mut = rc.borrow_mut();
    let instance = RustObjAccess::new_strong(rc.clone());
    let result = instance.try_with_borrow(|_| { panic!("Not supposed to be called") });
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RustObjAccessError::BorrowError(_)));
    ref_mut.add_assign(1); // Do something with original ref_mut to make sure it outlives the line with try_with_borrow()
}

#[test]
fn require_that_try_with_borrow_mut_fails_when_called_within_scope_of_borrow_of_original_ref_cell() {
    let rc = Rc::new(RefCell::new(42));
    let ref_ = rc.borrow();
    let instance = RustObjAccess::new_strong(rc.clone());
    let result = instance.try_with_borrow_mut(|_| { panic!("Not supposed to be called") });
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RustObjAccessError::BorrowMutError(_)));
    assert_eq!(*ref_, 42);
}

#[test]
fn require_that_try_with_borrow_mut_fails_when_called_within_scope_of_borrow_mut_of_original_ref_cell() {
    let rc = Rc::new(RefCell::new(42));
    let mut ref_mut = rc.borrow_mut();
    let instance = RustObjAccess::new_strong(rc.clone());
    let result = instance.try_with_borrow_mut(|_| { panic!("Not supposed to be called") });
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RustObjAccessError::BorrowMutError(_)));
    ref_mut.add_assign(1);
}

#[test]
fn require_that_try_with_borrow_holds_original_rc_when_class_is_constructed_with_weak_pointer_and_pointer_is_not_expired_at_the_moment_of_call() {
    let rc = Rc::new(RefCell::new(43));
    let weak = Rc::downgrade(&rc);
    let mut rc = Some(rc);
    let instance = RustObjAccess::new_weak(weak.clone());
    instance.try_with_borrow(|_value| {
        rc.take();
        assert_eq!(weak.strong_count(), 1);
    }).unwrap();
}

#[test]
fn require_that_try_with_borrow_fails_when_class_is_constructed_with_weak_pointer_and_pointer_is_expired_before_call() {
    let rc = Rc::new(RefCell::new(43));
    let weak = Rc::downgrade(&rc);
    let mut rc = Some(rc);
    let instance = RustObjAccess::new_weak(weak.clone());
    rc.take();
    let result = instance.try_with_borrow(|_value| {
        panic!("Not supposed to be executed")
    });
    assert!(matches!(result.unwrap_err(), RustObjAccessError::ExpiredWeakPtr));
}

#[test]
fn require_that_try_with_borrow_mut_holds_original_rc_when_class_is_constructed_with_weak_pointer_and_pointer_is_not_expired_at_the_moment_of_call() {
    let rc = Rc::new(RefCell::new(43));
    let weak = Rc::downgrade(&rc);
    let mut rc = Some(rc);
    let instance = RustObjAccess::new_weak(weak.clone());
    instance.try_with_borrow_mut(|_value| {
        rc.take();
        assert_eq!(weak.strong_count(), 1);
    }).unwrap();
}

#[test]
fn require_that_try_with_borrow_mut_fails_when_class_is_constructed_with_weak_pointer_and_pointer_is_expired_before_call() {
    let rc = Rc::new(RefCell::new(43));
    let weak = Rc::downgrade(&rc);
    let mut rc = Some(rc);
    let instance = RustObjAccess::new_weak(weak.clone());
    rc.take();
    let result = instance.try_with_borrow_mut(|_value| {
        panic!("Not supposed to be executed")
    });
    assert!(matches!(result.unwrap_err(), RustObjAccessError::ExpiredWeakPtr));
}


#[test]
fn require_that_try_store_handle_and_call_qml_succeeds_when_called_on_object_borrowed_immutably() {
    let rc = Rc::new(RefCell::new(44));
    let instance = RustObjAccess::new_strong(rc.clone());
    let _b = rc.borrow();
    let result = instance.try_store_handle_and_call_qml(|value| value + 6);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 50);
}

#[test]
fn require_that_try_store_handle_and_call_qml_mut_succeeds_when_called_on_object_borrowed_mutably() {
    let rc = Rc::new(RefCell::new(45));
    let instance = RustObjAccess::new_strong(rc.clone());
    let result;
    {
        let _b = rc.borrow_mut();
        result = instance.try_store_handle_and_call_qml_mut(|value| *value += 6);
    }
    assert!(result.is_ok());
    assert_eq!(*rc.borrow(), 51);
}

#[test]
fn require_that_try_with_borrow_succeeds_when_called_after_try_store_handle_and_call_qml_on_object_borrowed_immutably() {
    let rc = Rc::new(RefCell::new(46));
    let instance = RustObjAccess::new_strong(rc.clone());
    let _b = rc.borrow();
    let result = instance.try_store_handle_and_call_qml(|_value| {
        instance.try_with_borrow(|value2| value2 + 8)
    });

    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 54);
}

#[test]
fn require_that_try_with_borrow_mut_succeeds_when_called_after_try_store_handle_and_call_qml_mut_on_object_borrowed_mutably() {
    let rc = Rc::new(RefCell::new(47));
    let instance = RustObjAccess::new_strong(rc.clone());
    let result;
    {
        let _b = rc.borrow_mut();
        result = instance.try_store_handle_and_call_qml_mut(|_value| {
            instance.try_with_borrow_mut(|value2| *value2 += 8)
        });
    }
    assert!(result.is_ok());
    assert!(result.unwrap().is_ok());
    assert_eq!(*rc.borrow(), 55);
}

#[test]
fn require_that_try_store_handle_and_call_qml_fails_when_called_on_object_not_borrowed() {
    let rc = Rc::new(RefCell::new(48));
    let instance = RustObjAccess::new_strong(rc.clone());
    let result = instance.try_store_handle_and_call_qml(|_value| { panic!("Not supposed to be called") });
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RustObjAccessError::ExpectedBorrowed));
}

// TODO: uncomments the test below once borrowing in metacalls is in place
// #[test]
// fn require_that_try_store_handle_and_call_qml_mut_fails_when_called_on_object_not_borrowed() {
//     let rc = Rc::new(RefCell::new(49));
//     let instance = RustObjAccess::new_strong(rc.clone());
//     let result = instance.try_store_handle_and_call_qml_mut(|_value| { panic!("Not supposed to be called") });
//     assert!(result.is_err());
//     assert!(matches!(result.unwrap_err(), RustObjAccessError::ExpectedBorrowedMut));
// }

// TODO: uncomments the test below once borrowing in metacalls is in place
// #[test]
// fn require_that_try_store_handle_and_call_qml_fails_when_called_on_object_borrowed_mutably_before_the_call() {
//     let rc = Rc::new(RefCell::new(50));
//     let instance = RustObjAccess::new_strong(rc.clone());
//     let _b = rc.borrow_mut();
//     let result = instance.try_store_handle_and_call_qml(|_value| { panic!("Not supposed to be called") });
//     assert!(result.is_err());
//     assert!(matches!(result.unwrap_err(), RustObjAccessError::ExpectedBorrowed));
// }

// TODO: uncomments the test below once borrowing in metacalls is in place
// #[test]
// fn require_that_try_store_handle_and_call_qml_mut_fails_when_called_on_object_borrowed_immutably_before_the_call() {
//     let rc = Rc::new(RefCell::new(51));
//     let instance = RustObjAccess::new_strong(rc.clone());
//     let _b = rc.borrow();
//     let result = instance.try_store_handle_and_call_qml_mut(|_value| { panic!("Not supposed to be called") });
//     assert!(result.is_err());
//     assert!(matches!(result.unwrap_err(), RustObjAccessError::ExpectedBorrowedMut));
// }
