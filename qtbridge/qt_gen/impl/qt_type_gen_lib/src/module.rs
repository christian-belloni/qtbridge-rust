// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use syn::Token;

use qt_gen_common_no_types::cpp_include::CppInclude;
use qt_gen_common_no_types::parse_utils::is_two_segment_path_outer_attribute;

use crate::function::Function;
use crate::module_item::ModuleItem;
use crate::structure::BridgeStruct;
use crate::trait_impl::TraitImpl;


#[derive(Clone)]
pub struct Module {
    ident: syn::Ident,
    cpp_includes: Vec<CppInclude>,
    structure: Option<BridgeStruct>,
    functions: Vec<Function>,
    traits: Vec<TraitImpl>,
    other_items: Vec<syn::Item>,
}

impl Module {
    pub fn ident(&self) -> &syn::Ident {
        &self.ident
    }

    pub fn cpp_includes(&self) -> &Vec<CppInclude> {
        &self.cpp_includes
    }

    pub fn structure(&self) -> Option<&BridgeStruct> {
        self.structure.as_ref()
    }

    pub fn functions(&self) -> &Vec<Function> {
        &self.functions
    }

    pub fn traits(&self) -> &[TraitImpl] {
        &self.traits
    }

    pub fn other_items(&self) -> &Vec<syn::Item> {
        &self.other_items
    }

    pub fn is_mine_attr(attr: &syn::Attribute) -> bool {
        is_two_segment_path_outer_attribute(attr, "qt_gen", "bridge")
    }
}

impl syn::parse::Parse for Module {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _mod_attrs = input.call(syn::Attribute::parse_outer)?;
        let _vis: syn::Visibility = input.parse()?;
        let _mod: Token![mod] = input.parse()?;
        let ident: syn::Ident = input.parse()?;

        if !input.peek(syn::token::Brace) {
            return Err(input.error("Expected '{' here"));
        }

        let content;
        let _braces = syn::braced!(content in input);

        let mut structure = None;
        let mut cpp_includes = Vec::new();
        let mut functions = Vec::new();
        let mut traits = Vec::new();
        let mut other_items = Vec::new();

        while !content.is_empty() {
            let item_begin = content.fork();

            match content.parse::<ModuleItem>()? {
                ModuleItem::Include(include) => cpp_includes.push(include),
                ModuleItem::Struct(struct_) => {
                    if structure.is_some() {
                        return Err(item_begin.error("'struct' may be defined at most once per bridge module block"))
                    }
                    structure = Some(struct_);
                },
                ModuleItem::Func(function) => functions.push(function),
                ModuleItem::TraitImpl(trait_impl) => traits.push(trait_impl),
                ModuleItem::Other(item) => other_items.push(item),
            }
        }

        Ok(Self {
            ident,
            cpp_includes,
            structure,
            functions,
            traits,
            other_items,
        })
    }
}
