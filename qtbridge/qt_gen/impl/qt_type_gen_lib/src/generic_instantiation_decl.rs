// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::Span;
use syn::parse::Parse;
use syn::parse::discouraged::Speculative;
use syn::punctuated::Punctuated;

use qt_gen_common_no_types::multi_type_mapping::MultiTypeMapping;
use qt_gen_common_no_types::parse_utils::parse_name_value;

use crate::generic_idents::GenericIdents;
use crate::qmetatype_attribute::QMetaTypeAttribute;

/// Type (single or tuple) for which structure will be instantiated
#[derive(Clone)]
pub struct GenericInstantiationTypes {
    types: Vec<syn::Path>,
    span: Span,
}

impl GenericInstantiationTypes {
    pub fn list(&self) -> &[syn::Path] {
        &self.types
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn get_type_map_for(&self, idents: &GenericIdents) -> syn::Result<MultiTypeMapping> {
        if self.list().len() != idents.list().len() {
            return Err(syn::Error::new(self.span(), "Mismatch in number of items of struct generics and instantiation types"));
        }

        let map = idents.list().iter().cloned()
            .zip(self.list().iter().cloned())
            .collect();
        Ok(MultiTypeMapping::new(map))
    }
}

impl PartialEq for GenericInstantiationTypes {
    fn eq(&self, other: &Self) -> bool {
        // We don't compare spans here
        self.types == other.types
    }
}

impl Parse for GenericInstantiationTypes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            let span = content.span();
            let types = Punctuated::<_, syn::Token![,]>::parse_separated_nonempty(&content)?
                .into_iter()
                .collect();
            return Ok(Self {
                types,
                span,
            })
        }

        let span = input.span();
        Ok(Self {
            types: vec![input.parse()?],
            span,
        })
    }
}



#[derive(Clone)]
/// Description of single instantiation
pub struct GenericInstantiationDecl {
    types: GenericInstantiationTypes,
    alias: Option<syn::Ident>,
    qmetatype: Option<QMetaTypeAttribute>,
}

impl GenericInstantiationDecl {
    pub fn types(&self) -> &GenericInstantiationTypes {
        &self.types
    }

    pub fn alias(&self) -> Option<&syn::Ident> {
        self.alias.as_ref()
    }

    pub fn qmetatype_id(&self) -> Option<&QMetaTypeAttribute> {
        self.qmetatype.as_ref()
    }

    fn parse_alias_and_qmetatype(input: syn::parse::ParseStream) -> syn::Result<(Option<syn::Ident>, Option<QMetaTypeAttribute>)> {
        let mut alias = None;
        let mut qmetatype = None;

        while !input.is_empty() && input.peek(syn::Ident) {
            let keyword: syn::Ident = input.fork().parse()?;
            match keyword.to_string().as_str() {
                "alias" => {
                    alias = Some(parse_name_value::<syn::Ident, syn::Ident>(input)?.1)
                },
                "qmetatype" => {
                    qmetatype = Some(input.parse()?);
                }
                _ => return Err(syn::Error::new(keyword.span(), format!("Unexpected ident: '{keyword}'"))),
            }
            if input.is_empty() {
                break;
            }
            input.parse::<syn::Token![,]>()?;
        }

        Ok((alias, qmetatype))
    }
}

impl Parse for GenericInstantiationDecl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::token::Paren) {
            let fork = input.fork();

            let content;
            syn::parenthesized!(content in fork);
            if content.peek(syn::token::Paren) {
                // 2 nested parenthesis:
                // the outer scopes delimits whole item declaration
                // the inner scope delimits type list

                let types = content.parse()?;
                if !content.is_empty() {
                    content.parse::<syn::Token![,]>()?;
                }
                let (alias, qmetatype) = match content.is_empty() {
                    true => (None, None),
                    false => Self::parse_alias_and_qmetatype(&content)?,
                };
                input.advance_to(&fork);
                return Ok(Self {
                    types,
                    alias,
                    qmetatype
                })
            }
        }

        Ok(Self {
            types: input.parse()?,
            alias: None,
            qmetatype: None,
        })

    }
}


#[derive(Clone)]
pub struct GenericInstantiationsList {
    list: Vec<GenericInstantiationDecl>,
}

impl GenericInstantiationsList {
    pub fn check_size(&self, size: usize) -> syn::Result<()> {
        for inst in &self.list {
            let inst_len = inst.types().list().len();
            if inst_len != size {
                return Err(syn::Error::new(inst.types().span(), format!("Mismatch in generic parameter count: {inst_len} vs {size}")));
            }
        }

        Ok(())
    }

    pub fn list(&self) -> &[GenericInstantiationDecl] {
        &self.list
    }
}

impl Parse for GenericInstantiationsList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {

        let mut list = Vec::<GenericInstantiationDecl>::new();
        while !input.is_empty() {
            list.push(input.parse()?);
            if input.is_empty() {
                break;
            }
            input.parse::<syn::Token![,]>()?;
        }

        Ok(Self {
            list
        })
    }
}


/// Struct to hold inclusions/exclusions from the set of instantiation types.
/// Used from `#[include_if_struct_instantiation]` and `#[exclude_if_struct_instantiation]` attributes.
#[derive(Clone)]
pub struct GenericInstantiationTypesList {
    list: Vec<GenericInstantiationTypes>,
}

impl GenericInstantiationTypesList {
    pub fn list(&self) -> &[GenericInstantiationTypes] {
        &self.list
    }

    pub fn check_size(&self, size: usize) -> syn::Result<()> {
        for inst in &self.list {
            let inst_len = inst.list().len();
            if inst_len != size {
                return Err(syn::Error::new(inst.span(), format!("Mismatch in generic parameter count: {inst_len} vs {size}")));
            }
        }

        Ok(())
    }
}

impl Parse for GenericInstantiationTypesList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            list: Punctuated::<_, syn::Token![,]>::parse_separated_nonempty(input)?
                .into_iter()
                .collect()
        })
    }
}
