// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge_gen_common::type_mapping::TypeMapping;

pub struct SelfTypeMapping {
    mapped: syn::Type,
}

impl SelfTypeMapping {
    pub fn new(to: syn::Type) -> Self {
        Self {
            mapped: to
        }
    }
}

impl TypeMapping for SelfTypeMapping {
    fn map(&self, key: &syn::Ident) -> Option<syn::Type> {
        if key == "Self" {
            return Some(self.mapped.clone());
        }
        None
    }
}
