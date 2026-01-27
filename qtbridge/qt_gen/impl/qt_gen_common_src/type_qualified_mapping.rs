// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use quote::ToTokens;
use syn::visit_mut::VisitMut;
use syn::{Path, parse_quote, PathSegment, Ident};
use crate::type_registry;
use type_registry::{TypesEnum, type_traits::FindType};

pub struct TypeQualifiedMapping {
    source: CallOrigin,
    last_error: syn::Result<()>,
}

#[derive(Clone)]
pub enum CallOrigin {
    Internal,
    External,
}

impl CallOrigin {
    pub fn type_module(&self) -> Path {
        match self {
            CallOrigin::External => parse_quote!(qtbridge::qt_type_lib),
            CallOrigin::Internal => parse_quote!(qt_type_lib),
        }
    }

    pub fn iface_module(&self) -> Path {
        match self {
            CallOrigin::External => parse_quote!(qtbridge::qt_ifaces),
            CallOrigin::Internal => parse_quote!(qt_ifaces),
        }
    }

    pub fn bridge_module(&self) -> Path {
        match self {
            CallOrigin::External => parse_quote!(qtbridge::bridge),
            CallOrigin::Internal => parse_quote!(bridge),
        }
    }

    pub fn trait_module(&self) -> Path {
        match self {
            CallOrigin::External => parse_quote!(qtbridge::qt_traits),
            CallOrigin::Internal => parse_quote!(qt_traits),
        }
    }

}

impl TypeQualifiedMapping {
    pub fn new(mapping: CallOrigin) -> Self {
        Self {
            source: mapping,
            last_error: Ok(())
        }
    }

    pub fn result(&self) -> syn::Result<()> {
        self.last_error.clone()
    }

    fn do_visit_path(&mut self, src: &mut syn::Path) -> syn::Result<()> {
        if src.get_ident().is_some_and(|ident| ident == "Self") {
            return Ok(())
        }

        let ty = type_registry::Type::find_by_partial_path_result(src)
            .map_err(|err| syn::Error::new(err.span(),
                format!("Failed to get path '{}' fully qualified.\nError: {err}", src.to_token_stream())))?;
        let mut new = ty.dyn_type_info().complement_partially_qualified_path(src)?;
        if let CallOrigin::External = &self.source {
            if let Some(first_seg) = new.segments.first() {
                if first_seg.ident == "qt_type_lib" || first_seg.ident == "qt_ifaces" {
                    new.segments.insert(
                        0,
                        PathSegment {
                            ident: Ident::new("qtbridge", first_seg.ident.span()),
                            arguments: syn::PathArguments::None,
                        },
                    );
                }
            }
        }

        let segs_added = new.segments.len() - src.segments.len();
        new.segments.iter_mut()
            .skip(segs_added)
            .for_each(|seg: &mut PathSegment| self.visit_path_segment_mut(seg));
        *src = new;

        Ok(())
    }
}

impl VisitMut for TypeQualifiedMapping {
    fn visit_path_mut(&mut self, src: &mut syn::Path) {
        if let Err(err) = self.do_visit_path(src) {
            self.last_error = Err(err);
        }
    }
}
