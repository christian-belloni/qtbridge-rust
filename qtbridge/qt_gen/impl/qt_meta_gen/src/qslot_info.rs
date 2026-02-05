// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{spanned::Spanned, Ident, LitStr};
use qt_gen_common::case_conv;
use qt_gen_common::function_with_attributes::{BlockOrSemi, FunctionWithAttributes};
use qt_gen_common::parse_utils::{parse_name_value, partition_attr_by};
use qt_gen_common::signature_utils::{get_typed_arg_ident, get_typed_args};
use qt_gen_common::type_utils::{ValuePass, get_type_pass, remove_ref};
use qt_gen_common::type_registry::meta_types::{check_meta_call_signature_types, get_qmetatype_support_for_type};

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

    /// Generate code for registration of the given slot in `DynamicMetaObjectBuilder`.
    /// The output from the function is code like below:
    /// ```ignore
    /// meta_obj.as_mut().register_slot(
    ///       "doSomething",
    ///       &[i32::get_qmetatype(), qt_type_lib::QString::get_qmetatype()],
    ///       slot_callback_for::<Backend>(|this, args| {
    ///           let int_arg_ref = unsafe {
    ///               args[0].cast::<i32>().as_ref()
    ///           }.expect("Argument #1 is nullptr");
    ///           let str_arg_ref = unsafe {
    ///               args[1].cast::<qt_type_lib::QString>().as_ref()
    ///           }.expect("Argument #2 is nullptr");
    ///           let int_arg_var: i32 = int_arg_ref.clone();
    ///           let str_arg_var: <String as ToOwned>::Owned = str_arg_ref.into();
    ///           this.do_something(int_arg_var, &str_arg_var);
    ///       }),
    ///  );
    /// ```
    pub fn get_meta_registration_code(&self, struct_ident: &syn::Ident) -> syn::Result<TokenStream> {
        let name = self.get_qml_name_span().0;
        let sig = &self.func.sig;
        let method_ident = &sig.ident;
        let arg_count = sig.inputs.len() - 1;

        // Meta types for the arguments.
        let mut arg_meta_types = Vec::with_capacity(arg_count);

        // Definitions of references to source parameters cast from input raw pointers.
        let mut arg_src_refs = Vec::with_capacity(arg_count);

        // Intermediate variables converted from input meta types to be passed to the client callback.
        let mut arg_inter_vars = Vec::new();

        // Expressions used as arguments to the client callback.
        let mut arg_pass_list = Vec::with_capacity(arg_count);

        for (idx, arg) in get_typed_args(sig).enumerate() {
            // Get the type of the current argument in the target function.
            let arg_type = arg.ty.as_ref();
            // Remove reference from the argument type, if present.
            let arg_type_wo_ref = remove_ref(arg_type);
            // Determine what metatype corresponds to the argument type.
            let intermediate_meta_type = get_qmetatype_support_for_type(arg_type)?;
            let arg_meta_type = intermediate_meta_type.as_ref()
                .unwrap_or(arg_type_wo_ref);

            // Define a typed reference for the given input parameter.
            let arg_ident = get_typed_arg_ident(arg)?;
            let arg_ref_ident = format_ident!("{arg_ident}_ref");
            let null_ptr_err_msg = format!("Argument #{} is nullptr", idx + 1);
            let arg_src_ref = quote! {
                let #arg_ref_ident = unsafe {
                    args[#idx].cast::<#arg_meta_type>().as_ref()
                }.expect(#null_ptr_err_msg);
            };

            // Determine how the argument must be passed (by value or reference).
            let arg_pass = get_type_pass(arg_type);

            // Define a intermediate variable if type conversion is needed
            // or if argument passed by value.
            let arg_var_ident = format_ident!("{arg_ident}_var");
            if intermediate_meta_type.is_some() {
                arg_inter_vars.push(quote! {
                    let #arg_var_ident: <#arg_type_wo_ref as ToOwned>::Owned = #arg_ref_ident.into();
                });
            }
            else if matches!(arg_pass, ValuePass::ByValue) {
                arg_inter_vars.push(quote! {
                    let #arg_var_ident: #arg_type_wo_ref = #arg_ref_ident.clone();
                });
            }

            // Produce the code passing the argument value to the target function.
            let arg_pass_code = match arg_pass {
                ValuePass::ByValue => // Pass the intermediate variable by value
                    quote!{ #arg_var_ident },
                ValuePass::ByConstReference => {
                    match intermediate_meta_type.as_ref() {
                        Some(_) => // Pass the intermediate variable by reference.
                            quote! { &#arg_var_ident },
                        None => // Pass input argument reference as is.
                            quote! { #arg_ref_ident },
                    }
                },
                ValuePass::ByMutReference =>
                    return Err(syn::Error::new(arg_type.span(),
                        "Arguments passed by mutable references are not supported"))
            };

            arg_meta_types.push(arg_meta_type.clone());
            arg_src_refs.push(arg_src_ref);
            arg_pass_list.push(arg_pass_code);
        }

        let register_slot = quote!(
            meta_obj.as_mut().register_slot(#name, &[#(#arg_meta_types::get_qmetatype()),*],
                slot_callback_for::<#struct_ident>(|this, args| {
                    #(#arg_src_refs)*
                    #(#arg_inter_vars)*
                    this.#method_ident(#(#arg_pass_list),*);
                }));
        );
        Ok(register_slot)
    }

    fn check_signature(sign: &syn::Signature) -> syn::Result<()> {
        check_meta_call_signature_types(&sign)
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
