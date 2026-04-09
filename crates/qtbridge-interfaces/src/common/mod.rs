// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

pub mod qaim_cpp_bridge;
pub mod qaim_rust_bridge;
pub use qaim_cpp_bridge::ffi::QAIMProxyCpp;
pub use qaim_rust_bridge::QGenericAIMProxyRust;
