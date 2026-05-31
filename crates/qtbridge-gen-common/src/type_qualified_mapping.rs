// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::visit_mut::VisitMut;
use crate::type_registry;
use type_registry::QtType;
use type_registry::type_traits::{FindType, TypeName};

// TODO:
// - Rename to QtBridgeQualifiedMapping.
// - Split into QtBridgeQualifiedMapping and TypeQualifiedMapping.
pub struct TypeQualifiedMapping {
    last_error: syn::Result<()>,
}

impl TypeQualifiedMapping {
    pub fn result(&self) -> syn::Result<()> {
        self.last_error.clone()
    }

    fn do_visit_path(&mut self, src: &mut syn::Path) -> syn::Result<()> {
        if src.get_ident().is_some_and(|ident| ident == "Self") {
            return Ok(())
        }

        let Some(first_seg) = src.segments.first() else {
            return Ok(())
        };

        let mut new = src.clone();
        let mut add_qtbridge = false;
        let mut add_type_lib = false;
        let first_seg_ident_str = first_seg.ident.to_string();
        match first_seg_ident_str.as_str() {
            "qtbridge" => {
                // Don't add any segments. The path itself is already fully qualified.
            },
            "qtbridge_type_lib" |
            "qtbridge_interfaces" |
            "qtbridge_runtime" =>
                // The path must be prefixed with `qtbridge` to make it fully qualified.
                add_qtbridge = true,
            _ => {
                if let Some(type_) = type_registry::Type::find_by_path(src) {
                    // It is a type path.
                    new = type_.complement_partially_qualified_path(&src)?;
                    if matches!(type_, type_registry::Type::Qt(_)) {
                        add_qtbridge = true;
                    }
                }
                else if let Some(_) = QtType::find_by_name(&first_seg_ident_str) {
                    // Some Qt type is in a segment of a syn::Path
                    // but not the whole path. E.g.: QMetaType::default().
                    add_type_lib = true;
                }
            }
        }
        if add_type_lib {
            new.segments.insert(0, syn::PathSegment {
                ident: format_ident!("qtbridge_type_lib"),
                arguments: syn::PathArguments::None
            });
            add_qtbridge = true;
        }
        if add_qtbridge && first_seg_ident_str != "qtbridge" {
            new.segments.insert(0, syn::PathSegment {
                ident: format_ident!("qtbridge"),
                arguments: syn::PathArguments::None
            });
        }

        // Iterate old components of the path.
        let segs_added = new.segments.len() - src.segments.len();
        new.segments.iter_mut()
            .skip(segs_added)
            .for_each(|seg| self.visit_path_segment_mut(seg));
        *src = new;

        Ok(())
    }

    fn try_macro_token_stream_as_stmts(&mut self, src: &mut TokenStream) -> syn::Result<()> {
        let mut stmts = syn::Block::parse_within.parse2(src.clone())?;
        let mut result = TokenStream::new();
        stmts.iter_mut()
            .for_each(|stmt| {
                self.visit_stmt_mut(stmt);
                stmt.to_tokens(&mut result);
            });
        *src = result;
        Ok(())
    }

    fn try_macro_token_stream_as_punctuated_expr(&mut self, src: &mut TokenStream) -> syn::Result<()> {
        let mut args = Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated.parse2(src.clone())?;
        let mut result = TokenStream::new();
        args.iter_mut()
            .for_each(|expr| {
                self.visit_expr_mut(expr);
                expr.to_tokens(&mut result);
            });

        Ok(())
    }

}

impl Default for TypeQualifiedMapping {
    fn default() -> Self {
        Self {
            last_error: Ok(())
        }
    }
}

impl VisitMut for TypeQualifiedMapping {
    fn visit_expr_path_mut(&mut self, src: &mut syn::ExprPath) {
        // Need to handle `syn::ExprPath` to adjust `qself` after adding new path segments.

        let segs_before = src.path.segments.len();
        syn::visit_mut::visit_expr_path_mut(self, src);

        // Shift `QSelf.position` by the number of path segments added.
        if let Some(qself) = src.qself.as_mut() {
            let segs_added = src.path.segments.len() - segs_before;
            qself.position += segs_added;
        }
    }

    fn visit_type_path_mut(&mut self, src: &mut syn::TypePath) {
        let segs_before = src.path.segments.len();
        syn::visit_mut::visit_type_path_mut(self, src);

        // Shift `QSelf.position` by the number of path segments added.
        if let Some(qself) = src.qself.as_mut() {
            let segs_added = src.path.segments.len() - segs_before;
            qself.position += segs_added;
        }
    }

    fn visit_path_mut(&mut self, src: &mut syn::Path) {
        if let Err(err) = self.do_visit_path(src) {
            self.last_error = Err(err);
        }
    }

    fn visit_token_stream_mut(&mut self, src: &mut TokenStream) {
        // Need to handle TokenStreams to support a code inside macros.

        let result = self.try_macro_token_stream_as_stmts(src)
            .or_else(|_| self.try_macro_token_stream_as_punctuated_expr(src));
        if result.is_err() {
            self.last_error = result;
        }
    }
}
