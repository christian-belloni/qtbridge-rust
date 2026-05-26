// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]
#![allow(dead_code)]

// Constants from JavaScript
pub const MIN_SAFE_INTEGER: i64 = -9007199254740991;
pub const MAX_SAFE_INTEGER: u64 = 9007199254740991;


pub fn get_type_name<T: ?Sized>() -> String {
    let mut path_components: Vec<_> = std::any::type_name::<T>()
        .split(&[':', '<', '>'])
        .collect();

    // Remove additional path components but the type itself.
    // E.g. alloc::vec::Vec<alloc::string::String> -> Vec<String>.
    if path_components.len() > 1 {
        path_components.retain(|comp| {
            !["", "alloc", "string", "vec"].contains(comp)
        });
    }
    path_components.into_iter()
        .map(capitalize_first_char)
        .collect::<Vec<_>>()
        .join("")
}

pub fn capitalize_first_char(str: &str) -> String {
    let mut chars = str.chars();
    format!("{}{}", chars.next().unwrap().to_uppercase(), chars.as_str())
}

pub fn decapitalize_first_char(str: &str) -> String {
    let mut chars = str.chars();
    format!("{}{}", chars.next().unwrap().to_lowercase(), chars.as_str())
}
