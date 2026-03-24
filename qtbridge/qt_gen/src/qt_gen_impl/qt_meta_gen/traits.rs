// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::collections::BTreeSet;

use proc_macro2::{Span, TokenStream};

pub trait QmlName {
    fn get_qml_name_span(&self) -> (String, Span);
}

pub(crate) fn find_by_qml_name<'a, T: QmlName>(name: &str, list: &'a[T]) -> Option<&'a T> {
    list.iter().find(|o| o.get_qml_name_span().0 == name)
}

// TODO: move this function somewhere else
pub(crate) fn find_duplicate_by<T, U: Ord>(list: &[T], pred: fn (&T)->U) -> Option<&T> {
    if !list.is_empty() {
        let mut unique = BTreeSet::<U>::new();
        for item in list {
            let value = pred(item);
            if !unique.insert(value) {
                return Some(item);
            }
        }

    }
    None
}

pub fn find_duplicate_by_qml_name<T: QmlName>(list: &[T]) -> Option<&T> {
    find_duplicate_by(list, |item| {
        let (name, _span) = item.get_qml_name_span();
        name
    })
}

pub trait ExpandTokens {
    fn expand_tokens(&self) -> syn::Result<TokenStream>;
}
