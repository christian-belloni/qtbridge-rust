// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::path::{Path, PathBuf};

pub fn relative_input_file_path_to_path_qualified(src: &str) -> Result<Vec<String>, String> {
    let mut comps: Vec<String> = PathBuf::from(src)
        .components()
        .map(|comp| comp
            .as_os_str()
            .to_string_lossy()
            .to_string()
            .to_ascii_lowercase())
        .collect();

    if comps.get(0).is_some_and(|c0| c0 == "src") &&
       comps.get(1).is_some_and(|c1| c1 == "input") {
        comps.drain(0..2);
    }

    let Some(last_comp) = comps.last_mut() else {
        return Err("Path does not contain module components".into())
    };

    if let Some(new_stem) = last_comp.strip_suffix(".rs") {
        *last_comp = new_stem.into();
    }
    Ok(comps)
}

pub fn file_path_to_module_name(src: &Path) -> Result<String, String> {
    let last_comp = src
        .components()
        .next_back()
        .map(|comp| comp.as_os_str()
            .to_string_lossy()
            .to_string()
            .to_ascii_lowercase()
        );
    last_comp.and_then(|comp| comp.strip_suffix(".rs").map(String::from))
        .ok_or_else(|| format!("Failed to get module name from '{}'", src.display()))
}
