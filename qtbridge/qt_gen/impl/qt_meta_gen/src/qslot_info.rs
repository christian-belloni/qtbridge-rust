// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{spanned::Spanned, Ident, LitStr};
use qt_gen_common::case_conv;
use qt_gen_common::function_with_attributes::{BlockOrSemi, FunctionWithAttributes};
use qt_gen_common::parse_utils::{parse_name_value, partition_attr_by};
use qt_gen_common::signature_utils::{check_meta_call_signature, get_typed_args, get_typed_args_types};
use qt_gen_common::type_utils::{get_take_value_code, get_type_pass, unwrapped_ref_to_string};
use qt_gen_common::type_registry::meta_types::get_qmetatype_support_for_type;

use crate::traits::{ExpandTokens, QmlName};

#[derive(Default)]
struct QSlotMetaParams {
    name: Option<syn::LitStr>,
}

pub struct QSlotInfo {
    meta_params: QSlotMetaParams, // Params extracted from qslot attribute
    func: syn::ImplItemFn,        // Slot function
}

impl QSlotInfo {
    pub fn new(input: FunctionWithAttributes) -> syn::Result<Self> {
        Self::check_signature(&input.sig)?;

        let (attrs, slot_attr) = partition_attr_by(input.attrs.clone(), Self::is_for_me);
        let slot_attr = slot_attr
            .ok_or_else(|| syn::Error::new(input.sig.span(), "qslot attribute was not found for the function"))?;

        let BlockOrSemi::Block(block) = input.block else {
            return Err(syn::Error::new(input.sig.span(), "qslot must contain brackets (with some code optionally)"));
        };

        let meta_params = get_qslot_meta_params(&slot_attr)?;

        let func = syn::ImplItemFn {
            attrs,
            vis: input.vis,
            defaultness: None,
            sig: input.sig,
            block,
        };

        Ok(QSlotInfo {
            meta_params,
            func,
        })
    }

    pub fn is_for_me(attr: &syn::Attribute) -> bool {
        attr.style == syn::AttrStyle::Outer && attr.path().is_ident("qslot")
    }

    /// Get count of arguments after &self
    pub fn get_typed_arg_count(&self) -> usize {
        get_typed_args(&self.func.sig).count()
    }

    pub fn get_meta_registration_code(&self, struct_ident: &syn::Ident) -> syn::Result<TokenStream> {
        let name = self.get_qml_name_span().0;
        let sig = &self.func.sig;
        let method_ident = &sig.ident;
        let arg_count = sig.inputs.len() - 1;

        let mut arg_types_qt = Vec::with_capacity(arg_count);
        let mut arg_unpack   = Vec::with_capacity(arg_count);
        let mut arg_list     = Vec::with_capacity(arg_count);

        for (idx, arg_type) in get_typed_args_types(sig).enumerate() {
            let meta_type = get_qmetatype_support_for_type(arg_type)?
                .unwrap_or_else(|| arg_type.clone());
            let var_name = format_ident!("arg_{}", idx);
            let pass = get_type_pass(arg_type);
            let pass_var = get_take_value_code(&var_name, pass);
            let getter = get_arg_getter_func(&unwrapped_ref_to_string(arg_type)?);

            arg_types_qt.push(meta_type);
            arg_unpack.push(quote!{ let #var_name = params.#getter( #idx ); });
            arg_list.push(pass_var);
        }

        let register_slot = quote!(
            meta_obj.as_mut().register_slot(#name, &[#(#arg_types_qt::get_qmetatype()),*],
                slot_callback_for::<#struct_ident>(|this, params| {
                    #(#arg_unpack)*
                    this.#method_ident(#(#arg_list),*);
                }));
        );
        Ok(register_slot)
    }

    fn check_signature(sign: &syn::Signature) -> syn::Result<()> {
        check_meta_call_signature(&sign)
    }
}

fn get_qslot_meta_params(attr: &syn::Attribute) -> syn::Result<QSlotMetaParams> {
    match &attr.meta {
        syn::Meta::Path(_) => Ok(QSlotMetaParams::default()),
        syn::Meta::List(meta_list) => match syn::parse2::<QSlotMetaParams>(meta_list.tokens.clone()) {
            Ok(params) => Ok(params),
            Err(err) => Err(syn::Error::new(err.span(),format!("Failed to parse qslot attribute parameters: {}", err))),
        },
        _ => Err(syn::Error::new(attr.meta.span(), "Unexpected format of qslot attributes"))
    }
}

fn get_arg_getter_func(mut arg_type: &str) -> syn::Ident {
    arg_type = match arg_type {
        "str" | "String" => "string",
        "Vec<String>" => "string_list",
        _ => arg_type,
    };

    format_ident!("get_{}", arg_type)
}

mod qslot_keywords {
    syn::custom_keyword!(qml_name);
}

impl syn::parse::Parse for QSlotMetaParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut name = None;

        while !input.is_empty() {
            if input.peek(qslot_keywords::qml_name) {
                name = Some(parse_name_value::<Ident, LitStr>(input)?.1);
                // TODO: check charset of name?
            }
            else {
                return Err(input.error("Unsupported qslot parameter attribute"));
            }
        }

        Ok(Self{
            name,
        })
    }
}

impl QmlName for QSlotInfo {
    fn get_qml_name_span(&self) -> (String, proc_macro2::Span) {
        if let Some(name) = self.meta_params.name.as_ref() {
            (name.value(), name.span())
        }
        else {
            let ident = &self.func.sig.ident;
            (case_conv::snake_to_camel(&ident.to_string()), ident.span())
        }
    }
}

impl ExpandTokens for QSlotInfo {
    fn expand_tokens(&self) -> syn::Result<TokenStream> {
        Ok(self.func.to_token_stream())
    }
}
