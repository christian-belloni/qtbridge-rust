// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use super::type_traits::{StaticTypeGroup, TypeInfo, TypeName};

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct CellType {
    rust_name: &'static str,
}

impl CellType {
    const fn new(rust_name: &'static str) -> Self {
        Self {
            rust_name,
        }
    }

    pub fn dyn_type_info(&self) -> &dyn TypeInfo {
        self
    }

    pub fn mut_dyn_type_info(&mut self) -> &mut dyn TypeInfo {
        self
    }
}

impl TypeName for CellType {
    fn name(&self) -> &str {
        self.rust_name
    }

    fn path_before_name(&self) -> Option<&str> {
        Some("std::cell")
    }
}

impl TypeInfo for CellType {
    fn generic_arg_count(&self) -> usize {
        1
    }
}

impl StaticTypeGroup for CellType {
    fn get_static_sorted_list() -> &'static [Self] {
        static LIST: [CellType; 1] = [
            CellType::new("RefCell")
        ];

        LIST.as_slice()
    }
}
