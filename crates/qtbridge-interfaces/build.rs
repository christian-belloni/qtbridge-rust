// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge_build_common::qt_build::{link_qt_modules, qt_include_dirs, QtBuildConfigure};

const FILES_BRIDGE: [&'static str; 8] = [
    "src/generated/qabstract_item_model/proxy_cpp_bridge.rs",
    "src/generated/qabstract_item_model/proxy_rust_bridge.rs",
    "src/generated/qabstract_list_model/proxy_cpp_bridge.rs",
    "src/generated/qabstract_list_model/proxy_rust_bridge.rs",
    "src/common/qaim_cpp_bridge.rs",
    "src/common/qaim_rust_bridge.rs",
    "src/qobject/proxy_cpp_bridge.rs",
    "src/qobject/proxy_rust_bridge.rs",
];

const FILES_CPP: [&'static str; 4] = [
    "src/generated/qabstract_item_model/cpp/QAbstractItemModelProxyCpp.cpp",
    "src/generated/qabstract_list_model/cpp/QAbstractListModelProxyCpp.cpp",
    "src/common/cpp/QAIMProxyCpp.cpp",
    "src/qobject/cpp/QObjectProxyCpp.cpp",
];

fn main() {
    let mut builder = cxx_build::bridges(&FILES_BRIDGE);

    let type_lib_include = std::env::var("DEP_QTBRIDGE_TYPE_LIB_INCLUDE")
    .expect("DEP_QTBRIDGE_TYPE_LIB_INCLUDE not set. This variable should have been set by qtbridge-type-lib");

    let runtime_include = std::env::var("DEP_QTBRIDGE_RUNTIME_INCLUDE")
    .expect("DEP_QTBRIDGE_TYPE_LIB_INCLUDE not set - This variable should have been set by qtbridge-runtime");

    builder
        .std("c++17")
        .flag_if_supported("/Zc:__cplusplus")
        .flag_if_supported("/permissive-")
        .include("src")
        .include("../")
        .include(type_lib_include)
        .include(runtime_include)
        .configure_for_qt();

    FILES_CPP.iter()
        .for_each(|file| {
            builder.file(file);
            println!("cargo::rerun-if-changed={file}");
        });

    let qt_modules = ["Core", "Gui", "Qml"];
    for include_dir in qt_include_dirs(&qt_modules, true) {
        builder.include(include_dir);
    }

    builder.compile("qtbridge-interfaces");

    link_qt_modules(&qt_modules);
}
