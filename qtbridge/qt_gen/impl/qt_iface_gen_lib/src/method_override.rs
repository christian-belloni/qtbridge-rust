// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::TokenStream;
use syn::{spanned::Spanned, Ident, LitStr};

use qt_gen_common::case_conv;
use qt_gen_common::function_with_attributes::{BlockOrSemi, FunctionWithAttributes};
use qt_gen_common::parse_utils::{parse_name_value, partition_attr_by};
use qt_gen_common::signature_utils::check_method_signature;
use quote::ToTokens;

#[derive(Default)]
struct MethodOverrideMetaParams {
    cpp_name: Option<syn::LitStr>,         // Name of method declared on cpp side
}

// Override of method (used for both virtual and non-virtual function)
pub struct MethodOverride {
    meta_params: MethodOverrideMetaParams, // Meta params (cpp_name)
    pub func: syn::ImplItemFn,             // Function itself
}

impl MethodOverride {
    pub fn new(input: FunctionWithAttributes) -> syn::Result<Self> {
        Self::check_signature(&input.sig)?;

        let (attrs, overridden_attr) = partition_attr_by(input.attrs.clone(), Self::is_for_me);
        let overridden_attr = overridden_attr
            .ok_or_else(|| syn::Error::new(input.sig.span(), "'override' attribute was not found for the function"))?;

        let BlockOrSemi::Block(block) = input.block else {
            return Err(syn::Error::new(input.sig.span(), "Overridden function must contain code"));
        };

        let meta_params = get_method_override_meta_params(&overridden_attr)?;

        let func = syn::ImplItemFn {
            attrs,
            vis: input.vis,
            defaultness: None,
            sig: input.sig,
            block,
        };

        Ok(MethodOverride {
            meta_params,
            func
        })
    }

    // TODO: rename to is_mine()?
    pub fn is_for_me(attr: &syn::Attribute) -> bool {
        attr.style == syn::AttrStyle::Outer && attr.path().is_ident("overridden")
    }

    pub fn get_cpp_name(&self) -> String {
        self.meta_params.cpp_name.as_ref().map_or_else(
            ||case_conv::snake_to_camel(&self.func.sig.ident.to_string()),
            |lit_str| lit_str.value())
    }

    pub fn get_cpp_name_span(&self) -> proc_macro2::Span {
        self.meta_params.cpp_name.as_ref().map_or(self.func.sig.ident.span(), |name| name.span())
    }

    pub fn get_signature(&self) -> &syn::Signature {
        &self.func.sig
    }

    pub fn expand_tokens(&self) -> syn::Result<TokenStream> {
        Ok(self.func.to_token_stream())
    }

    fn check_signature(sign: &syn::Signature) -> syn::Result<()> {
        check_method_signature(sign)
    }

}

fn get_method_override_meta_params(attr: &syn::Attribute) -> syn::Result<MethodOverrideMetaParams> {
    match &attr.meta {
        syn::Meta::Path(_) => Ok(MethodOverrideMetaParams::default()),
        syn::Meta::List(meta_list) => {
            match syn::parse2::<MethodOverrideMetaParams>(meta_list.tokens.clone()) {
                Ok(params) => Ok(params),
                Err(err) => Err(syn::Error::new(err.span(),format!("Failed to parse 'override' attribute parameters: {}", err))),
            }
        },
        _ => Err(syn::Error::new(attr.span(), "Unexpected format of 'override' attributes"))
    }
}

mod override_keywords {
    syn::custom_keyword!(cpp_name);
}

impl syn::parse::Parse for MethodOverrideMetaParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut cpp_name = None;

        while !input.is_empty() {
            if input.peek(override_keywords::cpp_name) {
                cpp_name = Some(parse_name_value::<Ident, LitStr>(input)?.1);
                // TODO: check charset of name?
            }
            else {
                return Err(input.error("Unsupported attribute of 'override' annotation"));
            }
        }

        Ok(Self{
            cpp_name,
        })
    }
}

