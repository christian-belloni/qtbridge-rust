// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::Span;
use syn::spanned::Spanned;

use crate::type_utils::{ get_type_pass, is_rust_type_mapped_to_qmetatype, rust_type_to_qmetatype, unwrapped_ref_to_string, ValuePass};
use crate::type_to_cpp::type_to_cpp;


pub struct RustTypeInfo<'a> {
    ty: &'a syn::Type,
}

impl<'a> RustTypeInfo<'a> {
    pub fn new(ty: &'a syn::Type) -> Self{
        Self { ty }
    }

    pub fn get_type(&self) -> &syn::Type {
        self.ty
    }

    pub fn span(&self) -> Span {
        self.ty.span()
    }

    pub fn unwrapped_ref_to_str(&self) -> syn::Result<String> {
        unwrapped_ref_to_string(self.ty)
    }

    pub fn is_mapped_to_qmetatype(&self) -> bool {
        let str = match self.unwrapped_ref_to_str() {
            Ok(str) => str,
            Err(_) => return false,
        };

        is_rust_type_mapped_to_qmetatype(&str)
    }

    pub fn to_qmeta_type(&self) -> syn::Result<Option<&'static str>> {
        let rust_type_str = self.unwrapped_ref_to_str()?;
        Ok(rust_type_to_qmetatype(&rust_type_str))
    }

    pub fn to_cpp_type(&self) -> syn::Result<String> {
        type_to_cpp(self.get_type())
    }

    pub fn unwrap_if_ref(&self) -> Self {
        if self.is_ref() {
            self.unwrap_ref()
        }
        else {
            Self { ty: self.ty }
        }
    }

    pub fn unwrap_ref(&self) -> Self {
        match &self.ty {
            syn::Type::Reference(type_ref)
                => Self { ty: &*type_ref.elem },
            _ => panic!("Type is not a reference: {:#?}", self.ty)
        }
    }

    pub fn is_ref(&self) -> bool {
        match self.ty {
            syn::Type::Reference(_) => true,
            _ => false,
        }
    }

    pub fn is_const_ref(&self) -> bool {
        match &self.ty {
            syn::Type::Reference(type_ref) =>
                type_ref.mutability.is_none(),
            _ => false,
        }
    }

    pub fn is_mut_ref(&self) -> bool {
        match &self.ty {
            syn::Type::Reference(type_ref) =>
                type_ref.mutability.is_some(),
            _ => false,
        }
    }

    pub fn is_ptr(&self) -> bool {
        match &self.ty {
            syn::Type::Ptr(_) => true,
            _ => false,
        }
    }

    pub fn get_value_pass(&self) -> ValuePass {
        get_type_pass(&self.ty)
    }
}
