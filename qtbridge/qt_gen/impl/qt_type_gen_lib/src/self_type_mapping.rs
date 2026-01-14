// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qt_gen_common_no_types::type_mapping::TypeMapping;

pub struct SelfTypeMapping {
    mapped: syn::Path,
}

impl SelfTypeMapping {
    pub fn new(to: syn::Path) -> Self {
        Self {
            mapped: to
        }
    }
}

impl TypeMapping for SelfTypeMapping {
    fn map(&self, key: &syn::Ident) -> Option<syn::Path> {
        if key == "Self" {
            return Some(self.mapped.clone());
        }
        None
    }
}
