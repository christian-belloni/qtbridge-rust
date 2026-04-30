// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::collections::BTreeMap;

use proc_macro2::{TokenStream, TokenTree};

use quote::{ToTokens, format_ident, quote};
use syn::parse::discouraged::Speculative;
use syn::Token;

use qtbridge_gen_common::case_conv;
use qtbridge_gen_common::cpp_fn_sign::CppFnSign;
use qtbridge_gen_common::format_code::token_stream_to_code;
use qtbridge_gen_common::multi_type_mapping::MultiTypeMapping;
use qtbridge_gen_common::qt_generic_mapping::QtGenericMapping;
use qtbridge_gen_common::signature_utils::{is_unsafe, ExpectSelfRef};
use qtbridge_gen_common::type_mapping_nested::TypeMappingNested;
use qtbridge_gen_common::type_to_cpp::type_to_cpp_allow_unknown;

use crate::self_type_mapping::SelfTypeMapping;

#[derive(Clone)]
pub struct CppFun {
    rust_sign: syn::Signature,
    cpp_name: String,
    cpp_func_code: TokenStream,
}

impl CppFun {
    pub fn new(cpp_name: String, tokens: TokenStream) -> syn::Result<Self> {
        let mut fun: CppFun = syn::parse2(tokens)?;
        fun.set_cpp_name(cpp_name);

        Ok(fun)
    }

    pub fn signature(&self) -> &syn::Signature {
        &self.rust_sign
    }

    pub fn get_code(&self, cpp_name: &str) -> syn::Result<(String, String)> {
        let rust_sign = self.signature();
        let cpp_sign = CppFnSign::new_from_rust_sig(rust_sign, Some(cpp_name.to_owned()), ExpectSelfRef::Maybe)?;
        let mut decl = cpp_sign.to_declaration_string(true);
        let def = format!("{decl}\n{{\n{}\n}}\n", token_stream_to_code(&self.cpp_func_code));
        decl.push(';');

        Ok((decl, def))
    }

    /// Replace generic idents with concrete types
    /// E.g., T -> i32
    pub fn substitute_types(&self, type_map: &TypeMappingNested<MultiTypeMapping>, self_type: &syn::Type) -> syn::Result<Self> {
        let src_sign = &self.rust_sign;
        let mut new_sign = type_map.map_signature(src_sign)?;

        let self_map = TypeMappingNested::new(SelfTypeMapping::new(self_type.clone()));
        new_sign = self_map.map_signature(&new_sign)?;

        // Substitute types in C++ code
        let cpp_type_map = type_map.get_impl().iter()
            .filter_map(|(from, to)| {
                if let Ok(to_cpp) = type_to_cpp_allow_unknown(to) {
                    return Some((from.clone(), to_cpp));
                }
                None
            }).collect::<BTreeMap<_, _>>();

        let cpp_tokens = self.cpp_func_code.clone();
        let new_cpp_code = match cpp_type_map.is_empty() {
            true => cpp_tokens,
            false => Self::substitute_cpp_tokens(cpp_tokens, &cpp_type_map)?,
        };

        Ok(Self {
            rust_sign: new_sign,
            cpp_func_code: new_cpp_code,
            ..self.clone()
        })
    }

    fn substitute_cpp_tokens(src: TokenStream, type_map: &BTreeMap<syn::Ident, String>) -> syn::Result<TokenStream> {
        let mut result = TokenStream::new();
        for src_token in src {
            let new_tokens = match src_token {
                TokenTree::Group(group) => {
                    let new_stream = Self::substitute_cpp_tokens(group.stream(), type_map)?;
                    TokenTree::Group(
                        proc_macro2::Group::new(group.delimiter(), new_stream)
                    ).to_token_stream()
                }
                TokenTree::Ident(ident) => {
                    match type_map.get(&ident) {
                        Some(ty_str) => syn::parse_str(ty_str)?,
                        None => ident.to_token_stream(),
                    }
                }
                _ => src_token.to_token_stream(),
            };
            new_tokens.to_tokens(&mut result);
        }

        Ok(result)
    }

    /// Replace generic QtTypes with argument with the concrete type
    /// E.g., QHash<i32, f64> => QHash_i32_f64
    pub fn substitute_generic_qt_types(&mut self) -> syn::Result<()> {
        QtGenericMapping::map_signature(&mut self.rust_sign)
    }

    fn set_cpp_name(&mut self, name: String) {
        self.rust_sign.ident = format_ident!("{}", case_conv::camel_to_snake(&name));
        self.cpp_name = name;
    }
}

impl syn::parse::Parse for CppFun {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {

        let line1: syn::Token![|] = input.parse()?;

        let receiver = {
            let fork = input.fork();
            match fork.parse::<syn::Receiver>() {
                Ok(r) => {
                    let _delim = fork.parse::<Option<syn::Token![,]>>()?;
                    input.advance_to(&fork);
                    Some(r)
                },
                Err(_) => None,
            }
        };

        let mut typed_args = Vec::new();
        loop {
            if input.peek(syn::Token![|]) {
                break;
            }
            typed_args.push(input.parse::<syn::PatType>()?);
            let _delim = input.parse::<Option<syn::Token![,]>>()?;
        }
        let _line2: syn::Token![|] = input.parse()?;
        let output: syn::ReturnType = input.parse()?;

        let mut fn_args = Vec::<syn::FnArg>::new();
        if let Some(receiver) = receiver {
            fn_args.push(receiver.into());
        };
        fn_args.extend(typed_args.into_iter()
            .map(|arg| arg.into()));

        let mut rust_sign: syn::Signature = syn::parse2(quote!{
            fn tbd(#(#fn_args),*) #output
        })?;

        if is_unsafe(&rust_sign) {
            rust_sign.unsafety = Some(Token![unsafe](line1.span));
        }

        let cpp_func_code = input.step(|cursor| {
            let tt = cursor.group(proc_macro2::Delimiter::Brace)
                .ok_or_else(|| cursor.error("Expected braces with some code (C++) inside"))?;
            Ok((tt.0.token_stream(), tt.2))
        })?;

        Ok(CppFun {
            rust_sign,
            cpp_func_code,
            cpp_name: "tbd".to_owned(),
        })
    }
}
