// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge_type_lib::{QMetaObject, QMetaType};
use std::rc::Rc;
use std::cell::RefCell;

pub enum ConstructionMode {
    Strong,
    Weak,
    AtAddress(*mut u8),
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
/// The concrete C++ proxy type.
///
/// ## `RcRefCellType`
///
/// The Rust container type holding the actual object, typically:
///
/// ```rust, ignore
/// Rc<RefCell<dyn MarkerTrait>>
/// ```
/// The MarkerTrait is implemented on the users rust struct by the qobject macro.
///
pub trait QRustProxy {
    type ProxyCppType;
    type AdapterType: ?Sized;
    fn new(rust_obj: &Rc<RefCell<Self::AdapterType>>, construction: ConstructionMode, on_drop: fn(rust_obj: *const u8)) -> *mut Self;
    fn get_static_meta_object() -> &'static QMetaObject;
    fn get_size_of_cpp_proxy() -> usize;
    fn get_align_of_cpp_proxy() -> usize;
    fn get_qmetatype_list_of_cpp_proxy() -> QMetaType;
    fn get_cpp_proxy(&self) -> *const Self::ProxyCppType;
    fn get_cpp_proxy_mut(&self) -> *mut Self::ProxyCppType;
}
