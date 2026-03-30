// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::mem::MaybeUninit;
use std::ptr::{self, NonNull, addr_of_mut};
use std::rc::{Rc, Weak};
use std::cell::{BorrowError, BorrowMutError, Cell, Ref, RefCell, RefMut};

#[macro_export]
macro_rules! call_rust_trait_impl {
    // Mutable version. The mut in front of the self is just a marker
    (mut $self:expr, $method:ident ( $($arg:expr),* )) => {
        $self.rust_obj
            .try_with_borrow_mut(|vtable| {
                vtable.$method($($arg),*)
            })
            .expect(concat!(
                "Failed to borrow mutably for ",
                stringify!($method),
                "()"
            ))
    };

    // Immutable version. Without the mut marker before self
    ($self:expr, $method:ident ( $($arg:expr),* )) => {
        $self.rust_obj
            .try_with_borrow(|vtable| {
                vtable.$method($($arg),*)
            })
            .expect(concat!(
                "Failed to borrow for ",
                stringify!($method),
                "()"
            ))
    };
}

#[macro_export]
macro_rules! call_cpp_impl {
    (mut $self:expr, $method:ident ( $($arg:expr),* )) => {{
        let proxy = unsafe {
            $self.cpp_proxy
                .as_mut()
                .expect("cpp_proxy was null")
        };
        let proxy_pinned = unsafe { std::pin::Pin::new_unchecked(proxy) };
        $self.rust_obj
            .try_store_handle_and_call_qml_mut(|_| proxy_pinned.$method($($arg),*))
            .expect(concat!(
                "Failed to borrow mutably for ",
                stringify!($method),
                "()"
            ))
    }};

    ($self:expr, $method:ident ( $($arg:expr),* )) => {{
        let proxy = unsafe {
            $self.cpp_proxy
                .as_ref()
                .expect("cpp_proxy was null")
        };
        $self.rust_obj
            .try_store_handle_and_call_qml(|_| proxy.$method($($arg),*))
            .expect(concat!(
                "Failed to borrow for ",
                stringify!($method),
                "()"
            ))
    }};
}


/// A structure that provides controlled access to a Rust object by invoking functors
/// (potentially recursively) on the object while it is borrowed immutably or mutably.
///
/// Supports recursive borrowing under the following rules:
/// * If the object is initially borrowed mutably, both mutable and immutable recursive
///   calls are allowed.
/// * If the object is initially borrowed immutably, only immutable recursive calls are allowed.
///
/// The object is held using `Rc<RefCell<T>>` or `Weak<RefCell<T>>`.
///
/// The need for recursive borrowing (even mutable) appears from the separation
/// between Rust struct and C++ class we have to live with. The group of the objects consists of:
/// * the Rust object itself.
/// * the C++ proxy implementing needed C++ interface, forwarding calls to the Rust Proxy.
/// * the Rust proxy that connects C++ proxy and Rust object via 'CXX'.
/// In pure C++ implementation that would typically be a single class inheriting from a needed base class / interface.
/// However, because Rust lacks inheritance, we have to deal with Rust/C++ objects Frankenstein.
///
/// In summary, this struct prevents borrow errors and panics in scenarios involving
/// recursive call chains such as:
///     Rust object -> Base implementation in C++ proxy -> Rust object
/// even though this requires bending standard Rust borrowing principles.
/// From the user’s perspective, borrowing semantics remain equivalent to working with a
/// normal `Rc<RefCell<T>>`.
pub struct RustObjAccess<T: ?Sized + 'static>
{
    shared_reference: SharedReferenceWithQml<T>,
    borrow_handle: Cell<*mut RustObjBorrowHandle<'static, T>>,
}

impl<T: ?Sized> RustObjAccess<T> {
    pub fn new_strong(ptr: Rc<RefCell<T>>) -> Self {
        Self {
            shared_reference: SharedReferenceWithQml::OwnedByRust(ptr),
            borrow_handle: Cell::new(ptr::null_mut()),
        }
    }

    pub fn new_weak(ptr: Weak<RefCell<T>>) -> Self {
        Self {
            shared_reference: SharedReferenceWithQml::OwnedByQml(ptr),
            borrow_handle: Cell::new(ptr::null_mut()),
        }
    }

    pub fn try_with_borrow<F, R>(&self, f: F) -> Result<R, RustObjAccessError>
    where F:FnOnce(&T) -> R
    {
        // We already borrowed and store a reference and can execute on it
        let ptr_to_borrowed = self.borrow_handle.get();
        if let Some(borrowed) = unsafe { ptr_to_borrowed.as_ref() } {
            return Ok(f(borrowed.obj_ref.deref()))
        };

        // Create struct holding a reference on the stack
        let rc = self.shared_reference.get_rc()
            .ok_or(RustObjAccessError::ExpiredWeakPtr)?;
        let mut borrowed = RustObjBorrowHandle::new(rc, false)?;

        // Write the pointer to this object on the stack to self.borrowed
        self.borrow_handle.set(&mut borrowed);
        let result = f(borrowed.obj_ref.deref());
        self.borrow_handle.set(ptr::null_mut());

        Ok(result)
    }

    pub fn try_with_borrow_mut<F, R>(&self, f: F) -> Result<R, RustObjAccessError>
    where F:FnOnce(&mut T) -> R
    {
        let ptr_to_borrowed = self.borrow_handle.get();
        if let Some(borrowed) = unsafe { ptr_to_borrowed.as_mut() } {
            match &mut borrowed.obj_ref {
                RustHandle::Immutable(_) |
                RustHandle::ImmutableUnguardedBorrow(_) => {
                    // Call try_borrow_mut() to get BorrowMutError and return it from the function
                    let _ref = borrowed.obj_rc.try_borrow_mut()
                        .map_err(|err| RustObjAccessError::BorrowMutError(err))?;
                    unreachable!()
                },
                RustHandle::Mutable(_) |
                RustHandle::MutableUnguardedBorrow(_) => {
                    return Ok(f(borrowed.obj_ref.deref_mut().unwrap()))
                }
            }
        }

        let rc = self.shared_reference.get_rc()
            .ok_or(RustObjAccessError::ExpiredWeakPtr)?;
        let mut borrowed = RustObjBorrowHandle::new(rc, true)?;

        self.borrow_handle.set(&mut borrowed);
        let result = f(borrowed.obj_ref.deref_mut().unwrap());
        self.borrow_handle.set(ptr::null_mut());

        Ok(result)
    }

    /// Executes `f` on the inner `T` assuming that a valid `RefCell` borrow
    /// already exists somewhere in the current call stack.
    ///
    /// This is intended for Rust -> C++/QML -> Rust re-entry scenarios:
    /// Rust code borrows from `Rc<RefCell<T>>` and calls into C++/QML code.
    /// That code may call back into Rust. Borrowing the `RefCell` again when
    /// re-entering Rust would normally fail, because the original borrow is
    /// still active.
    ///
    /// To support this case, the function stores access to the object and
    /// invokes `f` using a raw pointer obtained via `Rc::as_ptr()`, bypassing
    /// the `RefCell` borrow mechanism.
    ///
    /// This must only be used when the caller can guarantee that a valid
    /// borrow of the `RefCell` already exists externally. Rust code that
    /// directly accesses the object should use the normal `borrow()` or
    /// `borrow_mut()` APIs instead.
    ///
    /// The function performs runtime checks to validate the assumption that
    /// the borrow originates outside this call.
    pub fn try_store_handle_and_call_qml<F, R>(&self, f: F) -> Result<R, RustObjAccessError>
    where F:FnOnce(&T) -> R
    {
        let ptr_to_borrowed = self.borrow_handle.get();
        if let Some(borrowed) = unsafe { ptr_to_borrowed.as_ref() } {
            return Ok(f(borrowed.obj_ref.deref()))
        }

        let rc = self.shared_reference.get_rc()
            .ok_or(RustObjAccessError::ExpiredWeakPtr)?;

        // Check that object is actually borrowed immutably at the moment.
        // TODO: add #[cfg(debug_assertions)] if these checks as slow.
        {
            // If try_borrow_mut() succeeds - the object is not borrowed.
            match rc.try_borrow_mut() {
                Ok(_) => return Err(RustObjAccessError::ExpectedBorrowed),
                Err(_) => {},
            }

            // If try_borrow() fails - it means that the object is already borrowed mutably.
            // TODO: uncomments the lines below once borrowing in metacalls is in place
            // rc.try_borrow()
            //     .map_err(|_err| RustObjAccessError::ExpectedBorrowed)?;
        }

        let mut borrowed = RustObjBorrowHandle::new_unguarded(rc, false);

        self.borrow_handle.set(&mut borrowed);
        let result = f(borrowed.obj_ref.deref());
        self.borrow_handle.set(ptr::null_mut());

        Ok(result)
    }

    /// Similar to [`try_store_handle_and_call_qml`].
    pub fn try_store_handle_and_call_qml_mut<F, R>(&self, f: F) -> Result<R, RustObjAccessError>
    where F:FnOnce(&mut T) -> R
    {
        let ptr_to_borrowed = self.borrow_handle.get();
        if let Some(borrowed) = unsafe { ptr_to_borrowed.as_mut() } {
             match &mut borrowed.obj_ref {
                RustHandle::Immutable(_) |
                RustHandle::ImmutableUnguardedBorrow(_) => {
                    // Call try_borrow_mut() to get BorrowMutError and return it from the function
                    let _ref = borrowed.obj_rc.try_borrow_mut()
                        .map_err(|err| RustObjAccessError::BorrowMutError(err))?;
                    panic!("Object assumed to be borrowed but it is not")
                },
                RustHandle::Mutable(_) |
                RustHandle::MutableUnguardedBorrow(_) => {
                    return Ok(f(borrowed.obj_ref.deref_mut().unwrap()))
                }
            }
        }

        let rc = self.shared_reference.get_rc()
            .ok_or(RustObjAccessError::ExpiredWeakPtr)?;

        // Check that object is actually borrowed mutably
        // TODO: add #[cfg(debug_assertions)] if these checks as slow.
        {
            // If try_borrow_mut() succeeds - then object is not borrowed.
            // TODO: uncomments lines below once borrowing in metacalls is in place
            // match rc.try_borrow_mut() {
            //     Ok(_) => return Err(RustObjAccessError::ExpectedBorrowedMut),
            //     Err(_) => {},
            // }

            // // If try_borrow() succeeds - then object is borrowed but immutably.
            // match rc.try_borrow() {
            //     Ok(_) => return Err(RustObjAccessError::ExpectedBorrowedMut),
            //     Err(_) => {},
            // }
        }

        let mut borrowed = RustObjBorrowHandle::new_unguarded(rc, true);

        self.borrow_handle.set(&mut borrowed);
        let result = f(borrowed.obj_ref.deref_mut().unwrap());
        self.borrow_handle.set(ptr::null_mut());

        Ok(result)
    }

}

/// Enum containing possible errors that may occur on attempting to borrow object.
#[derive(Debug)]
pub enum RustObjAccessError {
    /// Standard error returned from RefCell::try_borrow()
    BorrowError(BorrowError),
    BorrowMutError(BorrowMutError),
    ExpiredWeakPtr,
    ExpectedBorrowed,
    ExpectedBorrowedMut,
}

impl From<BorrowError> for RustObjAccessError {
    fn from(value: BorrowError) -> Self {
        Self::BorrowError(value)
    }
}

impl From<BorrowMutError> for RustObjAccessError {
    fn from(value: BorrowMutError) -> Self {
        Self::BorrowMutError(value)
    }
}

pub enum SharedReferenceWithQml<T: ?Sized> {
    OwnedByRust(Rc<RefCell<T>>),
    OwnedByQml(Weak<RefCell<T>>),
}

impl<T: ?Sized> SharedReferenceWithQml<T> {
    fn get_rc(&self) -> Option<Rc<RefCell<T>>> {
        match self {
            SharedReferenceWithQml::OwnedByRust(rc) => Some(rc.clone()),
            SharedReferenceWithQml::OwnedByQml(weak) => weak.upgrade()
        }
    }
}

/// Struct with a borrowed object
struct RustObjBorrowHandle<'a, T: ?Sized> {
    /// Strong pointer being held to make sure referenced object is alive while borrowed.
    obj_rc: Rc<RefCell<T>>,

    /// Ref or RefMut obtained by borrowing from RefCell.
    obj_ref: RustHandle<'a, T>,
}

impl<'a, T: ?Sized> RustObjBorrowHandle<'a, T> {
    fn new(rc: Rc<RefCell<T>>, is_mutable: bool) -> Result<Self, RustObjAccessError> {
        let mut uninit: MaybeUninit<Self> = MaybeUninit::uninit();
        let ptr = uninit.as_mut_ptr();
        unsafe {
            addr_of_mut!((*ptr).obj_rc).write(rc.clone());
            addr_of_mut!((*ptr).obj_ref).write(RustHandle::new(&(*ptr).obj_rc, is_mutable)?);
            Ok(uninit.assume_init())
        }
    }

    fn new_unguarded(rc: Rc<RefCell<T>>, is_mutable: bool) -> Self {
        let mut uninit: MaybeUninit<Self> = MaybeUninit::uninit();
        let ptr = uninit.as_mut_ptr();
        unsafe {
            addr_of_mut!((*ptr).obj_rc).write(rc.clone());
            addr_of_mut!((*ptr).obj_ref).write(RustHandle::new_unguarded(&(*ptr).obj_rc, is_mutable));
            uninit.assume_init()
        }
    }
}

/// Mutable or immutable reference to Rust object
/// obtained via RefCell::try_borrow()/try_borrow_mut()
/// or RefCell::as_ptr() if the object was already borrowed externally
enum RustHandle<'a, T: ?Sized> {
    Immutable(Ref<'a, T>),
    Mutable(RefMut<'a, T>),
    ImmutableUnguardedBorrow(NonNull<T>),
    MutableUnguardedBorrow(NonNull<T>),
}

impl<'a, T: ?Sized> RustHandle<'a, T> {
    fn new(rc: &'a Rc<RefCell<T>>, is_mutable: bool) -> Result<Self, RustObjAccessError> {
        match is_mutable {
            true => Ok(Self::Mutable(
                rc.try_borrow_mut()
                    .map_err(RustObjAccessError::from)?
            )),
            false => Ok(Self::Immutable(
                rc.try_borrow()
                    .map_err(RustObjAccessError::from)?
            ))
        }
    }

    fn new_unguarded(rc: &'a Rc<RefCell<T>>, is_mutable: bool) -> Self {
        // This Rc is stored in parent struct
        // so there is no risk of input rc getting out of scope.
        let nn = NonNull::new(rc.as_ptr()).unwrap();
        match is_mutable {
            true => Self::MutableUnguardedBorrow(nn),
            false => Self::ImmutableUnguardedBorrow(nn),
        }
    }

    fn deref(&self) -> &T {
        match self {
            Self::Immutable(ref_) => ref_,
            Self::Mutable(ref_mut) => ref_mut,
            Self::ImmutableUnguardedBorrow(ptr) => unsafe { ptr.as_ref() },
            Self::MutableUnguardedBorrow(ptr) => unsafe {ptr.as_ref() },
        }
    }

    fn deref_mut(&mut self) -> Option<&mut T> {
        match self {
            Self::Immutable(_) => None,
            Self::ImmutableUnguardedBorrow(_) => None,
            Self::Mutable(ref_mut) => Some(ref_mut),
            Self::MutableUnguardedBorrow(ptr) => Some(unsafe {ptr.as_mut() }),
        }
    }
}
