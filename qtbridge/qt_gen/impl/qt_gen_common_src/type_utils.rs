// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::type_to_string::type_to_string;

#[derive(Copy, Clone, PartialEq)]
pub enum ValuePass {
    ByValue,
    ByConstReference,
    ByMutReference,
    // TODO: ByValueCopy ?
}

pub fn remove_ref_to_string(ty: &syn::Type) -> syn::Result<String> {
    type_to_string(remove_ref(ty))
}

/// Recursively unwraps the type until non-reference type is found
pub fn remove_ref(ty: &syn::Type) -> &syn::Type {
    let mut unwrapped = ty;
    loop {
        match unwrapped {
            syn::Type::Reference(type_ref) =>
                unwrapped = &type_ref.elem.as_ref(),
            _ => break
        }
    }
    unwrapped
}

pub fn get_type_pass(ty: &syn::Type) -> ValuePass {
    match ty {
        syn::Type::Reference(reference) => {
            match &reference.mutability {
                Some(_) => ValuePass::ByMutReference,
                None => ValuePass::ByConstReference,
            }
        },
        _ => ValuePass::ByValue,
    }
}

pub fn get_take_value_code(value: &syn::Ident, pass: ValuePass) -> TokenStream {
    match pass {
        ValuePass::ByValue => quote!{ #value },
        ValuePass::ByConstReference => quote!{ &#value },
        ValuePass::ByMutReference => quote!{ &mut #value },
    }
}

pub fn is_ptr(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Ptr(_))
}

pub fn is_mut_ref(ty: &syn::Type) -> bool {
    if let syn::Type::Reference(ref_) = ty {
        return ref_.mutability.is_some()
    }
    false
}

pub fn ident_str_to_path(src: &str) -> syn::Path {
    let ident = format_ident!("{src}");
    ident_to_path(ident)
}

pub fn ident_to_path(src: syn::Ident) -> syn::Path {
    src.into()
}

pub fn path_to_type(src: syn::Path) -> syn::Type {
    let type_path = syn::TypePath {
        qself: None,
        path: src,
    };
    type_path.into()
}


// TODO: switch to using where applicable
pub fn get_ident_of_last_path_segment(src: &syn::Path) -> Option<&syn::Ident> {
    let last_seg = src.segments.last()?;
    Some(&last_seg.ident)
}

pub fn get_angle_bracketed_generic_arguments_of_last_path_segment(src: &syn::Path) -> Option<&syn::AngleBracketedGenericArguments> {
    let last_seg = src.segments.last()?;
    let syn::PathArguments::AngleBracketed(ab) = &last_seg.arguments else {
        return None
    };

    Some(ab)
}

/// Return true if 'path' has the same component idents as specified in 'qualified_path'.
/// 'path' is allowed to have some first components missing.
pub fn is_same_path<I, S>(path: &syn::Path, qualified_path_comps: I) -> bool
where
    I: DoubleEndedIterator<Item = S>,
    S: AsRef<str>
{
    path.segments.iter()
        .rev()
        .zip(qualified_path_comps.rev())
        .all(|(lhs, rhs)| lhs.ident == rhs )
}

/// Return true if path has arguments in angle brackets
/// and all of those arguments contained in 'idents' slice.
pub fn are_all_args_generic_idents(src: &syn::Path, idents: &[syn::Ident]) -> bool {
    let Some(ab) = get_angle_bracketed_generic_arguments_of_last_path_segment(src) else {
        return false
    };

    ab.args.iter()
        .all(|gen_arg| {
            if let syn::GenericArgument::Type(ty) = gen_arg &&
               let syn::Type::Path(ty_path) = ty &&
               ty_path.qself.is_none() &&
               let Some(ident) = ty_path.path.get_ident() &&
               idents.contains(ident)
            {
                    return true
            }
            false
        })
}
