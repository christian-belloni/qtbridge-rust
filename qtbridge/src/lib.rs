// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

//! This library enables building modern Qt Quick user interfaces with a Rust backend.
//! It allows you to run [QML](https://doc.qt.io/qt-6/qmlreference.html) code and expose
//! Rust data structures directly to the [QML engine](https://doc.qt.io/qt-6/qqmlengine.html),
//! combining a declarative UI with Rust-based business logic.
//!
//! Qt itself is written in C++, and QtBridge builds on [CXX](https://cxx.rs/) to access the required Qt interfaces.
//! As a user, you do not need to write any C++ code. Instead, Rust structs and data can be exposed to QML using
//! attribute macros provided by the library.
//!
//! If your project requires mixing Rust and C++ code, using Qt Widgets, or accessing Qt modules that only provide
//! a C++ API, consider using [CXX-Qt](https://github.com/KDAB/cxx-qt) instead.
//!
//! Internally, the library relies on Qt concepts such as [QObjects](https://doc.qt.io/qt-6/qobject.html),
//! [properties](https://doc.qt.io/qt-6/properties.html), [signals and slots](https://doc.qt.io/qt-6/signalsandslots.html),
//! and the [Model/View architecture](https://doc.qt.io/qt-6/model-view-programming.html).
//! While these are exposed through a Rust-friendly API, familiarity with these Qt concepts will help you get
//! the most out of building UIs with Qt Quick.
//!
//!
//! ## Example:
//!
//! Main.rs
//! ```ignore
//! use qtbridge::{qobject_impl, qml_element, QApp};
//!
//! #[derive(Default)]
//! pub struct Backend {
//! }
//!
//! #[qobject_impl]
//! #[qml_element]
//! impl Backend {
//!     #[qslot]
//!     fn say_hello(&self) {
//!         println!("Hello World!")
//!     }
//! }
//!
//! impl Drop for Backend {
//!     fn drop(&mut self) {
//!         self.detach_qobject();
//!    }
//! }
//!
//! fn main() {
//!     QApp::new()
//!         .load_qml(include_bytes!("main.qml"))
//!         .run();
//! }
//! ```
//!
//! main.qml
//! ```js, ignore
//! import QtQuick
//! import QtQuick.Controls
//! import hello_world
//!
//! ApplicationWindow {
//!
//!    visible: true
//!     title: qsTr("Minimal QML app")
//!
//!     Backend {
//!         id: backend
//!     }
//!
//!     Button {
//!         anchors.centerIn: parent
//!         text: "Hello World!"
//!         onClicked: backend.sayHello()
//!     }
//! }
//! ```
//!
#[doc(hidden)]
pub use bridge;
#[doc(hidden)]
pub use qt_gen;
pub use qt_traits;
pub use qt_traits::*;
#[doc(hidden)]
pub use qt_ifaces;
pub use qt_type_lib;
#[doc(hidden)]
pub use quicktest_macro::*;
pub use qresource_macro::*;
#[doc(hidden)]
pub use build_common;
pub use qt_container::*;
pub use qt_container_macro::*;
#[doc(hidden)]
pub use quicktest;


#[cfg(doc)]
pub use qt_gen::{qsignal, qslot};
pub use qt_gen::{qobject, qobject_impl, qml_element};
pub use bridge::{QApp, run_simple_app, qresource};
pub use qt_ifaces::{QAbstractItemModel, QAbstractItemModelBase};
pub use qt_ifaces::{QAbstractListModel,QAbstractListModelBase};
pub use qt_ifaces::{QObject, QObjectBase};
