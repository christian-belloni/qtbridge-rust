// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only


use qtbridge_type_lib::{QMetaObject, QMetaType};
use crate::DynamicMetaObjectData;
use std::rc::Rc;
use std::cell::RefCell;
use std::pin::Pin;

pub enum ConstructionMode {
    Strong,
    Weak,
    AtAddress(*mut u8),
}


/// `QCppProxy` defines what a C++ proxy to a QObject (C++) must implement.
///
/// This includes access to the C++ static meta-object, which is then extended
/// on the Rust side to create a dynamic meta-object.
pub trait QCppProxy {
    type ProxyRustType: QRustProxy;
    fn get_static_meta_object() -> &'static QMetaObject;
    fn get_size() -> usize;
    fn get_align() -> usize;
    fn get_qmetatype_list() -> QMetaType;
    unsafe fn create(rust_proxy: *mut Self::ProxyRustType, metaobject: &'static DynamicMetaObjectData) -> *mut Self;
    unsafe fn create_at(rust_proxy: *mut Self::ProxyRustType, metaobject: &'static DynamicMetaObjectData, addr: *mut u8) -> *mut Self;
    fn emit_signal(&self, signal_name: &str, argv: &[*const u8]);
    fn emit_signal_mut(self: Pin<&mut Self>, signal_name: &str, argv: &[*const u8]);
}

/// `QRustProxy` defines the Rust-side bridge object that binds:
///
/// - A Rust object stored in `Rc<RefCell<dyn _>>`
/// - A corresponding C++ QObject-based proxy
///
/// Implementations of this trait are the concrete glue layer between
/// Rust and Qt, usually using Cxx.
///
/// # Purpose
///
/// A `QRustProxy` implementation:
///
/// - Stores a raw pointer to the C++ proxy (`cpp_proxy`)
/// - Stores access to the Rust object through `RustObjAccess<dyn _>` (`rust_obj`)
/// - Coordinates destruction, layout and Qt meta-object information
/// - Forwards all foreign function calls to the C++ proxy
///
/// Typical structure:
///
/// ```rust, ignore
/// pub struct QObjectProxyRust {
///     cpp_proxy: *mut QObjectProxyCpp,
///     rust_obj: RustObjAccess<dyn QObjectProxyGet>,
///     on_drop: fn(rust_obj: *const u8),
/// }
/// ```
///
/// Where:
///
/// - `cpp_proxy` points to the actual C++ QObject subclass.
/// - `rust_obj` wraps access to the users rust object.
/// - `on_drop` cleaning up memory.
///
/// # Associated Types
///
/// ## `ProxyCppType`
///
/// The concrete C++ proxy type. Has to implement [´QCppProxy´].
///
/// ## `AdapterType`
///
/// A wrapper trait for the interface trait that QtBridge users implement. This wrapper
/// is required because not all traits can be used with dyn and are thus incompatible with
/// [`RustObjAccess`] (See "object safety" or "dyn compatibility").
///
pub trait QRustProxy {
    type ProxyCppType: QCppProxy<ProxyRustType = Self>;
    type AdapterType: ?Sized;
    fn new(rust_obj: &Rc<RefCell<Self::AdapterType>>, metaobject: &'static DynamicMetaObjectData, construction: ConstructionMode, on_drop: Box<dyn FnOnce() + 'static>) -> *mut Self;
    fn get_cpp_proxy(&self) -> *const Self::ProxyCppType;
    fn get_cpp_proxy_mut(&self) -> *mut Self::ProxyCppType;
    fn emit_signal(&self, reference: &Self::AdapterType, signal_name: &str, argv: &[*const u8]);
    fn emit_signal_mut(&self, mut_ref: &mut Self::AdapterType, signal_name: &str, argv: &[*const u8]);
}
