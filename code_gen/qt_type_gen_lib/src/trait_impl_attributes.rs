// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use syn::spanned::Spanned;
use crate::generic_instantiation_decl::{GenericInstantiationTypesList, GenericInstantiationsList};

#[derive(Clone)]
pub struct TraitImplAttributes {
    /// List of types for which this trait will be instantiated.
    /// Not to be confused with the struct instantiation.
    instantiations: Option<GenericInstantiationsList>,

    /// List of struct instantiation types for which this `TraitImpl` must be included in the generated code.
    instantiation_inclusions: Option<GenericInstantiationTypesList>,

    /// List of struct instantiation types for which this `TraitImpl` must not be included in the generated code.
    instantiation_exclusions: Option<GenericInstantiationTypesList>,

    /// List of doc attributes
    docs: Vec<syn::Attribute>,
}

impl TraitImplAttributes {
    pub fn new(attrs: &[syn::Attribute]) -> syn::Result<Self> {
        let mut instantiations = None;
        let mut instantiation_inclusions = None;
        let mut instantiation_exclusions = None;
        let mut docs = Vec::new();

        for attr in attrs {
            if let syn::AttrStyle::Inner(tok) = attr.style {
               return Err(syn::Error::new(tok.span(), "Inner attributes are not supported"));
            }

            match &attr.meta {
                syn::Meta::List(meta_list) => {
                    let ident = meta_list.path.get_ident()
                        .ok_or_else(||syn::Error::new(attr.path().span(), "Ident of attribute not found"))?;

                    match ident.to_string().as_str() {
                        "instantiate_for" => {
                            instantiations = Some(meta_list.parse_args()?);
                        },
                        "include_if_struct_instantiation" => {
                            instantiation_inclusions = Some(meta_list.parse_args()?);
                        },
                        "exclude_if_struct_instantiation" => {
                            instantiation_exclusions = Some(meta_list.parse_args()?);
                        },
                        "doc" => docs.push(attr.clone()), // Non-comment doc attribute
                        _ => return Err(syn::Error::new(ident.span(), "Unsupported attribute")),
                    }
                    if instantiation_inclusions.is_some() && instantiation_exclusions.is_some() {
                        return Err(syn::Error::new(attr.meta.span(), "Forbidden to have #[include_if_struct_instantiation] and #[exclude_if_struct_instantiation] attributes for the same trait impl"))
                    }
                },
                syn::Meta::NameValue(nv) => {
                    let Some(ident) = nv.path.get_ident() else {
                        return Err(syn::Error::new(nv.span(), "Unexpected path of name-value attribute"))
                    };
                    match ident.to_string().as_str() {
                        "doc" => docs.push(attr.clone()), // Doc comment
                        _ => return Err(syn::Error::new(nv.span(), "Unsupported name-value attribute"))
                    }
                },
                _ => return Err(syn::Error::new(attr.meta.span(), "Unsupported type of attribute")),
            }
        }

        Ok(Self {
            instantiations,
            instantiation_inclusions,
            instantiation_exclusions,
            docs,
        })
    }

    pub fn instantiations(&self) -> Option<&GenericInstantiationsList> {
        self.instantiations.as_ref()
    }

    pub fn instantiation_inclusions(&self) -> Option<&GenericInstantiationTypesList> {
        self.instantiation_inclusions.as_ref()
    }

    pub fn instantiation_exclusions(&self) -> Option<&GenericInstantiationTypesList> {
        self.instantiation_exclusions.as_ref()
    }

    pub fn docs(&self) -> &[syn::Attribute] {
        &self.docs
    }
}
