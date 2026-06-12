// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only


//! This module contains functions for importing artifacts from the
//! [Qt Resource System](https://doc.qt.io/qt-6/resources.html).
//!
//! All artifacts must be compiled into dynamic resources with the
//! [`rcc` tool](https://doc.qt.io/qt-6/rcc.html):
//! ```bash, ignore
//! rcc -binary [more options] <inputs>
//! ```
//! before or at compile time.
//! The input for the `rcc` tool are Qt Resource Collection (*.qrc) files,
//! that contain lists of all files to be imported.
//! ```xml, ignore
//! <RCC>
//!     <qresource prefix="/">
//!         <file>images/copy.png</file>
//!         <file>images/cut.png</file>
//!         ...
//!     </qresource>
//! </RCC>
//! ```
//!
//! Dynamic resources can be registered with [`register_bytes`] and
//! are then available at runtime through the `qrc:/` scheme in QML:
//! ```qml, ignore
//! Image {
//!     source: "qrc:/images/copy.png"
//! }
//! ```
//!
//! We further provide the `include_bytes_qml` macro that generates the
//! required dynamic resource directly in Rust at compile time.
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cpp/qresource.h");

        fn register_resource(data: &[u8], resource_root: &str) -> bool;
    }
}

/// Registers Qt resource data from an in-memory byte slice, making embedded
/// resources available at runtime through the `qrc:/` scheme.
///
/// Returns `true` if registration succeeded, `false` otherwise.
pub fn register_bytes(data: &[u8]) -> bool {
    ffi::register_resource(data, "")
}

/// Registers Qt resource data from an in-memory byte slice under `resource_root`,
/// making embedded resources available at runtime through the `qrc:/` scheme.
///
/// Returns `true` if registration succeeded, `false` otherwise.
pub fn register_bytes_with_prefix(data: &[u8], resource_root: &str) -> bool {
    ffi::register_resource(data, resource_root)
}
