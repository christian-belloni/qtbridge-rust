// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::Parse;
use syn::Token;

use qtbridge_gen_common::multi_type_mapping::MultiTypeMapping;
use qtbridge_gen_common::type_mapping::TypeMapping;
use qtbridge_gen_common::type_mapping_nested::TypeMappingNested;

use crate::generic_idents::GenericIdents;
use crate::generic_instantiation_decl::GenericInstantiationsList;
use crate::qmetatype_attribute::QMetaTypeAttribute;
use crate::structure_attributes::StructureAttributes;

#[derive(Clone)]
pub struct StructureField {
    ident: syn::Ident,
    ty: syn::Path,
}

#[derive(Clone)]
pub struct BridgeStruct {
    attrs: Option<StructureAttributes>,
    ident: syn::Ident,
    generics: GenericIdents,
    fields: Vec<StructureField>,
}

impl BridgeStruct {
    pub fn is_for_me(input: syn::parse::ParseStream) -> bool {
        input.peek(Token![struct])
    }

    pub fn ident(&self) -> &syn::Ident {
        &self.ident
    }

    pub fn get_path_instantiated(&self, type_map: &MultiTypeMapping) -> syn::Result<syn::Path> {
        let ident = self.ident();
        let generics = self.generics();

        let path: syn::Path = if generics.is_empty() {
            ident.clone().into()
        } else {
            let args = generics.list().iter()
                .map(|gen_ident| type_map.map(gen_ident)
                    .ok_or_else(|| syn::Error::new(gen_ident.span(), format!("Failed to map generic parameter '{gen_ident}'"))))
                .collect::<syn::Result<Vec<_>>>()?;

            syn::parse2(quote!{
                #ident<#(#args),*>
            })?
        };

        Ok(path)
    }

    pub fn is_generic(&self) -> bool {
        !self.generics().is_empty()
    }

    pub fn generics(&self) -> &GenericIdents {
        &self.generics
    }

    pub fn fields(&self) -> &Vec<StructureField> {
        &self.fields
    }

    pub fn set_attributes(&mut self, attributes: &[syn::Attribute]) -> syn::Result<()> {
        let attrs = StructureAttributes::new(attributes)?;
        // Check whether the input attribute data is consistent with the struct declaration.
        match attrs.instantiations() {
            Some(insts) => insts.check_size(self.generics().len())?,
            None => if let Some(first_gen) = self.generics().list().first() {
                return Err(syn::Error::new(first_gen.span(), "Instantiations must be specified for generics. Use 'instantiate_for' attribute of structure"));
            }
        }

        self.attrs = Some(attrs);
        Ok(())
    }

    pub fn namespace(&self) -> Option<&String> {
        self.attrs.as_ref()
            .and_then(StructureAttributes::namespace)
    }

    pub fn qmetatype_attr(&self) -> Option<&QMetaTypeAttribute> {
        self.attrs.as_ref()
            .and_then(StructureAttributes::qmetatype)
    }

    pub(crate) fn instantiations_declaration(&self) -> Option<&GenericInstantiationsList> {
        self.attrs.as_ref()
            .and_then(StructureAttributes::instantiations)
    }

    pub fn docs(&self) -> &[syn::Attribute] {
        self.attrs.as_ref()
            .map(|attr| attr.docs())
            .unwrap_or_default()
    }

    pub fn derived_traits(&self) -> &[String] {
        self.attrs.as_ref()
            .map(|attrs| attrs.derive_traits())
            .unwrap_or_default()
    }

    pub fn is_trait_cpp_derived(&self, name: &str) -> bool {
        self.attrs.as_ref()
            .is_some_and(|attrs| attrs.derive_cpp_traits().contains(&name.to_string()))
    }

    pub fn get_fields_instantiated(&self, type_map: &TypeMappingNested<MultiTypeMapping>) -> syn::Result<Vec<StructureField>> {

        // Substitute types of generic fields with definite ones
        // Clone other fields
        self.fields.iter()
            .map(|field| {
                Ok(StructureField {
                    ident: field.ident.clone(),
                    ty: type_map.map_path(&field.ty)?
                })
            })
            .collect()
    }
}

impl Parse for BridgeStruct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _vis = input.parse::<syn::Visibility>()?;
        let _struct: Token![struct] = input.parse()?;
        let ident: syn::Ident = input.parse()?;
        let generics = if input.peek(Token![<]) {
            input.parse()?
        }
        else {
            GenericIdents::default()
        };

        let mut fields = Vec::new();
        if input.peek(syn::token::Brace) {
            let content;
            let _braces = syn::braced!(content in input);

            while !content.is_empty() {
                let field_ident = content.parse()?;
                let _colon: Token![:] = content.parse()?;
                let ty = content.parse()?;
                fields.push(StructureField {
                    ident: field_ident,
                    ty,
                });
                if content.peek(Token![,]) {
                    let _comma: Token![,] = content.parse()?;
                }
                else if !content.is_empty() {
                    return Err(content.error("Unexpected format of struct bridge declaration"));
                }
            }

            if fields.is_empty() {
                return Err(syn::Error::new(content.span(), "Expected some fields. If structure type is opaque, put ';' after structure name"));
            }
        }
        else if input.peek(Token![;]) {
            let _semi: Token![;] = input.parse()?;
        }
        else {
            return Err(input.error("Expected '{' or ';'"));
        }

        let instance = Self {
            attrs : None,
            ident,
            generics,
            fields,
        };
        Ok(instance)
    }
}


impl StructureField {
    pub fn get_type(&self) -> &syn::Path{
        &self.ty
    }
}

impl ToTokens for StructureField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self{ ident, ty } = self;
        quote!{
            #ident: #ty
        }.to_tokens(tokens);
    }
}
