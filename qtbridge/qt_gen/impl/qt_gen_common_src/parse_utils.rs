// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
use syn::{buffer::Cursor, parse::{Parse, ParseBuffer, StepCursor}};

struct NameEqValue<Name, Value> {
    pub name: Name,
    pub value: Value,
}

impl<Name: Parse, Value: Parse> syn::parse::Parse for NameEqValue<Name, Value> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _eq: syn::Token![=] = input.parse()?;
        let value = input.parse()?;
        Ok(NameEqValue { name, value })
    }
}

pub fn parse_name_value<Name: Parse, Value: Parse>(input: &ParseBuffer) -> syn::Result<(Name, Value)> {
    let begin = input.fork();
    let nv = match input.parse::<NameEqValue<Name, Value>>() {
        Ok(nv) => nv,
        Err(_) => return Err(begin.error("Failed to parse expression like name=value")),
    };

    Ok((nv.name, nv.value))
}

pub fn partition_attr_by(attrs: Vec<syn::Attribute>, pred: fn (&syn::Attribute)->bool) -> (Vec<syn::Attribute>, Option<syn::Attribute>) {

    if let Some(pos) = attrs.iter().position(pred) {
        let mut attrs = attrs;
        let found = attrs.remove(pos);
        return (attrs, Some(found));
    }

    (attrs, None)
}

pub fn is_doc_attribute(attr: &syn::Attribute) -> bool {
    if let syn::AttrStyle::Outer = attr.style &&
       let Some(ident) = attr.path().get_ident() &&
       ident == "doc"
    {
        return true
    }

    false
}

pub fn is_two_segment_path_outer_attribute(attr: &syn::Attribute, seg0_str: &str, seg1_str: &str) -> bool {
    if let syn::AttrStyle::Inner(_) = attr.style  {
        return false;
    }

    let segs = &attr.path().segments;
    let result = segs.get(0).is_some_and(|seg| seg.ident == seg0_str) &&
                 segs.get(1).is_some_and(|seg| seg.ident == seg1_str);
    result
}

pub fn is_cxx_bridge_attribute(attr: &syn::Attribute) -> bool {
    is_two_segment_path_outer_attribute(attr, "cxx", "bridge")
}

pub fn parse_include_path<'c, 'a>(cursor: StepCursor<'c, 'a>) -> syn::Result<(String, Cursor<'c>)> {

    if let Some((literal, next_cursor)) = cursor.literal() {
        let literal_str = literal.to_string();
        if literal_str.len() < 2 || !literal_str.starts_with('\"') || !literal_str.ends_with('\"') {
            return Err(cursor.error("Non empty string literal expected"));
        }
        return Ok((literal_str, next_cursor));
    }

    let (left_delim, mut cursor) = match cursor.punct() {
        Some((delim, next_cursor)) => (delim.as_char(), next_cursor),
        None => return Err(cursor.error("Delimiter expected")),
    };
    if left_delim != '<' {
        return Err(syn::Error::new(cursor.span(), format!("Unsupported delimiter: '{left_delim}'")));
    };
    let right_delim = '>';

    let mut content = String::new();
    loop {

        let (tt, next_cursor) = cursor.token_tree()
            .ok_or_else(|| syn::Error::new(cursor.span(), "Unterminated token stream"))?;

        match tt {
            TokenTree::Punct(punct) => {
                let ch = punct.as_char();
                if ch == right_delim {
                    if !next_cursor.eof() {
                        return Err(syn::Error::new(cursor.span(), "Unexpected tokens after closing delimiter"));
                    }
                    return Ok((format!("<{content}>"), next_cursor));
                }

                match ch {
                    '/' | '\\' => content.push('/'),
                    '.' => content.push('.'),
                    d => return Err(syn::Error::new(cursor.span(), format!("Unexpected #include delimiter: '{}'", d))),
                }
            },
            TokenTree::Ident(ident) => content.push_str(&ident.to_string()),
            _ => return Err(syn::Error::new(tt.span(), "Unsupported token type")),
        }

        cursor = next_cursor;
    }
}

pub fn find_token<F>(src: TokenStream, pred: &F) -> Option<proc_macro2::Ident>
    where F: Fn(&proc_macro2::Ident) -> bool {

    for token in src {
        match token {
            TokenTree::Group(group) =>
                if let Some(result) = find_token(group.stream(), pred) {
                    return Some(result);
                }
            TokenTree::Ident(ident) => if pred(&ident) {
                return Some(ident.clone())
            }
            _ => {},
        }
    }

    None
}

// TODO: maybe this should belong to other file
pub fn replace_idents_in_token_stream<F>(src_tokens: TokenStream, predicate: &F) -> TokenStream
    where F: Fn(&proc_macro2::Ident) -> Option<proc_macro2::Ident> {

    let mut new_stream = TokenStream::new();
    for src_token in src_tokens {
        let new_token = match src_token {
            TokenTree::Group(group) => {
                let new_stream = replace_idents_in_token_stream(group.stream(), predicate);
                TokenTree::Group(
                    proc_macro2::Group::new(group.delimiter(), new_stream)
                )
            }
            TokenTree::Ident(ident) => {
                let new_ident = predicate(&ident)
                    .unwrap_or(ident);
                TokenTree::Ident(new_ident)
            },
            _ => src_token.clone(),
        };
        new_token.to_tokens(&mut new_stream);
    }

    new_stream
}
