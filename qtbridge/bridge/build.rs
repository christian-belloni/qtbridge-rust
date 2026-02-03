// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::path::Path;
use build_common::qt_build::{qt_include_dirs, link_qt_modules};

fn main() {

    let bridge_files = vec!(
        "dynamicmetaobjectdata",
        "metamethodparams",
        "qresource"
    );

    let other_rust_files: Vec<&str> = vec!(
    );

    let other_cpp_files: Vec<&str> = vec!(
        "dynamicmetaobjectdata.cpp",
        "qresource_rust.cpp"
    );

    let qt_modules = [
        "Core",
        "Gui",
        "Qml",
        "QuickTest"
    ];

    let mut rust_bridge_files: Vec<String> = Vec::new();
    let mut cpp_files: Vec<String> = Vec::new();

    // Handle bridge files (rust+[cpp+h] file with the same stem)
    for bridge_file in &bridge_files {
        let rust_file = format!("src/{bridge_file}.rs");
        let cpp_file = format!("src/cpp/{bridge_file}.cpp");

        println!("cargo::rerun-if-changed={rust_file}");
        rust_bridge_files.push(rust_file);
        if Path::new(&cpp_file).is_file() {
            cpp_files.push(cpp_file);
        }
    }

    for rust_file in other_rust_files {
        let rust_file = format!("src/{rust_file}");
        println!("cargo::rerun-if-changed={rust_file}");
    }

    for cpp_file in other_cpp_files {
        let cpp_file = format!("src/cpp/{cpp_file}");
        cpp_files.push(cpp_file);
    }

    let mut builder = cxx_build::bridges(rust_bridge_files);

    builder
        .std("c++17")
        .flag_if_supported("/Zc:__cplusplus")
        .flag_if_supported("/permissive-")
        .include("../")
        .include("../utils")
        .include("src");

    let qt_include_dirs = qt_include_dirs(qt_modules, true);
    for include_dir in qt_include_dirs {
        builder.include(include_dir);
    }

    for cpp_file in &cpp_files {
        builder.file(cpp_file);

        println!("cargo::rerun-if-changed={cpp_file}");
        let cpp_path = Path::new(cpp_file);
        let h_path = cpp_path.with_extension("").with_extension("h");
        if h_path.is_file() {
            println!("cargo::rerun-if-changed={}", h_path.to_str().unwrap());
        }
    }

    println!("cargo::rerun-if-changed=src/lib.rs");

    builder.compile("bridge");

    link_qt_modules(&qt_modules);
}
