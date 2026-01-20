// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use build_common::qt_build::{link_qt_modules, qt_include_dirs};
use qt_type_gen::{generated_files_bridge,generated_files_cpp};

fn main() {
    let mut builder = cxx_build::bridges(generated_files_bridge());
    builder
        .std("c++17")
        .flag_if_supported("/Zc:__cplusplus")
        .flag_if_supported("/permissive-")
        .include("src")
        .include("../")
        .include("../utils");

    generated_files_cpp().iter()
        .for_each(|file| {
            builder.file(file);
        });

    let qt_modules = ["Core", "Gui", "Qml", "Test"];
    for include_dir in qt_include_dirs(&qt_modules, true) {
        builder.include(include_dir);
    }

    builder.compile("qt_type_lib");

    link_qt_modules(&qt_modules);
}

