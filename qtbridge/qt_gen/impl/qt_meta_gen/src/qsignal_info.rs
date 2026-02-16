// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Ident, LitStr};
use syn::spanned::Spanned;

use qt_gen_common::case_conv;
use qt_gen_common::function_with_attributes::{FunctionWithAttributes, BlockOrSemi};
use qt_gen_common::parse_utils::{parse_name_value, partition_attr_by};
use qt_gen_common::signature_utils::{get_typed_args, get_typed_args_types};
use qt_gen_common::type_utils::remove_ref;
use qt_gen_common::type_qualified_mapping::CallOrigin;
use qt_gen_common::type_registry::meta_types::{check_meta_call_signature_types, get_qmetatype_support_for_type};
use crate::meta_call_bridge_generator::MetaCallBridgeGenerator;
use crate::traits::{ExpandTokens, QmlName};

#[derive(Default)]
struct QSignalMetaParams {
    name: Option<syn::LitStr>,
}

pub struct QSignalInfo {
    attrs: Vec<syn::Attribute>,     // Attributes other than qsignal
    meta_params: QSignalMetaParams, // Params extracted from qsignal attribute
    vis: syn::Visibility,
    sig: syn::Signature,
    #[allow(dead_code)]
    origin: CallOrigin,
}

impl QSignalInfo {
    pub fn new(input: FunctionWithAttributes, origin: &CallOrigin) -> syn::Result<Self> {
        Self::check_signature(&input.sig)?;

        let (attrs, signal_attr) = partition_attr_by(input.attrs.clone(), Self::is_for_me);
        let signal = signal_attr
            .ok_or_else(|| syn::Error::new(input.sig.span(), "qsignal attribute was not found for the function"))?;

        if let BlockOrSemi::Block(block) = input.block {
            if !block.stmts.is_empty() {
                return Err(syn::Error::new(block.span(), "qsignal must not contain any code in brackets"));
            }
        }

        let meta_params = get_qsignal_meta_params(&signal)?;

        Ok(QSignalInfo{
            attrs,
            meta_params,
            vis: input.vis,
            sig: input.sig,
            origin: origin.clone(),
        })
    }

    pub fn is_for_me(attr: &syn::Attribute) -> bool {
        attr.style == syn::AttrStyle::Outer && attr.path().is_ident("qsignal")
    }

    pub fn get_rust_name(&self) -> syn::Ident {
        self.sig.ident.clone()
    }

    /// Get count of arguments after &self
    pub fn get_typed_arg_count(&self) -> usize {
        get_typed_args(&self.sig).count()
    }

    pub fn get_arg_type(&self, num: usize) -> syn::Result<&syn::Type> {
        get_typed_args_types(&self.sig)
            .nth(num)
            .ok_or_else(|| syn::Error::new(self.sig.span(), format!("Failed to get typed argument #{num}")))
    }

    pub fn get_meta_registration_code(&self) -> syn::Result<TokenStream> {
        let sig = &self.sig;

        let name = self.get_qml_name_span().0;
        let arg_types_qt = get_typed_args_types(sig)
            .map(|ty| {
                let meta_type = get_qmetatype_support_for_type(ty)?
                    .unwrap_or_else(|| remove_ref(ty).clone());
                Ok(meta_type)
            })
            .collect::<syn::Result<Vec<_>>>()?;

        let register_signal = quote!{
            meta_obj.as_mut().register_signal(#name, &[#(#arg_types_qt::get_qmetatype()),*]);
        };
        Ok(register_signal)
    }

    fn check_signature(sign: &syn::Signature) -> syn::Result<()> {
        check_meta_call_signature_types(&sign)
    }
}

mod qsignal_keywords {
    syn::custom_keyword!(qml_name);
}

fn get_qsignal_meta_params(attr: &syn::Attribute) -> syn::Result<QSignalMetaParams> {
    match &attr.meta {
        syn::Meta::Path(_) => Ok(QSignalMetaParams::default()),
        syn::Meta::List(meta_list) => match syn::parse2::<QSignalMetaParams>(meta_list.tokens.clone()) {
            Ok(params) => Ok(params),
            Err(err) => Err(syn::Error::new(err.span(), format!("Failed to parse qsignal attributes: {}", err))),
        },
        _ => Err(syn::Error::new(attr.span(), "Unexpected format of qsignal attributes"))
    }
}

impl syn::parse::Parse for QSignalMetaParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut name = None;

        while !input.is_empty() {
            if input.peek(qsignal_keywords::qml_name) {
                name = Some(parse_name_value::<Ident, LitStr>(input)?.1);
                // TODO: check charset of name?
            }
            else {
                return Err(input.error("Unsupported qsignal parameter attribute"));
            }
        }

        Ok(Self{
            name,
        })
    }
}

impl QmlName for QSignalInfo {
    fn get_qml_name_span(&self) -> (String, proc_macro2::Span) {
        if let Some(name) = self.meta_params.name.as_ref() {
            (name.value(), name.span())
        }
        else {
            let ident = &self.sig.ident;
            (case_conv::snake_to_camel(&ident.to_string()), ident.span())
        }
    }
}

impl ExpandTokens for QSignalInfo {
    fn expand_tokens(&self) -> syn::Result<TokenStream> {
        let Self {attrs, vis, sig, ..} = self;

        let bridge_generator = MetaCallBridgeGenerator::new(sig)?;
        let qml_name = self.get_qml_name_span().0;
        let fn_metacall = parse_quote! {
            dynamic_meta_obj.emit_signal(qobj, #qml_name)
        };
        let bridge_code = bridge_generator.generate_bridge_user_fn_to_metacall(fn_metacall)?;

        // Generate function that calls signal
        let code = quote! {
            #(#attrs)*
            #vis
            #sig
            {
                let dynamic_meta_obj = <Self as qtbridge::bridge::QMetaInfo>::get_shared_dynamic_meta_object();
                let qobj = <Self as qtbridge::QObjectHolder>::get_qobject(self);
                #bridge_code
            }
        };
        Ok(code)
    }
}
