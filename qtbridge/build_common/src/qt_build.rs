// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::fmt::Display;
use std::path::PathBuf;
use std::process::Command;

pub fn qmake_query(var_name: &str) -> String {
    let output = Command::new("qmake")
        .args(["-query", var_name])
        .output()
        .expect("failed to execute qmake process");
    let output = std::str::from_utf8(&output.stdout).unwrap();
    output.trim().to_string()
}

pub fn qt_include_dir() -> String {
    qmake_query("QT_INSTALL_HEADERS")
}

pub fn qt_include_dirs(modules: impl IntoIterator<Item: Display>, add_private: bool) -> Vec<String> {
    let qtbase_include_dir = PathBuf::from(&qmake_query("QT_INSTALL_HEADERS"));
    let qt_version = qmake_query("QT_VERSION");

    modules.into_iter()
        .flat_map(|module| {
            let module_name = format!("Qt{module}");
            let module_path = qtbase_include_dir.join(&module_name);
            if add_private {
                let module_version_path = module_path.join(&qt_version);
                let private_module_path = module_version_path.join(&module_name);

                vec![module_version_path, private_module_path, module_path]
            } else {
                vec![module_path]
            }
        })
        .chain(std::iter::once(qtbase_include_dir.clone()))
        .map(|path| path.to_string_lossy().to_string())
        .collect()
}

pub fn qt_libs_dir() -> String {
    qmake_query("QT_INSTALL_LIBS")
}

pub fn link_qt_modules(modules: impl IntoIterator<Item: Display>) {

    let qt_libs_dir = qt_libs_dir();
    println!("cargo::rustc-link-search={qt_libs_dir}");

    modules.into_iter()
        .for_each(|module| {
            println!("cargo::rustc-link-lib=Qt6{module}")
    });
}
