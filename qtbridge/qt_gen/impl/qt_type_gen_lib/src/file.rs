// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use syn::spanned::Spanned;
use qt_gen_common_no_types::parse_utils::{is_cxx_bridge_attribute, partition_attr_by};
use crate::module::Module;

pub enum Item {
    QtBridge(Module),
    CxxBridge(syn::ItemMod),
    Other(syn::Item),
}

impl Item {
    pub fn is_cxx_bridge(&self) -> bool {
        match self {
            Item::CxxBridge(_) => true,
            _ => false,
        }
    }
}

pub struct File {
    _attrs: Vec<syn::Attribute>,
    items: Vec<Item>,
}

impl File {
    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }
}

impl syn::parse::Parse for File {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_inner)?;
        let mut items = Vec::new();
        let mut qt_bridge_parsed = false;
        while !input.is_empty() {
            let fork = input.fork();

            let mut attrs = fork.call(syn::Attribute::parse_outer)?;

            let item = if !attrs.is_empty() {
                let qt_mod_attr;
                (attrs, qt_mod_attr) = partition_attr_by(attrs, Module::is_mine_attr);
                if let Some(attr) = qt_mod_attr {
                    if qt_bridge_parsed {
                        return Err(syn::Error::new(attr.span(), "Only one #[qt_gen::bridge] per file is allowed"))
                    }
                    qt_bridge_parsed = true;
                    Item::QtBridge(input.parse()?)
                }
                else {
                    let cxx_mod_attr;
                    (_, cxx_mod_attr) = partition_attr_by(attrs, is_cxx_bridge_attribute);
                    if cxx_mod_attr.is_some() {
                        Item::CxxBridge(input.parse()?)
                    }
                    else {
                        Item::Other(input.parse()?)
                    }
                }
            }
            else {
                Item::Other(input.parse()?)
            };
            items.push(item);
        }

        Ok(File {
            _attrs: attrs,
            items,
        })
    }
}

