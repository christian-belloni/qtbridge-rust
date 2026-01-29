// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::Span;
use syn::parse::Parse;

/// The attribute marking the `struct` as being supported by `QMetaType` system.
///
/// It can be applied to a non-generic struct:
/// ```ignore
/// #[qmetatype = 12]
/// struct QByteArray {
///     ...
/// }
/// ```
///
/// Or in instantiation of a generic struct:
/// ```ignore
/// #[instantiate_for[
///     ((QString, QVariant), qmetatype = 8),
/// ]]
/// struct QMap<K, V> {
///     ...
/// }
/// ```
///
/// Explicit QMetaType `id` may be omitted if it is not defined as constant.
///
#[derive(Clone)]
pub struct QMetaTypeAttribute {
    /// If `id` is `Some`, it holds the constant with predefined QMetaType `id` for given type.
    /// `None` - the type can be treated with QMetaType but has no constant `id`. `id` is assigned to that type at runtime.
    id: Option<syn::LitInt>,
}

impl QMetaTypeAttribute {
    pub fn id(&self) -> Option<i32> {
        self.id.as_ref()
            .map(|lit| lit.base10_parse().ok())
            .flatten()
    }

    pub fn id_span(&self) -> Option<Span> {
        self.id.as_ref()
            .map(|lit| lit.span())
    }

    pub fn new_without_id() -> Self {
        Self {
            id: None,
        }
    }
}

impl Parse for QMetaTypeAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let keyword: syn::Ident = input.parse()?;
        if keyword != "qmetatype" {
            return Err(syn::Error::new(keyword.span(), "Expected 'qmetatype' here"))
        }
        let result = match input.peek(syn::Token![=]) {
            true => {
                let _eq: syn::Token![=] = input.parse()?;
                let id: syn::LitInt = input.parse()?;
                QMetaTypeAttribute {
                    id: Some(id)
                }
            }
            false => QMetaTypeAttribute {
                id: None
            }
        };
        Ok(result)
    }
}
