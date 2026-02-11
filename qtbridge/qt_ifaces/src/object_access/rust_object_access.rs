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
                vtable.get_trait_mut().$method($($arg),*)
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
                vtable.get_trait().$method($($arg),*)
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
            .try_with_assuming_borrowed_mut(|_| proxy_pinned.$method($($arg),*))
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
            .try_with_assuming_borrowed(|_| proxy.$method($($arg),*))
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
    /// Pointer to Rust object we provide access to
    obj_ptr: RustObjPtr<T>,

    /// Reference to borrowed object
    borrowed: Cell<*mut RustObjBorrowed<'static, T>>,
}

impl<T: ?Sized> RustObjAccess<T> {
    pub fn new_strong(ptr: Rc<RefCell<T>>) -> Self {
        Self {
            obj_ptr: RustObjPtr::Strong(ptr),
            borrowed: Cell::new(ptr::null_mut()),
        }
    }

    pub fn new_weak(ptr: Weak<RefCell<T>>) -> Self {
        Self {
            obj_ptr: RustObjPtr::Weak(ptr),
            borrowed: Cell::new(ptr::null_mut()),
        }
    }

    pub fn try_with_borrow<F, R>(&self, f: F) -> Result<R, RustObjAccessError>
    where F:FnOnce(&T) -> R
    {
        let ptr_to_borrowed = self.borrowed.get();
        if let Some(borrowed) = unsafe { ptr_to_borrowed.as_ref() } {
            return Ok(f(borrowed.obj_ref.deref()))
        };

        // Create struct holding reference on the stack
        let rc = self.obj_ptr.get_rc()
            .ok_or(RustObjAccessError::ExpiredWeakPtr)?;
        let mut borrowed = RustObjBorrowed::new(rc, false)?;

        // Write the pointer to this object on the stack to self.borrowed
        self.borrowed.set(&mut borrowed);
        let result = f(borrowed.obj_ref.deref());
        self.borrowed.set(ptr::null_mut());

        Ok(result)
    }

    pub fn try_with_borrow_mut<F, R>(&self, f: F) -> Result<R, RustObjAccessError>
    where F:FnOnce(&mut T) -> R
    {
        let ptr_to_borrowed = self.borrowed.get();
        if let Some(borrowed) = unsafe { ptr_to_borrowed.as_mut() } {
            match &mut borrowed.obj_ref {
                RustObjRef::Immutable(_) |
                RustObjRef::ImmutableBorrowedExternally(_) => {
                    // Call try_borrow_mut() to get BorrowMutError and return it from the function
                    let _ref = borrowed.obj_rc.try_borrow_mut()
                        .map_err(|err| RustObjAccessError::BorrowMutError(err))?;
                    unreachable!()
                },
                RustObjRef::Mutable(_) |
                RustObjRef::MutableBorrowedExternally(_) => {
                    return Ok(f(borrowed.obj_ref.deref_mut().unwrap()))
                }
            }
        }

        let rc = self.obj_ptr.get_rc()
            .ok_or(RustObjAccessError::ExpiredWeakPtr)?;
        let mut borrowed = RustObjBorrowed::new(rc, true)?;

        self.borrowed.set(&mut borrowed);
        let result = f(borrowed.obj_ref.deref_mut().unwrap());
        self.borrowed.set(ptr::null_mut());

        Ok(result)
    }

    /// Try to execute functor on `Rc<RefCell<T>>` assuming that it was already borrowed externally (not inside this struct).
    /// This might be the case when borrowing happens in the user code via borrowing functions of `Rc<RefCell<T>>` that we share with the user.
    /// This function does not actually borrow, but remembers that the borrowing already occurred in the caller code outside of this call.
    /// Performs checks to verify the assumption that the borrowing was done externally.
    /// Access to the target object is performed via a raw pointer obtained from `Rc::as_ptr()`.
    pub fn try_with_assuming_borrowed<F, R>(&self, f: F) -> Result<R, RustObjAccessError>
    where F:FnOnce(&T) -> R
    {
        let ptr_to_borrowed = self.borrowed.get();
        if let Some(borrowed) = unsafe { ptr_to_borrowed.as_ref() } {
            return Ok(f(borrowed.obj_ref.deref()))
        }

        let rc = self.obj_ptr.get_rc()
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

        let mut borrowed = RustObjBorrowed::new_borrowed_externally(rc, false);

        self.borrowed.set(&mut borrowed);
        let result = f(borrowed.obj_ref.deref());
        self.borrowed.set(ptr::null_mut());

        Ok(result)
    }

    pub fn try_with_assuming_borrowed_mut<F, R>(&self, f: F) -> Result<R, RustObjAccessError>
    where F:FnOnce(&mut T) -> R
    {
        let ptr_to_borrowed = self.borrowed.get();
        if let Some(borrowed) = unsafe { ptr_to_borrowed.as_mut() } {
             match &mut borrowed.obj_ref {
                RustObjRef::Immutable(_) |
                RustObjRef::ImmutableBorrowedExternally(_) => {
                    // Call try_borrow_mut() to get BorrowMutError and return it from the function
                    let _ref = borrowed.obj_rc.try_borrow_mut()
                        .map_err(|err| RustObjAccessError::BorrowMutError(err))?;
                    panic!("Object assumed to be borrowed but it is not")
                },
                RustObjRef::Mutable(_) |
                RustObjRef::MutableBorrowedExternally(_) => {
                    return Ok(f(borrowed.obj_ref.deref_mut().unwrap()))
                }
            }
        }

        let rc = self.obj_ptr.get_rc()
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

        let mut borrowed = RustObjBorrowed::new_borrowed_externally(rc, true);

        self.borrowed.set(&mut borrowed);
        let result = f(borrowed.obj_ref.deref_mut().unwrap());
        self.borrowed.set(ptr::null_mut());

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


/// Strong (if created on the Rust side) or weak (if created from Qml)
/// pointer to Rust object.
pub enum RustObjPtr<T: ?Sized> {
    Strong(Rc<RefCell<T>>),
    Weak(Weak<RefCell<T>>),
}

impl<T: ?Sized> RustObjPtr<T> {
    fn get_rc(&self) -> Option<Rc<RefCell<T>>> {
        match self {
            RustObjPtr::Strong(rc) => Some(rc.clone()),
            RustObjPtr::Weak(weak) => weak.upgrade()
        }
    }
}

/// Struct with a borrowed object
struct RustObjBorrowed<'a, T: ?Sized> {
    /// Strong pointer being held to make sure referenced object is alive while borrowed.
    obj_rc: Rc<RefCell<T>>,

    /// Ref or RefMut obtained by borrowing from RefCell.
    obj_ref: RustObjRef<'a, T>,
}

impl<'a, T: ?Sized> RustObjBorrowed<'a, T> {
    fn new(rc: Rc<RefCell<T>>, is_mutable: bool) -> Result<Self, RustObjAccessError> {
        let mut uninit: MaybeUninit<Self> = MaybeUninit::uninit();
        let ptr = uninit.as_mut_ptr();
        unsafe {
            addr_of_mut!((*ptr).obj_rc).write(rc.clone());
            addr_of_mut!((*ptr).obj_ref).write(RustObjRef::new(&(*ptr).obj_rc, is_mutable)?);
            Ok(uninit.assume_init())
        }
    }

    fn new_borrowed_externally(rc: Rc<RefCell<T>>, is_mutable: bool) -> Self {
        let mut uninit: MaybeUninit<Self> = MaybeUninit::uninit();
        let ptr = uninit.as_mut_ptr();
        unsafe {
            addr_of_mut!((*ptr).obj_rc).write(rc.clone());
            addr_of_mut!((*ptr).obj_ref).write(RustObjRef::new_borrowed_externally(&(*ptr).obj_rc, is_mutable));
            uninit.assume_init()
        }
    }
}

/// Mutable or immutable reference to Rust object
/// obtained via RefCell::try_borrow()/try_borrow_mut()
/// or RefCell::as_ptr() if the object was already borrowed externally
enum RustObjRef<'a, T: ?Sized> {
    Immutable(Ref<'a, T>),
    Mutable(RefMut<'a, T>),
    ImmutableBorrowedExternally(NonNull<T>),
    MutableBorrowedExternally(NonNull<T>),
}

impl<'a, T: ?Sized> RustObjRef<'a, T> {
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

    fn new_borrowed_externally(rc: &'a Rc<RefCell<T>>, is_mutable: bool) -> Self {
        // This Rc is stored in parent struct
        // so there is no risk of input rc getting out of scope.
        let nn = NonNull::new(rc.as_ptr()).unwrap();
        match is_mutable {
            true => Self::MutableBorrowedExternally(nn),
            false => Self::ImmutableBorrowedExternally(nn),
        }
    }

    fn deref(&self) -> &T {
        match self {
            Self::Immutable(ref_) => ref_,
            Self::Mutable(ref_mut) => ref_mut,
            Self::ImmutableBorrowedExternally(ptr) => unsafe { ptr.as_ref() },
            Self::MutableBorrowedExternally(ptr) => unsafe {ptr.as_ref() },
        }
    }

    fn deref_mut(&mut self) -> Option<&mut T> {
        match self {
            Self::Immutable(_) => None,
            Self::ImmutableBorrowedExternally(_) => None,
            Self::Mutable(ref_mut) => Some(ref_mut),
            Self::MutableBorrowedExternally(ptr) => Some(unsafe {ptr.as_mut() }),
        }
    }
}
