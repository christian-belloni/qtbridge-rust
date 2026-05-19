// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::cell::RefCell;
use std::ptr::NonNull;
use std::rc::Rc;

use qtbridge_type_lib::{QMetaTypeGet, QObject, QVariant};
use crate::qproxies::{QRustProxy, ConstructionMode};
use crate::rustobjectgetter::get_rust_object_rc_ptr;
use crate::{DispatchMetaCall, QMetaInfo, QmlMethodInvoker};
use std::collections::HashMap;


pub trait QObjectHolder : DispatchMetaCall + QMetaInfo + Default {
    /// Alias for the Rust proxy type corresponding to the user-defined type.
    /// The Rust proxy is an intermediate layer between the Rust object and the C++ proxy,
    /// forwarding calls in both directions and managing borrowing of the Rust object
    /// during QAIM calls (and TBD for meta calls as well).
    #[doc(hidden)]
    type ProxyRust: QRustProxy<ProxyCppType = <Self as QMetaInfo>::CppProxy>;

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

    /// Return a pointer to the Rust proxy associated with the specified object,
    /// or `None` if no proxy is registered.
    #[doc(hidden)]
    fn try_get_rust_proxy_ptr(&self) -> Option<*mut Self::ProxyRust> {
        let rust_obj_ptr = std::ptr::from_ref(self).cast::<u8>();
        let proxy_ptr = Self::try_borrow_mut_proxies_map(|map| {
            map.get(&rust_obj_ptr).copied().unwrap_or_default()
        });
        NonNull::new(proxy_ptr as *mut Self::ProxyRust).map(|nn| nn.as_ptr())
    }

    #[doc(hidden)]
    fn get_qobject_ptr(&self) -> *mut QObject {
        let Some(proxy_ptr) = Self::try_get_rust_proxy_ptr(self) else {
            return std::ptr::null_mut()
        };
        let rust_proxy = unsafe { &*proxy_ptr };
        let cpp_proxy = rust_proxy.get_cpp_proxy();
        cpp_proxy as *mut QObject
    }

    #[doc(hidden)]
    /// Return `QObject` attached to the specified Rust object.
    fn rc_ref_cell_to_qobject(self_obj: &Rc<RefCell<Self>>) -> *const QObject {
        // Avoid borrowing here. We don't actually access the Rust object in the function.
        // We only need its raw pointer to perform the lookup in the instance map.
        unsafe { self_obj.as_ptr().as_ref() }
            .unwrap()
            .get_qobject_ptr()
    }

    #[doc(hidden)]
    /// Return the Rust object attached to the specified `QObject`.
    unsafe fn qobject_to_rc_ref_cell(qobj_ptr: *const QObject) -> Rc<RefCell<Self>>
    where Self: QMetaTypeGet
    {
        let qobj_ref = unsafe { qobj_ptr.as_ref() }
            .expect("Input QObject is null");
        let raw_u8 = get_rust_object_rc_ptr(qobj_ref);
        if raw_u8.is_null() {
            panic!("Rust object associated with given QObject was already dropped")
        }

        let qobj_meta_obj_ptr = qobj_ref.get_qmeta_object();
        let qobj_meta_obj_ref = unsafe { qobj_meta_obj_ptr.as_ref() }
            .expect("QMetaObject is null");
        let qobj_meta_type = qobj_meta_obj_ref.meta_type();
        let self_meta_type = Self::get_qmetatype();
        if self_meta_type != qobj_meta_type {
            panic!("Value of wrong type is assigned to property: '{}' instead of '{}'",
                qobj_meta_type.name(), self_meta_type.name())
        }

        let raw_ref_cell = raw_u8 as *const RefCell<Self>;
        unsafe { Rc::from_raw(raw_ref_cell) }
    }

    /// Returns a [`QmlMethodInvoker`] that can invoke methods on the underlying
    /// `QObject` from any thread.
    ///
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
    /// invoker.invoke_method("dataReady");
    /// ```
    fn get_qml_method_invoker(&self) -> QmlMethodInvoker
    {
        QmlMethodInvoker::new(self)
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
        let dynamic_meta = <Self as QMetaInfo>::get_shared_dynamic_meta_object_data();
        let proxy = Self::ProxyRust::new(&dyn_rc, dynamic_meta, construction, Box::new(move || Self::unregister_instance_in_map(key)));
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

    /// Create a new default-initialized instance of this type and attach
    /// the required [`QObject`]. This enables use of this instance in QML.
    /// Instances created with this function must remain at its original heap
    /// location and must not be moved out of `Rc<RefCell<T>>`.
    fn default_with_attached_qobject() -> std::rc::Rc<std::cell::RefCell<Self>> {
        let instance = Default::default();
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
    }

    /// Detach and remove the dedicated [`QObject`] from the specified object.
    /// This function is intended to be called during the [`Drop`] implementation
    /// of this type.
    fn detach_qobject(&self) {
        let qobj_ptr = self.get_qobject_ptr();
        if !qobj_ptr.is_null() {
            QObject::delete(qobj_ptr);
        }
    }

    /// Return a [`QVariant`] containing a pointer to this object.
    fn as_qvariant(&self) -> QVariant {
        let qobj_ptr = self.get_qobject_ptr();
        assert!(!qobj_ptr.is_null(), "QObject is not attached");
        qobj_ptr.into()
    }

}
