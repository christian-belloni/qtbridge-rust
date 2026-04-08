// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use super::type_traits::{StaticTypeGroup, TypeInfo, TypeName};

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct CxxType {
    rust_name: &'static str,
    path_before_name: Option<&'static str>,
    cpp_name: Option<&'static str>,
    include: Option<&'static str>,
    generic_args: usize,
}

impl CxxType {
    const fn new(rust_name: &'static str,
        path_before_name: Option<&'static str>,
        cpp_name: Option<&'static str>,
        include: Option<&'static str>,
        generic_args: usize) -> Self {
            CxxType { rust_name, path_before_name, cpp_name, include, generic_args }
        }

    pub fn dyn_type_info(&self) -> &dyn TypeInfo {
        self
    }

    pub fn mut_dyn_type_info(&mut self) -> &mut dyn TypeInfo {
        self
    }
}

impl TypeName for CxxType {
    fn name(&self) -> &str {
        self.rust_name
    }

    fn path_before_name(&self) -> Option<&str> {
        self.path_before_name
    }
}

impl TypeInfo for CxxType {
    fn cpp_name(&self) -> Option<&'static str> {
        self.cpp_name
    }

    fn cpp_include(&self) -> Option<String> {
        self.include.map(String::from)
    }

    fn generic_arg_count(&self) -> usize {
        self.generic_args
    }
}

impl StaticTypeGroup for CxxType {
    fn get_static_sorted_list() -> &'static [Self] {
        static LIST: [CxxType; 2] = [
            CxxType::new("Pin", Some("std::pin"), None, None, 1), // Not really a type from CXX but type that CXX uses a lot
            CxxType::new("UniquePtr", Some("cxx"), Some("std::unique_ptr"), Some("<memory>"), 1),
        ];

        &LIST
    }
}
