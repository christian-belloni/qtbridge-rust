// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::LazyLock;

pub fn try_qmake_query(var_name: &str) -> Result<String, String> {
    let output = Command::new("qmake")
        .args(["-query", var_name])
        .output()
        .map_err(|err| format!("Failed to run 'qmake -query'. Error: {err}"))?;
    let output = std::str::from_utf8(&output.stdout)
        .map_err(|err| format!("Failed to convert qmake output to string. Error: {err}"))?;
    Ok(output.trim().to_string())
}

pub fn qmake_query(var_name: &str) -> String {
    try_qmake_query(var_name)
        .unwrap()
}

pub fn run_moc(input: &Path, output: &Path) {
    static MOC_PATH: LazyLock<String> = LazyLock::new(|| {
        find_program_path("moc")
            .unwrap()
            .to_string_lossy()
            .to_string()
    });
    Command::new(&*MOC_PATH)
        .arg(input)
        .arg("-o")
        .arg(output)
        .output()
        .expect("Failed to execute moc process");
}

fn find_program_path(name: &str) -> Option<PathBuf> {
    let var_names = [
        "QT_HOST_LIBEXECS",
        "QT_INSTALL_LIBEXECS",
        "QT_HOST_BINS",
        "QT_INSTALL_BINS",
    ];
    for var in var_names {
        let Ok(path) = try_qmake_query(var) else {
            continue
        };
        let path = PathBuf::from(path).join(name);

        // Try to launch the program at the given path querying its version.
        // Assume it is there if the command succeeds.
        if Command::new(&path)
            .arg("-v")
            .output()
            .is_ok()
            {
                return Some(path)
            }
    }

    None
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
