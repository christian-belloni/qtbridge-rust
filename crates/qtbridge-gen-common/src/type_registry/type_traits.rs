// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

use crate::type_to_string::path_to_string_fallback;
use crate::type_utils::{get_ident_of_last_path_segment_or_err, is_same_path};

#[derive(Debug)]
pub enum TypeCategory {
    Standard = 0,
    Cxx = 1,
    Qt = 2,
}

impl std::fmt::Display for TypeCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Standard => f.write_str("Standard"),
            Self::Cxx => f.write_str("CXX"),
            Self::Qt => f.write_str("Qt"),
        }
    }
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MetaTypeId {
    /// Builtin type having Id as a predefined constant.
    /// See <https://doc.qt.io/qt-6/qmetatype.html#Type-enum>.
    Constant(i32),

    // Type that can be registered in QMetaType registry at runtime.
    // QMetaType id is assigned to the type at runtime.
    Runtime,

    /// Not supposed to be treated via QMetaType.
    None,
}

impl ToTokens for MetaTypeId {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let code = match self {
            MetaTypeId::Constant(id) => quote! { MetaTypeId::Constant(#id)},
            MetaTypeId::Runtime => quote! { MetaTypeId::Runtime },
            MetaTypeId::None => quote! { MetaTypeId::None },
        };
        code.to_tokens(tokens);
    }
}

impl From<i32> for MetaTypeId {
    fn from(value: i32) -> Self {
        match value {
            1.. => MetaTypeId::Constant(value),
            0 => MetaTypeId::Runtime,
            ..0 => MetaTypeId::None,
        }
    }
}

pub trait TypeName {
    fn name(&self) -> &str;

    /// Return the full type name but without leading path segments (e.g. without 'qtbridge_type_lib' or 'qtbrdige')
    /// Needed for generic types that have generic arguments in angle brackets.
    fn full_name(&self) -> &str {
        self.name()
    }

    fn path_before_name(&self) -> Option<&str>;

    fn qualified_path_string(&self) -> String {
        let name = self.full_name();
        if let Some(path_before) = self.path_before_name()
            && !path_before.is_empty() {
                return format!("{path_before}::{name}")
        }

        name.into()
    }

    fn qualified_path_components(&self) -> Vec<&str> {
        let Some(path_before) = self.path_before_name() else {
            return vec![self.name()]
        };

        path_before.split("::")
            .filter(|comp| !comp.is_empty())
            .chain(std::iter::once(self.name()))
            .collect()
    }

    fn complement_partially_qualified_path(&self, path: &syn::Path) -> syn::Result<syn::Path> {
        let comps = self.qualified_path_components();
        if !is_same_path(path, comps.iter()) {
            return Err(syn::Error::new(path.span(), "Path has different components"))
        }

        let mut new_segs = path.segments.iter()
            .cloned()
            .collect::<Vec<_>>();

        let missing_seg_count = comps.len() as i32 - new_segs.len() as i32;
        if missing_seg_count > 0 {
            // Insert missing segments to front
            let missing_segs = comps.iter()
                .take(missing_seg_count as usize)
                .map(|seg_str| syn::PathSegment {
                    ident: format_ident!("{seg_str}"),
                    arguments: syn::PathArguments::None,
                });
            new_segs.splice(0..0, missing_segs);
        }

        Ok(syn::Path {
            leading_colon: None,
            segments: Punctuated::from_iter(new_segs),
        })
    }
}

pub trait TypeInfo: TypeName {
    fn generic_arg_count(&self) -> usize {
        0
    }

    fn cpp_name(&self) -> Option<&str> {
        None
    }

    fn cpp_namespace(&self) -> Option<&str> {
        None
    }

    fn cpp_name_qualified(&self) -> Option<String> {
        let name = self.cpp_name()?;
        let ns = self.cpp_namespace()
            .unwrap_or_default();
        let result = if ns.is_empty() { name.to_owned() } else { format!("::{ns}::{name}") };
        Some(result)
    }

    /// Return path to C++ header file for given type
    /// Returned path should not be delimited with quotes
    fn cpp_include(&self) -> Option<String> {
        None
    }

    fn metatype_id(&self) -> MetaTypeId {
        MetaTypeId::None
    }

    fn category(&self) -> TypeCategory {
        TypeCategory::Standard
    }

}

pub trait StaticTypeGroup: Sized {
    fn get_static_sorted_list() -> &'static [Self];
}

/// A trait for finding a type by its name or by a `syn::Path` in a certain category of types (e.g. primitives, arithmetical, strings, Qt types, etc.).
pub trait FindType: TypeName + Sized {
    /// Finds a type by its name in a group of types.
    ///
    /// The name must match the identifier of the last segment of the type's
    /// path (e.g. `Foo` in `crate::module::Foo<i32>`).
    fn find_by_name(name: &str) -> Option<Self>;

    /// Finds a type from a `syn::Path` (possibly partially-qualified) in a group of types.
    ///
    /// Returns `None` if the type is not found.
    fn find_by_path(path: &syn::Path) -> Option<Self> {
        get_type_by_path::<Self>(path)
            .ok()
            .flatten()
    }

    /// Finds a type from a `syn::Path` (possibly partially-qualified) in a group of types.
    ///
    /// Returns `syn::Error` if the type is not found.
    fn find_by_path_checked(path: &syn::Path) -> syn::Result<Self> {
        get_type_by_path::<Self>(path)?
            .ok_or_else(|| syn::Error::new(path.span(), format!("Failed to find type by path '{}'", path_to_string_fallback(path))))
    }
}

/// Finds a type from a `syn::Path` within a type group.
///
/// Returns:
/// - `Ok(Some(_))` if a matching type is found.
/// - `Ok(None)` if no matching type exists.
/// - `Err` if an error occurs during path processing.
pub fn get_type_by_path<T: FindType>(path: &syn::Path) -> syn::Result<Option<T>> {
    let last_seg_ident = get_ident_of_last_path_segment_or_err(path)?;
    let last_seg_str = last_seg_ident.to_string();

    let Some(ty) = T::find_by_name(&last_seg_str) else {
        return Ok(None)
    };
    let comps = ty.qualified_path_components();
    if !is_same_path(path, comps.iter()) {
        return Ok(None)
    }

    Ok(Some(ty))
}

impl<T: StaticTypeGroup + TypeName + Clone + 'static> FindType for T {
    fn find_by_name(name: &str) -> Option<Self> {
        let list = Self::get_static_sorted_list();
        list.binary_search_by(|ty| ty.name().cmp(name))
            .map(|idx| list[idx].clone())
            .ok()
    }
}


/// The type implementing this trait is an enum
/// where every variant of that enum is separate type category
pub trait TypesEnum {
    fn dyn_type_info(&self) -> &dyn TypeInfo;
}

/// Forward calls to underlying enum variant
impl<T: TypesEnum> TypeName for T {
    fn name(&self) -> &str {
        self.dyn_type_info().name()
    }

    fn full_name(&self) -> &str {
        self.dyn_type_info().full_name()
    }

    fn path_before_name(&self) -> Option<&str> {
        self.dyn_type_info().path_before_name()
    }
}

/// Forward calls to underlying enum variant
impl<T: TypesEnum> TypeInfo for T {
    fn generic_arg_count(&self) -> usize {
        self.dyn_type_info().generic_arg_count()
    }

    fn cpp_name(&self) -> Option<&str> {
        self.dyn_type_info().cpp_name()
    }

    fn cpp_namespace(&self) -> Option<&str> {
        self.dyn_type_info().cpp_namespace()
    }

    fn cpp_name_qualified(&self) -> Option<String> {
        self.dyn_type_info().cpp_name_qualified()
    }

    fn cpp_include(&self) -> Option<String> {
        self.dyn_type_info().cpp_include()
    }

    fn metatype_id(&self) -> MetaTypeId {
        self.dyn_type_info().metatype_id()
    }

    fn category(&self) -> TypeCategory {
        self.dyn_type_info().category()
    }
}
