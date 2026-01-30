// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::cell::RefCell;
use std::rc::Rc;

use qt_type_lib::QObject;
use bridge::QMetaInfo;

/// Trait to enable access to the bridge between C++ and Rust. This trait is
/// automatically implemented by the 'qobject_impl' macro and should not be implemnted
/// manually.
///
/// The functions are meant to be called by other generated code and should not
/// be called manually.
pub trait QObjectHolder : QMetaInfo {
    /// Alias for the Rust proxy type corresponding to the user-defined type.
    /// The Rust proxy is an intermediate layer between the Rust object and the C++ proxy,
    /// forwarding calls in both directions and managing borrowing of the Rust object
    /// during QAIM calls (and TBD for meta calls as well).
    type ProxyRust;

    fn try_borrow_mut_proxies_map<F, R>(f: F) -> R
    where F: FnOnce(&mut std::collections::HashMap<*const u8, *const Self::ProxyRust>) -> R;

    /// Return an immutable reference to the Rust proxy linked to the Rust object specified in the argument.
    fn get_rust_proxy(rust_obj_ref: &Self) -> &Self::ProxyRust
    {
        Self::get_rust_proxy_mut(rust_obj_ref)
    }

    /// Return a mutable reference to the Rust proxy linked to the Rust object specified in the argument.
    fn get_rust_proxy_mut(rust_obj_ref: &Self) -> &mut Self::ProxyRust
    {
        Self::try_get_rust_proxy_mut(rust_obj_ref)
            .expect("No proxy registered for given rust object")
    }

    /// Return a Result wrapping mutable reference to the Rust proxy associated with the specified object.
    fn try_get_rust_proxy_mut(rust_obj_ref: &Self) -> Option<&mut Self::ProxyRust>
    {
        let ptr = Self::try_borrow_mut_proxies_map(|proxies| {
            let rust_obj_ptr = std::ptr::from_ref(rust_obj_ref).cast();
            match proxies.get(&rust_obj_ptr) {
                Some(ptr) => ptr.cast_mut(),
                None => std::ptr::null_mut(),
            }
        });
        unsafe { ptr.as_mut() }
    }

    /// Return Result with QObject linked to the Rust object provided as an argument.
    fn try_get_qobject(&self) -> Option<&mut QObject>;

    /// Return QObject linked to the Rust object provided as an argument.
    fn get_qobject(&self) -> &mut QObject
    {
        self.try_get_qobject()
            .expect("QObject is not attached")
    }

    /// Register the given Rust object instance in the multiton.
    /// Create Rust and C++ proxies and links them to the Rust object.
    fn register_instance_in_map(rust_obj_rc: Rc<RefCell<Self>>, register_strong: bool);

    /// Register the given Rust object instance in the multiton.
    /// Create Rust and C++ proxies and links them to the Rust object.
    /// C++ proxy created using placement new operator at the memory address provided as the first argument.
    fn register_instance_in_map_with_cpp_proxy_at(addr: *mut u8, rust_obj_rc: Rc<RefCell<Self>>);

    /// Removes the entry associated with the specified Rust object from the multiton map.
    fn unregister_instance_in_map(rust_obj_ptr: *const u8) {
        Self::try_borrow_mut_proxies_map(|proxies| proxies.remove(&rust_obj_ptr))
            .expect("Proxy object for rust object is not registered")
            .cast_mut();
    }

    /// Configure the QObject associated with the given Rust object to use
    /// the dynamic metaobject specific to this Rust type.
    fn set_dynamic_meta(instance: &Rc<RefCell<Self>>)
    {
        let dynamic_meta = Self::get_shared_dynamic_meta_object();
        let instance_ref = &instance.borrow();
        let qobject_ref = instance_ref.get_qobject();
        dynamic_meta.set_to_qobject(qobject_ref);
    }
}
