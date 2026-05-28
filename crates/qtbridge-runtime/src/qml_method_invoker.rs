// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use qtbridge_type_lib::{QMetaObject, QObject, QVariantList};

use crate::QObjectHolder;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qobject/cpp/qobject.h");
        type QObject = qtbridge_type_lib::QObject;

        include!("cpp/qml_method_invoker.h");

        unsafe fn connect_destroyed_callback(obj: *mut QObject, flag_ptr: usize);
    }

    extern "Rust" {
        fn on_qobject_destroyed(flag_ptr: usize);
    }
}

fn on_qobject_destroyed(flag_ptr: usize) {
    let arc = unsafe { Arc::from_raw(flag_ptr as *const AtomicBool) };
    arc.store(false, Ordering::Release);
}

/// Convenience macro for invoking methods with a [`QmlMethodInvoker`].
/// The first argument is the invoker, the second is the method name,
/// and the rest are the arguments to be passed to the method.
///
/// The macro will dispatch to the appropriate `invoke_method` or
/// `invoke_method_with_args` based on the number of arguments provided.
///
/// ```
/// use qtbridge::invoke_method;
/// use qtbridge::{qobject_impl, QObjectHolder};
///
/// #[derive(Default)]
/// pub struct MyClass { }
///
/// #[qobject_impl]
/// impl MyClass {
///     #[qslot]
///     fn immutable_slot(&self) {
///     }
/// }
///
/// let qobject_holder = MyClass::default_with_attached_qobject();
/// let invoker = qobject_holder.borrow().get_qml_method_invoker();
/// invoke_method!(invoker, "immutableSlot");
/// ```
///
#[macro_export]
macro_rules! invoke_method {
    ($invoker:expr, $name:expr $(,)?) => {{
        $invoker.invoke_method($name)
    }};

    ($invoker:expr, $name:expr, $($arg:expr),+ $(,)?) => {{
        let args = qtbridge::qtbridge_type_lib::QVariantList::from([
            $(qtbridge::qtbridge_type_lib::QVariant::from($arg)),+
        ]);
        $invoker.invoke_method_with_args($name, &args)
    }};
}

/// A thread-safe handle for invoking methods on a [`QObjectHolder`] from any thread.
///
/// Allows calling `#[qslot]`s and `#[qsignal]`s defined on a [`QObjectHolder`]
/// enabling interaction with QML from any thread.
///
/// Obtain an instance via [`QObjectHolder::get_qml_method_invoker`].
/// # Example
///
/// ```
/// # use qtbridge::{qobject, QObjectHolder};
/// # #[qobject]
/// # pub mod example {
/// #     #[derive(Default)]
/// #     pub struct Backend {}
/// #     impl Backend {
/// #         #[qsignal]
/// #         pub fn data_ready(&mut self);
/// #     }
/// # }
/// # use example::Backend;
/// let backend = Backend::default_with_attached_qobject();
/// let invoker = backend.borrow().get_qml_method_invoker();
/// std::thread::spawn(move || {
///     invoker.invoke_method("dataReady");
/// }).join().unwrap();
/// ```
pub struct QmlMethodInvoker {
    obj: *mut QObject,
    alive: Arc<AtomicBool>,
}

unsafe impl Send for QmlMethodInvoker {}

impl QmlMethodInvoker {

    /// Creates a `QmlMethodInvoker` for `target` and tracks its lifetime.
    pub fn new<T: QObjectHolder>(target: &T) -> Self {
        let obj = target.get_qobject_ptr();
        let alive = Arc::new(AtomicBool::new(true));
        let flag_ptr = Arc::into_raw(alive.clone()) as usize;
        unsafe { ffi::connect_destroyed_callback(obj, flag_ptr) };
        Self { obj, alive }
    }

    fn is_alive(&self) -> bool {
        self.alive.load(Ordering::Acquire)
    }

    /// Invokes the method `name` on the underlying `QObject`.
    /// The method will be scheduled to run on the Qt event loop on the Qt thread.
    /// Qt will attempt to borrow a reference from the Rc<RefCell<>> that it holds.
    /// If it fails to borrow, the program will panic.
    /// If the `QObject` has been dropped in the meantime, the call will be ignored and `invoke_method` will return false.
    ///
    /// `name` must be the name of a `#[qslot]` or `#[qsignal]`.
    ///
    /// Returns true if the member could be invoked.
    /// Returns false if there is no such member or it cannot be invoked.
    pub fn invoke_method(&self, name: &str) -> bool {
        if !self.is_alive() {
            return false;
        }
        QMetaObject::invoke_method(self.obj, name)
    }

    /// Invokes the method `name` on the underlying `QObject` with the parameters
    /// provided in `args`. If the parameters don't match the method signature,
    /// the call will be ignored and `invoke_method_with_args` will return false.
    /// The method will be scheduled to run on the Qt event loop on the Qt thread.
    /// Qt will attempt to borrow a reference from the Rc<RefCell<>> that it holds.
    /// If it fails to borrow, the program will panic.
    /// If the `QObject` has been dropped in the meantime, the call will be ignored
    /// and `invoke_method` will return false.
    ///
    /// `name` must be the name of a `#[qslot]` or `#[qsignal]`.
    ///
    /// Returns true if the member could be invoked.
    /// Returns false if there is no such member or it cannot be invoked.
    pub fn invoke_method_with_args(&self, name: &str, args: &QVariantList) -> bool {
        if !self.is_alive() {
            return false;
        }
        QMetaObject::invoke_method_with_args(self.obj, name, args)
    }
}
