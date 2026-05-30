// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use quote::{quote, ToTokens};
use syn::spanned::Spanned;

/// Generate impl Drop for given struct in which we delete attached qobject.
pub fn generate_drop(struct_ident: &syn::Ident, struct_generics: &syn::Generics) -> syn::Result<syn::ItemImpl> {

    let (impl_generics, type_generics, where_clause) = struct_generics.split_for_impl();

    let drop = syn::parse2(quote! {
        /// This is an automatic implementation by qtbridges.
        /// If you see E0119 about Drop, use #[qobject_impl(NoDrop)] or #[qobject(NoDrop)]
        /// Remember to call <Self as qtbridge::QObjectHolder>::detach_qobject(self) to
        /// avoid memory leaks
        impl #impl_generics Drop for #struct_ident #type_generics
        #where_clause
        {
            fn drop(&mut self) {
                <Self as qtbridge::qtbridge_runtime::QObjectHolder>::detach_qobject(self);
            }
        }
    })?;
    Ok(drop)
}

pub fn adjust_drop_impl(input: &syn::ItemImpl) -> syn::Result<syn::ItemImpl> {

    let items = &input.items;
    if items.len() != 1 {
        return Err(syn::Error::new(input.span(),
            "Drop traits expected to have one item"))
    }
    let item = &input.items[0];
    let syn::ImplItem::Fn(item_fn) = item else {
        return Err(syn::Error::new(input.span(),
            format!("Expected drop() function. Found: {}", item.to_token_stream())))
    };

    let mut new_item_fn = item_fn.clone();

    // Make sure the last expression has trailing semicolon
    if let Some(last_stmt) = new_item_fn.block.stmts.last_mut() &&
        let syn::Stmt::Expr(_expr, semi) = last_stmt {
        *semi = Some(Default::default());
    }

    let drop_expr: syn::Expr = syn::parse2(quote!{
        <Self as qtbridge::qtbridge_runtime::QObjectHolder>::detach_qobject(self)
    })?;
    new_item_fn.block.stmts.push(syn::Stmt::Expr(drop_expr, Some(Default::default())));

    Ok(syn::ItemImpl {
        items: vec![new_item_fn.into()],
        ..input.clone()
    })
}
