// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::env;
use std::fs;
use std::path::{self, Path, PathBuf};

pub fn write_to_file(path: &Path, content: &str) -> Result<(), String> {
    fs::write(path, content)
        .map_err(|err| format!("Failed to write to file '{}'.\nError: {err}", path.display()))
}

pub fn write_to_file_if_changed(path: &Path, content: &str) -> Result<(), String> {
    // Do not overwrite file and do not update its timestamp if contents remain the same.
    // Used for files with generated code to avoid rebuilding of those files on every build.
    let mut need_write = true;
    if path.is_file() {
        let old_content = read_file_content(path)?;
        if old_content == content {
            need_write = false;
        }
    }

    if need_write {
        write_to_file(path, content)?
    }

    Ok(())
}

pub fn read_file_content(path: &Path) -> Result<String, String> {
    fs::read_to_string(path)
        .map_err(|err| format!("Failed to read content of file '{}'.\nError: {err}", path.display()))
}

pub fn get_manifest_dir() -> Result<PathBuf, String> {
    Ok(env::var("CARGO_MANIFEST_DIR")
        .map_err(|err| format!("Failed to get environment variable 'CARGO_MANIFEST_DIR'.\nError:{err}"))?
        .into())
}

pub fn get_workspace_dir() -> Result<PathBuf, String> {
    let start_dir: PathBuf = match get_manifest_dir() {
        Ok(dir) => dir,
        Err(_) => std::env::current_dir()
            .expect("Failed to get current dir")
    };

    find_file_upwards(&start_dir, "Cargo.lock", 10)
        .ok_or_else(|| "Failed to find workspace root".to_string())
        .and_then(|lock_path| parent_dir(&lock_path))
}

pub fn get_target_dir() -> Result<PathBuf, String> {
    // Environment variable "CARGO_TARGET_DIR" is not set when running build script.
    // It is the way to configure cargo from the user side.
    // So we can't rely on this variable.
    let path = get_workspace_dir()?.join("target");
    Ok(path)
}

pub fn find_file_upwards(start_dir: &Path, filename: &str, max_search_depth: u32) -> Option<PathBuf> {
    // Start from the given directory
    let mut cur_dir = start_dir;

    for _level in 0..=max_search_depth {
        let file_path = cur_dir.join(filename);
        if file_path.is_file() {
            return Some(file_path)
        }

        // Go one level up after the each attempt
        cur_dir = cur_dir.parent()?;
    }

    None
}

pub fn find_files(root: &Path, recursively: bool, predicate: impl Fn(&Path) -> bool) -> Result<Vec<PathBuf>, String> {
    let mut dir_queue = Vec::from([PathBuf::from(root)]);
    let mut result = Vec::new();

    while let Some(dir_path) = dir_queue.pop() {
        let dir_entries = fs::read_dir(&dir_path)
            .map_err(|err| format!("Failed to get entries of directory '{}'.\nError: {err}", dir_path.display()))?;

        for entry in dir_entries {
            let entry = entry
                .map_err(|err| format!("Failed to get directory entry.\nError: {err}"))?;
            let metadata = entry.metadata()
                .map_err(|err| format!("Failed to get dir entry metadata.\nError:{err}"))?;
            let path = entry.path();
            if metadata.is_file() {
                if predicate(&path) {
                    result.push(path);
                }
            }
            else if metadata.is_dir() && recursively {
                dir_queue.push(path);
            }
        }
    }

    result.sort_unstable();

    Ok(result)
}

pub fn find_all_files(root: &Path, recursively: bool) -> Result<Vec<PathBuf>, String> {
    find_files(root, recursively, |_| true)
}

pub fn find_files_with_extension(dir: &Path, ext: &str) -> Result<Vec<PathBuf>, String> {
    find_files(dir, true,
        |path| path.extension().is_some_and(|path_ext| path_ext == ext))
}

pub fn get_relative_path(input: &Path, base: &Path) -> Result<PathBuf, String> {
    match input.strip_prefix(base) {
        Ok(result) => Ok(result.to_path_buf()),
        Err(_) => {
            let abs_input = absolute_path(input)?;
            let abs_base = absolute_path(base)?;
            abs_input.strip_prefix(abs_base)
                .map(|path| path.to_path_buf())
                .map_err(|err| format!("Failed to strip path '{}' from '{}'.\nError: {err}", input.display(), base.display()))
        }
    }
}

pub fn get_path_from(from_dir: &Path, to: &Path) -> Result<PathBuf, String> {
    let abs_from = absolute_path(from_dir)?;
    let abs_to = absolute_path(to)?;

    let equal_comps = abs_from.components()
        .zip(abs_to.components())
        .take_while(|(lhs, rhs)| lhs == rhs)
        .count();

    let mut result = PathBuf::from("./");
    abs_from.components()
        .skip(equal_comps)
        .for_each(|_| result.push(".."));

    abs_to.iter()
        .skip(equal_comps)
        .for_each(|comp| result.push(comp));

    Ok(result)
}

pub fn absolute_path(input: &Path) -> Result<PathBuf, String> {
    let absolute = path::absolute(input)
        .map_err(|err| format!("Failed to get absolute path from '{}'.\nError: {err}", input.display()))?;
    Ok(absolute)
}

/// Return path using '/' as directory separator
pub fn normalize_dir_separators(input: &Path) -> PathBuf {
    let str = input
        .as_os_str()
        .to_string_lossy()
        .to_string();
    str.replace('\\', "/")
        .into()
}

pub fn remove_file(path: &Path) -> Result<(), String> {
    fs::remove_file(path)
        .map_err(|err| format!("Failed to remove file '{}'.\nError: {err}", path.display()))
}

pub fn remove_dir_all(path: &Path) -> Result<(), String> {
    fs::remove_dir_all(path)
        .map_err(|err| format!("Failed to remove directory '{}'.\nError: {err}", path.display()))
}

pub fn parent_dir(path: &Path) -> Result<PathBuf, String> {
    Ok(path.parent()
        .ok_or_else(|| format!("Failed to get parent dir for {}", path.display()))?
        .to_path_buf())
}

pub fn create_dirs(path: &Path) -> Result<(), String> {
    fs::create_dir_all(path)
        .map_err(|err| format!("Failed to create output directory '{}': {err}", path.display()))
}
