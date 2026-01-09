// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qt_gen_common::rust_type_info::RustTypeInfo;
use qt_gen_common::signature_utils::{get_arg_type_info, get_return_type_info, is_arg_self_ref};
use quote::ToTokens;
use syn::spanned::Spanned;

pub(crate) fn deduce_type_from_getter<'a>(getter_ident: &syn::Ident, methods: &'a[syn::Signature]) -> syn::Result<RustTypeInfo<'a>> {
    let getter = methods.iter()
        .find(|g| g.ident == *getter_ident)
        .ok_or_else(|| syn::Error::new(getter_ident.span(), format!("Property getter '{getter_ident}' not found")))?;

    get_property_getter_type(getter)
        .map_err(|err| syn::Error::new(err.span(), format!("Function '{getter_ident}' is not suitable to be property getter.\nReason: {err}")))
}

pub(crate)fn deduce_type_from_setter<'a>(setter_ident: &syn::Ident, methods: &'a[syn::Signature]) -> syn::Result<RustTypeInfo<'a>> {
    let setter = methods.iter()
        .find(|s| s.ident == *setter_ident)
        .ok_or_else(||syn::Error::new(setter_ident.span(), format!("Property setter '{setter_ident}' not found")))?;

    get_property_setter_type(setter)
        .map_err(|err| syn::Error::new(err.span(), format!("Function {setter_ident} is not suitable to be property setter. Reason: {err}")))
}

fn get_property_getter_type(sig: &syn::Signature) -> syn::Result<RustTypeInfo<'_>> {
    let args = &sig.inputs;
    if args.len() != 1 || !is_arg_self_ref(&args[0], Some(false)) {
        return Err(syn::Error::new(sig.span(), "Property getter must have single argument (&self)"));
    }

    let return_type = get_return_type_info(&sig.output)
        .ok_or_else(|| syn::Error::new(sig.span(), format!("Getter has return type not specified : {}", sig.output.to_token_stream())))?;

    if !return_type.is_mapped_to_qmetatype() {
        let type_tok = return_type.get_type().to_token_stream();
        return Err(syn::Error::new(return_type.span(), format!("Return type {type_tok} is not supported for bridging")));
    }

    Ok(return_type)
}

fn get_property_setter_type(sig: &syn::Signature) -> syn::Result<RustTypeInfo<'_>> {
    let args = &sig.inputs;
    if args.len() != 2 {
        let span = match args.len() {
            0 => sig.ident.span(),
            1 => args[0].span(),
            _ => args[2].span(),
        };
        return Err(syn::Error::new(span, "Property setter supposed to have 2 arguments (&self and value)"));
    }

    let arg0 = &args[0];
    if !is_arg_self_ref(arg0, None) {
        return Err(syn::Error::new(arg0.span(), "First argument must be &self"));
    }

    let arg1 = &args[1];
    let arg_type = match get_arg_type_info(arg1) {
        Ok(t) => t,
        Err(err) => return Err(syn::Error::new(err.span(), format!("Failed to get type of argument: {err}"))),
    };

    if !arg_type.is_mapped_to_qmetatype() {
        let type_tok = arg_type.get_type().to_token_stream();
        return Err(syn::Error::new(arg_type.span(), format!("Type {type_tok} is not supported for bridging")));
    }

    Ok(arg_type)
}
