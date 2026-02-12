// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

//! # Qt Interfaces (Internal Documentation)
//!
//! This crate provides proxies for various Qt (C++) interfaces using Cxx.
//! Proxies serve as the bridge between Rust and C++ implementations of Qt
//! interfaces, allowing Rust types to implement behavior expected by C++
//! code and to safely access C++ functionality from Rust.
//!
//! Each Qt interface has a corresponding **proxy** in Rust. A proxy works together
//! with a set of traits to provide three main capabilities:
//!
//! 1. **User-implemented functionality**: Rust implementations of Qt virtual functions.
//! 2. **Rust access to C++ functionality**: Rust wrappers around C++ methods of the interface.
//! 3. **Internal type erasure and proxy plumbing**: Allows dynamic behavior when types
//!    are not fully known at compile time.
//!
//! These proxies rely on the following Rust traits to define functionality and structure,
//! shown on the example of `QListModel`.
//!
//! ### 1. `QListModel`
//! * **Responsibility**: Must be implemented by the user.
//! * **Purpose**: Functions in this trait are called from C++ via the proxy, providing Rust
//!   implementations of Qt virtual functions.
//! * **Associated Types**: Defines types (e.g., `Item`) that are part of the interface contract.
//! * **Notes**: Users implement the logic of each function according to the Qt behavior
//!   expected by the C++ side. Default implementation can exist for non-pure virtual functions.
//!
//! ### 2. `QListModelBase`
//! * **Responsibility**: Implemented automatically by `qt_gen` for all Rust types that have
//!   `QListModel` as a base. Has generic or default implementations for most interface functions.
//! * **Purpose**: Provides access to C++ functions from Rust and other convenience functions
//!   that should not be overridden. Serves as a bridge between user logic and C++ functionality.
//! * **Notes**: This trait forms the core Rust interface to the C++ side for types implementing
//!   `QListModel`.
//!
//! ### 3. `QListModelProxyGet` (Internal Only)
//! * **Responsibility**: Fully implemented by `qt_gen` for all types that have `QListModel` as a base.
//! * **Purpose**: Provides internal access to proxies and related interfaces. Required for machinery
//!   that serves to connect a Rust object to corresponding proxy object.
//! * **Notes**: Never called or implemented by end users. Contains functions generated once `qt_gen`
//!   knows all traits and proxies for a specific type. Serves as internal glue between all the different
//!   parts of the language bridge.
//!
//! ### 4. `QListModelAdapter` (Internal Only)
//! * **Responsibility**: Fully implemented by `qt_gen` for all types with `QListModel` as a base.
//! * **Purpose**: Provides a type-erased interface for internal machinery. Traits with associated
//!   types (like `QListModel::Item`) cannot be used as `dyn` traits in Rust. `QListModelAdapter`
//!   erases unknown types while exposing the necessary interface internally.
//! * **Notes**: Never meant for user implementation or usage. Facilitates dynamic behavior and
//!   internal bridging without exposing associated types to user code.

// TODO
// - Define what is meant with "base". We will probably change it soon.
// - Clarify the role of `qt_gen` and how it interacts with proxies and internal traits.

mod generated;
pub use generated::*;

mod manual;
pub use manual::*;

pub mod object_access;
pub use object_access::rust_object_access::RustObjAccess;
