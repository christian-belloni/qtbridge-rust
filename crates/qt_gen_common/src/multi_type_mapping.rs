// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::collections::BTreeMap;

use crate::type_mapping::TypeMapping;

#[derive(Clone, Default)]
/// Map where
///   Key - generic param name (e.g. 'T')
///   Value - concrete type that this param takes (e.g. 'String')
pub struct MultiTypeMapping {
    map: BTreeMap<syn::Ident, syn::Path>,
}

impl<'a> MultiTypeMapping {
    pub fn new(map: BTreeMap<syn::Ident, syn::Path>) -> Self {
        Self {
            map
        }
    }

    pub fn iter(&'a self) -> impl Iterator<Item = (&'a syn::Ident, &'a syn::Path)> {
         self.map.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn extend<'f>(&mut self, src: impl Iterator<Item = (&'f syn::Ident, &'f syn::Path)>) {
        self.map.extend(src.map(|(gen_ident, gen_type)| (gen_ident.clone(), gen_type.clone())))
    }
}

impl From<BTreeMap<syn::Ident, syn::Path>> for MultiTypeMapping {
    fn from(map: BTreeMap<syn::Ident, syn::Path>) -> Self {
        Self {
            map
        }
    }
}

impl TypeMapping for MultiTypeMapping {
    fn map(&self, key: &syn::Ident) -> Option<syn::Path> {
        self.map.get(key).cloned()
    }
}
