// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use syn::punctuated::Punctuated;
use syn::visit_mut::VisitMut;

use crate::type_mapping::TypeMapping;


// TypeMapping that works with more high level (potentially recursive) AST entities.
pub struct TypeMappingNested<T: TypeMapping> {
    /// Object implementing mapping
    map_impl: T,

}

impl<T: TypeMapping> TypeMappingNested<T> {
    pub fn new(map_impl: T) -> Self {
        Self {
            map_impl
        }
    }

    pub fn get_impl(&self) -> &T {
        &self.map_impl
    }

    pub fn map_impl_item(&self, src: &syn::ImplItem) -> syn::ImplItem {
        let mut result = src.clone();
        let mut v = Visitor::new(self.get_impl());
        v.visit_impl_item_mut(&mut result);
        result
    }

    pub fn map_item_fn(&self, src: &syn::ItemFn) -> syn::ItemFn {
        let mut result = src.clone();
        let mut v = Visitor::new(self.get_impl());
        v.visit_item_fn_mut(&mut result);
        result
    }

    pub fn map_signature(&self, src: &syn::Signature) -> syn::Signature {
        let mut result = src.clone();
        let mut v = Visitor::new(self.get_impl());
        v.visit_signature_mut(&mut result);
        result
    }

    pub fn map_path(&self, src: &syn::Path) -> syn::Path {
        let mut result = src.clone();
        let mut v = Visitor::new(self.get_impl());
        v.visit_path_mut(&mut result);
        result
    }
}

struct Visitor<'a, T: TypeMapping> {
    map: &'a T
}

impl<'a, T: TypeMapping> Visitor<'a, T> {
    pub fn new(map: &'a T) -> Self {
        Self {
            map
        }
    }
}

impl<'a, T: TypeMapping> VisitMut for Visitor<'a, T> {
    fn visit_path_mut(&mut self, src: &mut syn::Path) {
        let mut new_segs = Punctuated::new();

        for seg in &src.segments {
            match seg.arguments {
                // We map either segment ident or segment argument but not both
                syn::PathArguments::None => {
                    let mapped = self.map.map(&seg.ident);
                    match mapped {
                        Some(mapped_path) => new_segs.extend(mapped_path.segments),
                        None => new_segs.push(seg.clone()),
                    }
                },
                _ => {
                    let mut new_seg = seg.clone();
                    self.visit_path_segment_mut(&mut new_seg);
                    new_segs.push(new_seg);
                }
            }
        }

        src.segments = new_segs;
    }
}
