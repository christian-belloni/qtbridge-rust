// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use quote::ToTokens;
use syn::parse::discouraged::Speculative;
use syn::spanned::Spanned;

use qt_gen_common::cpp_include::CppInclude;
use qt_gen_common::parse_utils::find_token;

use crate::function::Function;
use crate::structure::BridgeStruct;
use crate::trait_impl::TraitImpl;

/// Enum for handling different kinds of items in the `qt_gen::bridge` module
/// in a generic way (parsing, attribute handling, and fallback to `syn::Item` if parsing fails).
pub(crate) enum ModuleItem {
    Include(CppInclude),
    Struct(BridgeStruct),
    Func(Function),
    TraitImpl(TraitImpl),
    Other(syn::Item),
}

impl ModuleItem {
    fn set_attributes(&mut self, attrs: &[syn::Attribute]) -> syn::Result<()> {
        let Some(first_attr) = attrs.first() else {
            return Ok(())
        };

        match self {
            Self::Struct(struct_) => struct_.set_attributes(attrs),
            Self::Func(func) => func.set_attributes(attrs),
            Self::TraitImpl(trait_impl) => trait_impl.set_attributes(attrs),
            _ => Err(syn::Error::new(first_attr.span(), "Attribute is unsupported for this Module item"))
        }
    }
}

/// Parse item of one of the following custom types:
/// CppInclude, BridgeStruct, Function, TraitImpl.
/// Return Ok(None) if input item is not one of them.
fn parse_custom_item(input: syn::parse::ParseStream) -> syn::Result<Option<ModuleItem>> {
    let fork = input.fork();

    let attrs = fork.call(syn::Attribute::parse_outer)?;

    let mut result: Option<ModuleItem> = if CppInclude::is_for_me(&fork) {
        let include: CppInclude = fork.parse()?;
        Some(ModuleItem::Include(include))
    }
    else if BridgeStruct::is_for_me(&fork) {
        let struct_: BridgeStruct = fork.parse()?;
        Some(ModuleItem::Struct(struct_))
    }
    else if Function::is_for_me(&fork) {
        let func: Function = fork.parse()?;
        Some(ModuleItem::Func(func))
    }
    else if TraitImpl::is_for_me(&fork) {
        let trait_: TraitImpl = fork.parse()?;
        Some(ModuleItem::TraitImpl(trait_))
    }
    else {
        None
    };

    if let Some(item) = result.as_mut() {
        if !attrs.is_empty() {
            item.set_attributes(&attrs)?;
        }

        input.advance_to(&fork);
    }
    Ok(result)
}

impl syn::parse::Parse for ModuleItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item_fork = input.fork();
        let item = match parse_custom_item(&item_fork) {
            Ok(Some(custom_item)) => {
                // Custom item parsed successfully.
                input.advance_to(&item_fork);
                custom_item
            },
            Ok(None) => {
                // Not a custom item. Parse it as regular syn::Item.
                let regular_item: syn::Item = input.parse()?;
                ModuleItem::Other(regular_item)
            },
            Err(err) => {
                // Parsing as custom item has failed.
                // Try to parse as a regular syn::Item.
                let regular_item: syn::Item = input.parse()?;
                if find_token(regular_item.to_token_stream(),
                    &|token| token == "cpp_fn").is_some() {
                        return Err(syn::Error::new(err.span(),
                            format!("Found cpp_fn! but failed to parse as qtgen::bridge item.\nError: {err}")));
                }
                ModuleItem::Other(regular_item)
            }
        };

        Ok(item)
    }
}

