// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::collections::BTreeSet;
use proc_macro2::TokenStream;
use qt_gen_common_no_types::case_conv;
use syn::parse::Parser;

#[derive(Clone)]
pub struct Reexport {
    types: BTreeSet<syn::Ident>,
    others: BTreeSet<syn::Ident>,
}

impl Reexport {
    pub fn new() -> Self {
        Self {
            types: BTreeSet::new(),
            others: BTreeSet::new(),
        }
    }

    pub fn types(&self) -> &BTreeSet<syn::Ident> {
        &self.types
    }

    // Probably will be used later
    #[allow(dead_code)]
    pub fn others(&self) -> &BTreeSet<syn::Ident> {
        &self.others
    }

    pub fn all(&self) -> BTreeSet<syn::Ident> {
        self.types.iter()
            .chain(self.others.iter())
            .map(|ident| ident.clone())
            .collect()
    }

    pub fn collect_from_token_stream(&mut self, src: TokenStream) -> syn::Result<()> {
        let file = syn::parse2(src)?;
        self.collect_from_file(&file)
    }

    pub fn collect_from_file(&mut self, src: &syn::File) -> syn::Result<()> {
        src.items.iter()
            .try_for_each(|item| self.collect_from_item(item))
    }

    pub fn collect_from_item(&mut self, src: &syn::Item) -> syn::Result<()> {
        match src {
            syn::Item::Const(const_) => self.collect_other_if_pub(&const_.vis, &const_.ident),
            syn::Item::Enum(enum_) => self.collect_type_if_pub(&enum_.vis, &enum_.ident),
            syn::Item::Fn(fn_) => self.collect_other_if_pub(&fn_.vis, &fn_.sig.ident),
            syn::Item::Static(static_) => self.collect_other_if_pub(&static_.vis, &static_.ident),
            syn::Item::Struct(struct_) => self.collect_type_if_pub(&struct_.vis, &struct_.ident),
            syn::Item::Trait(trait_) => self.collect_other_if_pub(&trait_.vis, &trait_.ident),
            syn::Item::Type(type_) => self.collect_type_if_pub(&type_.vis, &type_.ident),
            syn::Item::Union(union_) => self.collect_type_if_pub(&union_.vis, &union_.ident),
            syn::Item::Use(use_) => self.collect_from_item_use(&use_.vis, &use_),
            syn::Item::Verbatim(tokens) => self.collect_from_verbatim(tokens.clone())?,
            _ => {},
        }
        Ok(())
    }

    fn collect_type_if_pub(&mut self, vis: &syn::Visibility, ident: &syn::Ident) {
        if is_pub_vis(vis) {
            self.types.insert(ident.clone());
        }
    }

    fn collect_other_if_pub(&mut self, vis: &syn::Visibility, ident: &syn::Ident) {
        if is_pub_vis(vis) {
            self.others.insert(ident.clone());
        }
    }

    fn collect_from_item_use(&mut self, vis: &syn::Visibility, src: &syn::ItemUse) {
        if !is_pub_vis(vis) {
            return
        }

        self.collect_from_use_tree(&src.tree);
    }

    fn collect_from_verbatim(&mut self, src: TokenStream) -> syn::Result<()> {
        let parser = |input: syn::parse::ParseStream| -> syn::Result<()> {
            let _attrs = input.call(syn::Attribute::parse_outer)?;
            let vis = input.parse::<syn::Visibility>()?;
            if is_pub_vis(&vis) {
                if input.peek(syn::Token![static]) {
                    input.parse::<syn::Token![static]>()?;
                    let ident: syn::Ident = input.parse()?;
                    self.others.insert(ident);
                }
                // TBD: more specific cases handled here
            }

            let _rest = input.parse::<TokenStream>();
            Ok(())

        };
        parser.parse2(src)
    }

    fn collect_from_use_tree(&mut self, src: &syn::UseTree) {
        match src {
            syn::UseTree::Path(path) => self.collect_from_use_tree(&path.tree),
            syn::UseTree::Name(name) => self.collect_from_use_name(name),
            syn::UseTree::Group(group) => {
                group.items.iter()
                    .for_each(|item| self.collect_from_use_tree(item));
            },
            _ => {}, // Unhandled for now
        }
    }

    fn collect_from_use_name(&mut self, src: &syn::UseName) {
        let ident = &src.ident;
        // Assume if name is in PascalCase then it's type
        if case_conv::is_pascal_case(&ident.to_string()) {
            self.types.insert(ident.clone());
        }
    }
}

fn is_pub_vis(vis: &syn::Visibility) -> bool {
    match vis {
        syn::Visibility::Public(_) => true,
        _ => false,
    }
}
