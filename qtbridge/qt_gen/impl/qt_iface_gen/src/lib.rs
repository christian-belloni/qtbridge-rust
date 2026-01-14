// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

include!(concat!(env!("OUT_DIR"), "/generated_files_bridge.rs"));

pub fn generated_files_bridge() -> &'static [&'static str] {
    &GENERATED_FILES_BRIDGE
}

include!(concat!(env!("OUT_DIR"), "/generated_files_cpp.rs"));

pub fn generated_files_cpp() -> &'static [&'static str] {
    &GENERATED_FILES_CPP
}
