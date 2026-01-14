// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use super::type_traits::{StaticTypeGroup, TypeName, TypeInfo};

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct StandardContainer {
    rust_name: &'static str,
    cpp_name: Option<&'static str>,
    include: Option<&'static str>,
    generic_args: usize,
}

impl StandardContainer {
    const fn new(rust_name: &'static str, cpp_name: Option<&'static str>, include: Option<&'static str>, generic_args: usize) -> Self {
        Self { rust_name, cpp_name, include, generic_args }
    }

    pub fn dyn_type_info(&self) -> &dyn TypeInfo {
        self
    }
}

impl TypeName for StandardContainer {
    fn name(&self) -> &str {
        self.rust_name
    }

    fn path_before_name(&self) -> Option<&str> {
        None
    }
}

impl TypeInfo for StandardContainer {
    fn cpp_name(&self) -> Option<&'static str> {
        self.cpp_name.clone()
    }

    fn cpp_include(&self) -> Option<String> {
        self.include.map(String::from)
    }

    fn generic_arg_count(&self) -> usize {
        self.generic_args
    }
}

impl StaticTypeGroup for StandardContainer {
    fn get_static_sorted_list() -> &'static [Self] {
                static LIST: [StandardContainer; 1] = [
            StandardContainer::new("Vec", Some("rust::Vec"), Some(r#""rust/cxx.h""#), 1),
        ];

        &LIST
    }
}
