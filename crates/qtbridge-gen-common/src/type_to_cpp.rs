// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use quote::ToTokens;
use syn::spanned::Spanned;

use crate::type_registry;
use type_registry::TypeCategory;
use type_registry::type_traits::TypesEnum;

pub fn is_type_mapped_to_cpp(ty: &syn::Type) -> bool {
    type_to_cpp(ty).is_ok()
}

pub fn type_to_cpp(src: &syn::Type) -> syn::Result<String> {
    match src {
        syn::Type::BareFn(bare_fn) =>
            type_bare_fn_to_cpp(bare_fn),
        syn::Type::Path(type_path) =>
            type_path_to_cpp(type_path),
        syn::Type::Ptr(type_ptr) =>
            type_ptr_to_cpp(type_ptr),
        syn::Type::Reference(type_ref) =>
            type_ref_to_cpp(type_ref),
        syn::Type::Slice(type_slice) =>
            type_slice_to_cpp(type_slice),
        _ =>
            Err(syn::Error::new(src.span(), format!("Unsupported type category {:?} of type {}", std::mem::discriminant(src), src.to_token_stream()))),
    }
}

pub fn type_bare_fn_to_cpp(src: &syn::TypeBareFn) -> syn::Result<String> {
    if let Some(lt) = &src.lifetimes {
        return Err(syn::Error::new(lt.span(), "Bare function with explicit lifetimes are unsupported"))
    }
    if let Some(abi) = &src.abi {
        return Err(syn::Error::new(abi.span(), "Bare function with ABI are unsupported"))
    }
    if let Some(variadic) = &src.variadic {
        return Err(syn::Error::new(variadic.span(), "Bare function with variadic arguments are not supported"))
    }

    let return_type = match &src.output {
        syn::ReturnType::Default => "void".into(),
        syn::ReturnType::Type(_, ty) => type_to_cpp(ty.as_ref())?,
    };

    let mut all_args_str = String::new();
    for arg in &src.inputs {
        let mut arg_str = type_to_cpp(&arg.ty)?;
        if let Some((arg_name, _)) = arg.name.as_ref() {
            arg_str.push_str(&arg_name.to_string());
        }

        if !all_args_str.is_empty() {
            all_args_str.push_str(", ");
        }
        all_args_str.push_str(&arg_str);
    }

    Ok(format!("rust::Fn<{return_type}({all_args_str})>"))
}

pub fn type_path_to_cpp(src: &syn::TypePath) -> syn::Result<String> {
    if let Some(qself) = &src.qself {
        return Err(syn::Error::new(qself.span(), "QSelf is not supported in type conversion"));
    }

    path_to_cpp(&src.path)
}

fn type_ptr_to_cpp(src: &syn::TypePtr) -> syn::Result<String> {
    // TODO: Check if this is correct in cases of pointer to pointer or pointer to reference, etc

    let ty = type_to_cpp(src.elem.as_ref())?;
    let maybe_const = if src.const_token.is_some() { " const" } else { "" };
    Ok(format!("{ty}{maybe_const}*"))
}

fn type_ref_to_cpp(src: &syn::TypeReference) -> syn::Result<String> {
    // TODO: Check if this is correct in cases of reference to reference or reference to pointer, etc

    // Cases need to be handled in special way
    match src.elem.as_ref() {
        syn::Type::Slice(type_slice) => {
            let maybe_const = src.mutability
                .map_or("const ", |_| "");
            let ty = type_to_cpp(type_slice.elem.as_ref())?;
            return Ok(format!("rust::Slice<{maybe_const} {ty}>"))
        },
        syn::Type::Path(type_path) => {
            if type_path.path.is_ident("str") {
                return Ok("rust::Str".to_string())
            }
        },
        _ => {}
    }

    let maybe_const = if src.mutability.is_none() { "const " } else { "" };
    Ok(format!("{maybe_const}{}&", type_to_cpp(src.elem.as_ref())?))
}

fn type_slice_to_cpp(src: &syn::TypeSlice) -> syn::Result<String> {
    // TODO: &mut [T] ?
    let ty = type_to_cpp(src.elem.as_ref())?;
    Ok(format!("rust::Slice<const {ty}>"))
}

pub fn path_to_cpp(src: &syn::Path) -> syn::Result<String> {
    let segments = &src.segments;
    let seg0 = segments.first()
        .ok_or(syn::Error::new(src.span(), "Empty type path"))?;

    match segments.len() {
        1 => {
            match &seg0.arguments {
                syn::PathArguments::None => {
                    return type_ident_to_cpp(&seg0.ident, None)
                },
                syn::PathArguments::AngleBracketed(_ab) => {
                    return path_segment_angle_bracketed_to_cpp(seg0, None)
                },
                _ => {},
            }
        },
        2 => {
            if let syn::PathArguments::None = seg0.arguments {
                let seg1 = &segments[1];
                let category = match seg0.ident.to_string().as_str() {
                    "std" => TypeCategory::Standard,
                    "cxx" => TypeCategory::Cxx,
                    "qtbridge_type_lib" => TypeCategory::Qt,
                    _ => return Err(syn::Error::new(segments.span(), format!("Unsupported module in multi-segment type path: {}", seg0.ident))),
                };
                let result = match &seg1.arguments {
                    syn::PathArguments::None =>
                        type_ident_to_cpp(&seg1.ident, Some(&category)),
                    syn::PathArguments::AngleBracketed(_ab) =>
                        path_segment_angle_bracketed_to_cpp(seg1, Some(&category)),
                    syn::PathArguments::Parenthesized(p) =>
                        Err(syn::Error::new(seg1.arguments.span(), format!("Parenthesized arguments are unsupported: '{}'", p.to_token_stream()))),
                };
                return result
            }
        },
        _ => {},
    }

    Err(syn::Error::new(src.span(), format!("Conversion of unsupported type to C++: {}", segments.to_token_stream())))
}

pub fn type_ident_to_cpp(src: &syn::Ident, category: Option<&TypeCategory>) -> syn::Result<String> {
    let ty = type_registry::Type::find_by_ident_in_opt_category_result(src, category)?;
    let ty_info = ty.dyn_type_info();
    let cpp_name = ty_info.cpp_name()
        .ok_or_else(|| syn::Error::new(src.span(), "Type is not convertible to C++"))?;
    let ns = ty_info.cpp_namespace()
        .unwrap_or_default();
    let result = if ns.is_empty() { cpp_name.to_owned() } else { format!("::{ns}::{cpp_name}") };
    Ok(result)
}

fn path_segment_angle_bracketed_to_cpp(src: &syn::PathSegment, category: Option<&TypeCategory>) -> syn::Result<String> {
    let syn::PathArguments::AngleBracketed(ab) = &src.arguments else {
        return Err(syn::Error::new(src.span(), "Expected angle bracketed type"))
    };

    let ty = type_registry::Type::find_by_ident_in_opt_category_result(&src.ident, category)?;
    let ty_info = ty.dyn_type_info();

    let gen_params_count = ty_info.generic_arg_count();
    if gen_params_count != ab.args.len() {
        return Err(syn::Error::new(ab.span(), format!("Mismatch in count of generic parameters: {} vs {gen_params_count}", ab.args.len())));
    }

    let cpp_ident = ty_info.cpp_name_qualified()
        .ok_or_else(|| syn::Error::new(src.span(), "Generic type is not convertible to C++"))?;

    let mut args = Vec::new();
    ab.args.iter()
        .try_for_each(|src_arg| {
            let arg = match src_arg {
                syn::GenericArgument::Type(ty) => type_to_cpp(ty)?,
                _ => return Err(syn::Error::new(src_arg.span(), format!("Unsupported type of generic argument: {}", src_arg.to_token_stream())))
            };
            args.push(arg);
            Ok(())
        })?;

    Ok(format!("{cpp_ident}<{}>", args.join(",")))
}
