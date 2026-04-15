// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::cell::RefCell;
use std::rc::Rc;

use qtbridge_type_lib::{QObject, QVariant, QMetaType, QMetaObject};
use crate::qrustproxy::{QRustProxy, ConstructionMode};
use crate::{DispatchMetaCall, QMetaInfo};
use std::collections::HashMap;


pub trait QObjectHolder : DispatchMetaCall + QMetaInfo + Default {
    /// Alias for the Rust proxy type corresponding to the user-defined type.
    /// The Rust proxy is an intermediate layer between the Rust object and the C++ proxy,
    /// forwarding calls in both directions and managing borrowing of the Rust object
    /// during QAIM calls (and TBD for meta calls as well).
    #[doc(hidden)]
    type ProxyRust : QRustProxy;

    #[doc(hidden)]
    fn try_borrow_mut_proxies_map<F, R>(f: F) -> R
    where
        F: FnOnce( &mut HashMap<*const u8, *const u8>) -> R
    {
        thread_local! { static INSTANCES: RefCell<HashMap<*const u8, *const u8>> =
                RefCell::new(HashMap::new());
        }
        INSTANCES.with_borrow_mut(f)
    }

    /// Return an immutable reference to the Rust proxy linked to the Rust object specified in the argument.
    #[doc(hidden)]
    fn get_rust_proxy(&self) -> &Self::ProxyRust
    {
        Self::get_rust_proxy_mut(self)
    }

    /// Return a mutable reference to the Rust proxy linked to the Rust object specified in the argument.
    #[doc(hidden)]
    fn get_rust_proxy_mut(&self) -> &mut Self::ProxyRust
    {
        Self::try_get_rust_proxy_mut(self)
            .expect("No proxy registered for given rust object")
    }

    /// Return a Result wrapping mutable reference to the Rust proxy associated with the specified object.
    #[doc(hidden)]
    fn try_get_rust_proxy_mut(&self) -> Option<&mut Self::ProxyRust>
    {
        let rust_obj_ptr = std::ptr::from_ref(self).cast::<u8>();
        let proxy_ptr = Self::try_borrow_mut_proxies_map(|map| {
            map.get(&rust_obj_ptr).copied().unwrap_or_default()
        });

        unsafe {
            (proxy_ptr as *mut Self::ProxyRust).as_mut()
        }
    }

    /// Try to get the [`QObject`] linked to this Rust `struct`.
    #[doc(hidden)]
    fn try_get_qobject(&self) -> Option<&mut QObject> {
        let rust_proxy = Self::try_get_rust_proxy_mut(&self)?;
        let cpp_proxy = rust_proxy.get_cpp_proxy();
        let qobject_ptr: *const QObject = cpp_proxy.cast();
        unsafe { qobject_ptr.cast_mut().as_mut() }
    }

    /// Get the [`QObject`] linked to this Rust `struct`. Panics if no
    /// [`QObject`] is attached.
    #[doc(hidden)]
    fn get_qobject(&self) -> &mut QObject
    {
        self.try_get_qobject()
            .expect("QObject is not attached")
    }

    /// This function has to be implemented on the specific type and
    /// provides the conversion from the specific type to the dynamic
    /// trait type.
    ///
    /// This function ensures that the type indeed implements the trait
    /// specified by the [`QRustProxy`].
    #[doc(hidden)]
    fn as_adaptor_trait(rust_obj_rc: Rc<RefCell<Self>>) -> Rc<RefCell<<Self::ProxyRust as QRustProxy>::AdapterType>>;

    /// Register the given Rust object instance in the multiton.
    /// Create Rust and C++ proxies and links them to the Rust object.
    /// If `construction` is `AtAddress`, the C++ proxy is created using
    /// placement new operator at respective address
    #[doc(hidden)]
    fn register_instance_in_map(rust_obj_rc: Rc<RefCell<Self>>, construction: ConstructionMode) {
        let key = (*rust_obj_rc).as_ptr() as *const u8;
        let dyn_rc = Self::as_adaptor_trait(rust_obj_rc);
        let proxy = Self::ProxyRust::new(&dyn_rc, construction, Self::unregister_instance_in_map);
        Self::try_borrow_mut_proxies_map(|proxies| {
            proxies.insert(key, proxy as *const u8);
        })
    }

    /// Removes the entry associated with the specified Rust object from the multiton map.
    #[doc(hidden)]
    fn unregister_instance_in_map(rust_obj_ptr: *const u8) {
        Self::try_borrow_mut_proxies_map(|proxies| proxies.remove(&rust_obj_ptr))
            .expect("Proxy object for rust object is not registered")
            .cast_mut();
    }

    /// Configure the [`QObject`] associated with the given Rust object to use
    /// the dynamic metaobject specific to this Rust type.
    #[doc(hidden)]
    fn set_dynamic_meta(instance: &Rc<RefCell<Self>>)
    {
        let dynamic_meta = Self::get_shared_dynamic_meta_object();
        let instance_ref = &instance.borrow();
        let qobject_ref = instance_ref.get_qobject();
        dynamic_meta.set_to_qobject(qobject_ref);
    }

    /// Create a new default-initialized instance of this type and attach
    /// the required [`QObject`]. This enables use of this instance in QML.
    /// Instances created with this function must remain at its original heap
    /// location and must not be moved out of `Rc<RefCell<T>>`.
    fn default_with_attached_qobject() -> std::rc::Rc<std::cell::RefCell<Self>> {
        let instance = std::rc::Rc::new(std::cell::RefCell::new(Self::default()));
        Self::attach_qobject(&instance);
        instance
    }
    /// Create and attach a dedicated [`QObject`] to the `instance`.
    /// The instance must remain at its original heap location and must
    /// not be moved out of `Rc<RefCell<T>>`.
    fn attach_qobject(instance: &std::rc::Rc<std::cell::RefCell<Self>>) {
        Self::register_instance_in_map(
            instance.clone(),
            ConstructionMode::Weak
        );
        Self::set_dynamic_meta(instance);
    }

    /// Detach and remove the dedicated [`QObject`] from the specified object.
    /// This function is intended to be called during the [`Drop`] implementation
    /// of this type.
    fn detach_qobject(&self) {
        if let Some(qobj) = Self::try_get_qobject(self) {
            QObject::delete(std::ptr::from_mut(qobj));
        }
    }

    /// Return a [`QVariant`] containing a pointer to this object.
    fn as_qvariant(&self) -> QVariant {
        let qobj_ref = self.get_qobject();
        let qobj_ptr = std::ptr::from_mut(qobj_ref);
        qobj_ptr.into()
    }

    #[doc(hidden)]
    fn get_static_meta_object() -> &'static QMetaObject {
        <Self::ProxyRust as QRustProxy>::get_static_meta_object()
    }

    #[doc(hidden)]
    fn get_size_of_cpp_proxy() -> usize {
        <Self::ProxyRust as QRustProxy>::get_size_of_cpp_proxy()
    }

    #[doc(hidden)]
    fn get_align_of_cpp_proxy() -> usize {
        <Self::ProxyRust as QRustProxy>::get_align_of_cpp_proxy()
    }

    #[doc(hidden)]
    fn get_qmetatype_list_of_cpp_proxy() -> QMetaType {
        <Self::ProxyRust as QRustProxy>::get_qmetatype_list_of_cpp_proxy()
    }
}
