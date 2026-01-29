// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::Span;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

use crate::generic_instantiation_decl::GenericInstantiationsList;
use crate::qmetatype_attribute::QMetaTypeAttribute;

#[derive(Clone)]
pub struct StructureAttributes {
    derive_traits: Vec<String>,
    derive_cpp_traits: Vec<String>,
    instantiations: Option<GenericInstantiationsList>,
    namespace: Option<String>,
    qmetatype: Option<QMetaTypeAttribute>,
    docs: Vec<syn::Attribute>,
}

impl StructureAttributes {
    pub fn new(attrs: &[syn::Attribute]) -> syn::Result<Self> {
        let mut derive_traits = Vec::new();
        let mut derive_cpp_traits = Vec::new();
        let mut instantiations = None;
        let mut namespace = None;
        let mut qmetatype = None;
        let mut docs = Vec::new();

        for attr in attrs {
            if let syn::AttrStyle::Inner(tok) = attr.style {
               return Err(syn::Error::new(tok.span(), "Inner attributes are not supported"));
            }

            match &attr.meta {
                syn::Meta::Path(path) => {
                    if path.is_ident("qmetatype") {
                        // QMetaType without Id specified
                        qmetatype = Some(QMetaTypeAttribute::new_without_id())
                    }
                    else {
                        return Err(syn::Error::new(path.span(), "Unsupported meta path attribute"))
                    }
                }
                syn::Meta::List(meta_list) => {
                    let ident = meta_list.path.get_ident()
                        .ok_or_else(||syn::Error::new(attr.path().span(), "Ident of attribute not found"))?;

                    match ident.to_string().as_str() {
                        "derive" => {
                            let list = meta_list.parse_args_with(|input: &syn::parse::ParseBuffer|
                                Punctuated::<syn::Ident, syn::Token![,]>::parse_separated_nonempty(&input)
                            )?;
                            derive_traits.extend(list.iter().map(|i| i.to_string()));
                        },
                        "derive_cpp" => {
                            let list = meta_list.parse_args_with(|input: &syn::parse::ParseBuffer|
                                Punctuated::<syn::Ident, syn::Token![,]>::parse_separated_nonempty(&input)
                            )?;
                            derive_cpp_traits.extend(list.iter().map(|i| i.to_string()));
                        },
                        "instantiate_for" => {
                            instantiations = Some(meta_list.parse_args()?);
                        },
                        "doc" => docs.push(attr.clone()), // Non-comment doc attribute
                        _ => return Err(syn::Error::new(ident.span(), "Unsupported meta list attribute")),
                    }
                }
                syn::Meta::NameValue(nv) => {
                    let Some(ident) = nv.path.get_ident() else {
                        return Err(syn::Error::new(nv.span(), "Unexpected path of name-value attribute"))
                    };
                    match ident.to_string().as_str() {
                        "namespace" => {
                            let syn::Expr::Lit(lit) = &nv.value else {
                                return Err(syn::Error::new(nv.span(), "Wrong format of 'namespace' attribute"))
                            };
                            let syn::Lit::Str(lit_str) = &lit.lit else {
                                return Err(syn::Error::new(lit.span(), "Namespace value should be given in quotes"))
                            };
                            namespace = Some(lit_str.value());
                        },
                        "doc" => docs.push(attr.clone()), // Doc comment
                        "qmetatype" => qmetatype = Some(syn::parse2(nv.to_token_stream())?),
                        _ => return Err(syn::Error::new(nv.span(), format!("Unsupported name-value attribute '{ident}'")))
                    }
                }
            }
        }

        if let Some(qmetatype) = qmetatype.as_ref() {
            if qmetatype.id().is_some() && instantiations.is_some() {
                let span = qmetatype.id_span()
                    .unwrap_or_else(|| Span::call_site());
                return Err(syn::Error::new(span, "Can't specify QMetaType id for generic struct"))
            }
        }

        Ok(Self {
            derive_traits,
            derive_cpp_traits,
            instantiations,
            namespace,
            qmetatype,
            docs,
        })
    }

    pub fn derive_traits(&self) -> &[String] {
        &self.derive_traits
    }

    pub fn derive_cpp_traits(&self) -> &[String] {
        &self.derive_cpp_traits
    }

    pub fn instantiations(&self) -> Option<&GenericInstantiationsList> {
        self.instantiations.as_ref()
    }

    pub fn namespace(&self) -> Option<&String> {
        self.namespace.as_ref()
    }

    pub fn qmetatype(&self) -> Option<&QMetaTypeAttribute> {
        self.qmetatype.as_ref()
    }

    pub fn docs(&self) -> &[syn::Attribute] {
        &self.docs
    }
}
