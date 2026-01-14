// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use super::type_traits::{StaticTypeGroup, TypeInfo, TypeName};

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct ValueHolder {
    rust_name: &'static str,
    generic_args: usize,
}

impl ValueHolder {
    const fn new(rust_name: &'static str,  generic_args: usize) -> Self{
        Self { rust_name, generic_args }
    }

    pub fn dyn_type_info(&self) -> &dyn TypeInfo {
        self
    }
}

impl TypeName for ValueHolder {
    fn name(&self) -> &str {
        self.rust_name
    }

    fn path_before_name(&self) -> Option<&str> {
        None
    }
}

impl TypeInfo for ValueHolder {
    fn generic_arg_count(&self) -> usize {
        self.generic_args
    }
}

impl StaticTypeGroup for ValueHolder {
    fn get_static_sorted_list() -> &'static [Self] {
        static LIST: [ValueHolder; 2] = [
            ValueHolder::new("Result", 1), // TODO: make it handle also the case when second generic argument is specified
            ValueHolder::new("Option", 1),
        ];

        &LIST
    }
}
