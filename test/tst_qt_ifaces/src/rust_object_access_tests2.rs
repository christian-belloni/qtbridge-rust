// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#![cfg(test)]
use std::ops::AddAssign;
use std::cell::RefCell;
use std::rc::Rc;
use qtbridge_interfaces::object_access::rust_object_access2::{RustObjAccess2, RustObjAccessError};

// ---------------------------------------------------------------------------
// Construction
// ---------------------------------------------------------------------------

#[test]
fn require_that_new_strong_creates_an_instance() {
    let rc = Rc::new(RefCell::new(0i32));
    let _instance = RustObjAccess2::new_strong(rc);
}

#[test]
fn require_that_new_weak_creates_an_instance() {
    let rc = Rc::new(RefCell::new(0i32));
    let weak = Rc::downgrade(&rc);
    let _instance = RustObjAccess2::new_weak(weak);
}

// ---------------------------------------------------------------------------
// Basic borrow / borrow_mut
// ---------------------------------------------------------------------------

#[test]
fn require_that_try_call_rust_with_handle_invokes_function_with_value_and_succeeds_when_function_is_not_recursive() {
    let rc = Rc::new(RefCell::new(42));
    let instance = RustObjAccess2::new_strong(rc.clone());

    let result = instance.try_call_rust_with_handle(|value| value * 2).unwrap();
    assert_eq!(result, 84);
}

#[test]
fn require_that_try_call_rust_with_handle_mut_invokes_function_and_succeeds_when_function_is_not_recursive() {
    let rc = Rc::new(RefCell::new(42));
    let instance = RustObjAccess2::new_strong(rc.clone());

    assert_eq!(true, instance.try_call_rust_with_handle_mut(|value| *value += 1).is_ok());
    assert_eq!(*rc.borrow(), 43);
}

// ---------------------------------------------------------------------------
// Direct recursion
// ---------------------------------------------------------------------------

// In RustObjAccess2, direct recursion through try_call_rust_with_handle works because
// consume() sets state to None, and RefCell allows multiple immutable borrows.

struct RecursionChecker {
    value: RustObjAccess2<i32>,
}
impl RecursionChecker {
    pub fn new(obj: Rc<RefCell<i32>>) -> Self {
        Self {
            value: RustObjAccess2::new_strong(obj)
        }
    }

    fn calc_product_recursively_borrow_immut(&self, times: u32, init_value: u32) -> u32 {
        self.value.try_call_rust_with_handle(|value| -> u32 {
            let new_value = init_value + *value as u32;
            if times > 1 {
                return self.calc_product_recursively_borrow_immut(times - 1, new_value);
            }
            new_value
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

// In RustObjAccess2, direct mutable recursion is blocked: consume() sets state
// to None, so re-entry tries RefCell::try_borrow_mut() which fails because the
// outer RefMut is still on the stack. This is intentional — it prevents aliasing UB.

#[test]
fn require_that_borrowing_mutably_recursively_fails() {
    let rc = Rc::new(RefCell::new(10));
    let instance = RustObjAccess2::new_strong(rc.clone());
    let result = instance.try_call_rust_with_handle_mut(|value| {
        *value += 1;
        instance.try_call_rust_with_handle_mut(|_| {})
    });
    assert!(result.is_ok());
    let inner = result.unwrap();
    assert!(matches!(inner, Err(RustObjAccessError::BorrowMutError(_))));
}

// Similarly, try_call_rust_with_handle inside try_call_rust_with_handle_mut fails because
// RefCell::try_borrow() fails when a RefMut is active.

#[test]
fn require_that_borrowing_immutably_within_mutable_borrow_fails() {
    let rc = Rc::new(RefCell::new(10));
    let instance = RustObjAccess2::new_strong(rc.clone());
    let result = instance.try_call_rust_with_handle_mut(|_| {
        instance.try_call_rust_with_handle(|_| {})
    });
    assert!(result.is_ok());
    let inner = result.unwrap();
    assert!(matches!(inner, Err(RustObjAccessError::BorrowError(_))));
}

// ---------------------------------------------------------------------------
// Borrow conflicts between try_call_rust_with_handle and try_call_rust_with_handle_mut
// ---------------------------------------------------------------------------

#[test]
fn require_that_try_call_rust_with_handle_mut_within_try_call_rust_with_handle_fails() {
    let rc = Rc::new(RefCell::new(0));
    let instance = RustObjAccess2::new_strong(rc);
    let result = instance.try_call_rust_with_handle(|_| {
        instance.try_call_rust_with_handle_mut(|_| {})
            .expect_err("Expected to be borrowed")
    });
    let error = result.unwrap();
    assert!(matches!(error, RustObjAccessError::BorrowMutError(_)))
}

// ---------------------------------------------------------------------------
// Interaction with original RefCell
// ---------------------------------------------------------------------------

#[test]
fn require_that_try_borrow_original_ref_cell_succeeds_when_called_within_try_call_rust_with_handle() {
    let rc = Rc::new(RefCell::new(36));
    let instance = RustObjAccess2::new_strong(rc.clone());
    instance.try_call_rust_with_handle(|_| {
        let nested_borrow_result = rc.try_borrow();
        assert!(nested_borrow_result.is_ok());
        assert_eq!(*nested_borrow_result.unwrap(), 36);
    }).unwrap();
}

#[test]
fn require_that_try_borrow_from_original_ref_cell_fails_when_called_within_try_call_rust_with_handle_mut() {
    let rc = Rc::new(RefCell::new(37));
    let instance = RustObjAccess2::new_strong(rc.clone());
    instance.try_call_rust_with_handle_mut(|_| {
        let nested_borrow_result = rc.try_borrow();
        assert!(nested_borrow_result.is_err());
    }).unwrap();
}

#[test]
fn require_that_try_borrow_mut_from_original_ref_cell_fails_when_called_within_try_call_rust_with_handle_mut() {
    let rc = Rc::new(RefCell::new(39));
    let instance = RustObjAccess2::new_strong(rc.clone());
    instance.try_call_rust_with_handle_mut(|_| {
        let nested_borrow_result = rc.try_borrow_mut();
        assert!(nested_borrow_result.is_err());
    }).unwrap();
}

#[test]
fn require_that_try_borrow_mut_from_original_ref_cell_fails_when_called_within_try_call_rust_with_handle() {
    let rc = Rc::new(RefCell::new(38));
    let instance = RustObjAccess2::new_strong(rc.clone());
    instance.try_call_rust_with_handle(|_| {
        let nested_borrow_result = rc.try_borrow_mut();
        assert!(nested_borrow_result.is_err());
    }).unwrap();
}

// ---------------------------------------------------------------------------
// External borrow then try_call_rust_with_handle
// ---------------------------------------------------------------------------

#[test]
fn require_that_try_call_rust_with_handle_succeeds_when_called_within_scope_of_borrow_of_original_ref_cell() {
    let rc = Rc::new(RefCell::new(40));
    let ref_ = rc.borrow();
    let instance: RustObjAccess2<i32> = RustObjAccess2::new_strong(rc.clone());
    instance.try_call_rust_with_handle(|value| {
        assert_eq!(*value, 40);
    }).unwrap();
    assert_eq!(*ref_, 40);
}

#[test]
fn require_that_try_call_rust_with_handle_fails_when_called_within_scope_of_borrow_mut_of_original_ref_cell() {
    let rc = Rc::new(RefCell::new(41));
    let mut ref_mut = rc.borrow_mut();
    let instance = RustObjAccess2::new_strong(rc.clone());
    let result = instance.try_call_rust_with_handle(|_| { panic!("Not supposed to be called") });
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RustObjAccessError::BorrowError(_)));
    ref_mut.add_assign(1);
}

#[test]
fn require_that_try_call_rust_with_handle_mut_fails_when_called_within_scope_of_borrow_of_original_ref_cell() {
    let rc = Rc::new(RefCell::new(42));
    let ref_ = rc.borrow();
    let instance = RustObjAccess2::new_strong(rc.clone());
    let result = instance.try_call_rust_with_handle_mut(|_| { panic!("Not supposed to be called") });
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RustObjAccessError::BorrowMutError(_)));
    assert_eq!(*ref_, 42);
}

#[test]
fn require_that_try_call_rust_with_handle_mut_fails_when_called_within_scope_of_borrow_mut_of_original_ref_cell() {
    let rc = Rc::new(RefCell::new(42));
    let mut ref_mut = rc.borrow_mut();
    let instance = RustObjAccess2::new_strong(rc.clone());
    let result = instance.try_call_rust_with_handle_mut(|_| { panic!("Not supposed to be called") });
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), RustObjAccessError::BorrowMutError(_)));
    ref_mut.add_assign(1);
}

// ---------------------------------------------------------------------------
// Weak pointer behavior
// ---------------------------------------------------------------------------

#[test]
fn require_that_try_call_rust_with_handle_holds_original_rc_when_class_is_constructed_with_weak_pointer_and_pointer_is_not_expired_at_the_moment_of_call() {
    let rc = Rc::new(RefCell::new(43));
    let weak = Rc::downgrade(&rc);
    let mut rc = Some(rc);
    let instance = RustObjAccess2::new_weak(weak.clone());
    instance.try_call_rust_with_handle(|_value| {
        rc.take();
        assert_eq!(weak.strong_count(), 1);
    }).unwrap();
}

#[test]
fn require_that_try_call_rust_with_handle_fails_when_class_is_constructed_with_weak_pointer_and_pointer_is_expired_before_call() {
    let rc = Rc::new(RefCell::new(43));
    let weak = Rc::downgrade(&rc);
    let mut rc = Some(rc);
    let instance = RustObjAccess2::new_weak(weak.clone());
    rc.take();
    let result = instance.try_call_rust_with_handle(|_value| {
        panic!("Not supposed to be executed")
    });
    assert!(matches!(result.unwrap_err(), RustObjAccessError::ExpiredWeakPtr));
}

#[test]
fn require_that_try_call_rust_with_handle_mut_holds_original_rc_when_class_is_constructed_with_weak_pointer_and_pointer_is_not_expired_at_the_moment_of_call() {
    let rc = Rc::new(RefCell::new(43));
    let weak = Rc::downgrade(&rc);
    let mut rc = Some(rc);
    let instance = RustObjAccess2::new_weak(weak.clone());
    instance.try_call_rust_with_handle_mut(|_value| {
        rc.take();
        assert_eq!(weak.strong_count(), 1);
    }).unwrap();
}

#[test]
fn require_that_try_call_rust_with_handle_mut_fails_when_class_is_constructed_with_weak_pointer_and_pointer_is_expired_before_call() {
    let rc = Rc::new(RefCell::new(43));
    let weak = Rc::downgrade(&rc);
    let mut rc = Some(rc);
    let instance = RustObjAccess2::new_weak(weak.clone());
    rc.take();
    let result = instance.try_call_rust_with_handle_mut(|_value| {
        panic!("Not supposed to be executed")
    });
    assert!(matches!(result.unwrap_err(), RustObjAccessError::ExpiredWeakPtr));
}

// ---------------------------------------------------------------------------
// store_handle: the C++ re-entry path
//
// In RustObjAccess2, try_store_handle_and_call_cpp[_mut] takes a reference
// to the object and a closure that receives no arguments. The reference is
// stored so that deeper re-entry via try_call_rust_with_handle[_mut] can reconstruct
// the reference from the stored pointer.
// ---------------------------------------------------------------------------

#[test]
fn require_that_try_store_handle_and_call_cpp_succeeds_when_called_with_immutable_reference() {
    let rc = Rc::new(RefCell::new(44));
    let instance = RustObjAccess2::new_strong(rc.clone());
    let b = rc.borrow();
    let result = instance.try_store_handle_and_call_cpp(&*b, || *b + 6);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 50);
}

#[test]
fn require_that_try_store_handle_and_call_cpp_mut_succeeds_when_called_with_mutable_reference() {
    let rc = Rc::new(RefCell::new(45));
    let instance = RustObjAccess2::new_strong(rc.clone());
    {
        let mut b = rc.borrow_mut();
        let result = instance.try_store_handle_and_call_cpp_mut(&mut *b, || {
            // Re-enter to modify through stored handle
            instance.try_call_rust_with_handle_mut(|value| *value += 6).unwrap()
        });
        assert!(result.is_ok());
    }
    assert_eq!(*rc.borrow(), 51);
}

#[test]
fn require_that_try_call_rust_with_handle_succeeds_when_called_after_try_store_handle_and_call_cpp() {
    let rc = Rc::new(RefCell::new(46));
    let instance = RustObjAccess2::new_strong(rc.clone());
    let b = rc.borrow();
    let result = instance.try_store_handle_and_call_cpp(&*b, || {
        instance.try_call_rust_with_handle(|value| value + 8)
    });

    assert!(result.is_ok());
    let inner = result.unwrap();
    assert!(inner.is_ok());
    assert_eq!(inner.unwrap(), 54);
}

#[test]
fn require_that_try_call_rust_with_handle_mut_succeeds_when_called_after_try_store_handle_and_call_cpp_mut() {
    let rc = Rc::new(RefCell::new(47));
    let instance = RustObjAccess2::new_strong(rc.clone());
    {
        let mut b = rc.borrow_mut();
        let result = instance.try_store_handle_and_call_cpp_mut(&mut b, || {
            instance.try_call_rust_with_handle_mut(|value| *value += 8)
        });
        assert!(result.is_ok());
        assert!(result.unwrap().is_ok());
    }
    assert_eq!(*rc.borrow(), 55);
}

// ---------------------------------------------------------------------------
// store_handle: possible conflicts when already stored
// ---------------------------------------------------------------------------

#[test]
fn require_that_try_store_handle_and_call_cpp_succeeds_when_called_while_already_stored() {
    let rc = Rc::new(RefCell::new(48));
    let instance = RustObjAccess2::new_strong(rc.clone());
    let b = rc.borrow();
    let mark_hit = std::cell::Cell::new(false);
    let result = instance.try_store_handle_and_call_cpp(&*b, || {
        instance.try_store_handle_and_call_cpp(&*b, || {
            mark_hit.set(true);
        })
    });
    assert!(result.is_ok());
    assert!(mark_hit.get(), "Inner callback should have been called");
}

// We do a lot of bad stuff that would trigger miri, but just in the test!
#[test]
#[cfg(not(miri))]
fn require_that_try_store_handle_and_call_cpp_mut_fails_when_called_while_already_stored() {
    let rc = Rc::new(RefCell::new(48));
    let instance = RustObjAccess2::new_strong(rc.clone());
    let instance_ptr = &instance as *const _ as *mut RustObjAccess2<i32>;
    let mut b = rc.borrow_mut();
    let b_ptr = &mut *b as *mut i32;
    let result = instance.try_store_handle_and_call_cpp_mut(&mut *b, || {
        let sneaky = unsafe { &*instance_ptr };
        sneaky.try_store_handle_and_call_cpp_mut(unsafe { &mut *b_ptr }, || {
            panic!("Not supposed to be called")
        })
    });
    assert!(result.is_ok());
    let inner = result.unwrap();
    assert!(matches!(inner, Err(RustObjAccessError::BorrowConflict)));
}
