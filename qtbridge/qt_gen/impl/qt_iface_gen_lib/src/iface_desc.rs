// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::collections::BTreeSet;

use proc_macro2::Span;
use quote::format_ident;
use syn::{spanned::Spanned, Token};

use qt_gen_common::case_conv;
use qt_gen_common::cpp_include::CppInclude;
use qt_gen_common::signature_utils::signature_eq;

use crate::iface_desc_method::{IfaceMethodDesc, MethodKind};

pub struct InterfaceDesc {
    name: syn::Ident,
    includes: Vec<CppInclude>,
    methods: Vec<IfaceMethodDesc>,
}

impl InterfaceDesc {
    pub fn new_from_ident(name: &syn::Ident) -> syn::Result<Self> {
        let name_str = name.to_string();
        let code = get_iface_desc_code(&name_str)
            .ok_or_else(|| syn::Error::new(name.span(), format!("Definition for interface '{}' not found", name_str)))?;

        let mut desc = syn::parse_str::<InterfaceDesc>(code)?;
        desc.name = name.clone();
        Ok(desc)
    }

    pub fn new_from_name_str(name: &str) -> syn::Result<Self>{
        let name_ident = format_ident!("{name}");
        Self::new_from_ident(&name_ident)
    }

    pub fn check_is_valid_virtual_override(&self, func_sig: &syn::Signature, cpp_name: &syn::Ident) -> syn::Result<()> {
        let cpp_name_str = cpp_name.to_string();
        let desc_methods = self.find_method_by_cpp_name(&cpp_name_str)
            .ok_or_else(|| syn::Error::new(cpp_name.span(), format!("Virtual method '{}' not found in declaration of Rust interface '{}'", cpp_name_str, self.name)))?;

        if !desc_methods.is_virtual() {
            return Err(syn::Error::new(cpp_name.span(), format!("Method '{}' is not virtual", cpp_name_str)));
        }

        let func_args = &func_sig.inputs;
        if let Some(first_func_arg) = func_args.first() {
            let syn::FnArg::Receiver(func_receiver) = first_func_arg else {
                return Err(syn::Error::new(first_func_arg.span(), "Virtual method should have receiver as the first argument (&[mut] self)"));
            };
            if func_receiver.reference.is_none() {
                return Err(syn::Error::new(first_func_arg.span(), "Self argument of virtual method should be passed by reference"));
            };

            let desc_receiver = desc_methods.get_receiver()?;
            if desc_receiver.mutability != func_receiver.mutability {
                let expected_self = if desc_receiver.mutability.is_some() { "&mut self" } else { "&self" };
                return Err(syn::Error::new(func_receiver.ty.span(), format!("First argument expected to be '{expected_self}'")));
            }
        } else {
            return Err(syn::Error::new(func_sig.ident.span(), "Virtual method should have at least one argument (for &self)"));
        }

        signature_eq(&func_sig, &desc_methods.get_signature())
    }

    pub fn find_method_by_cpp_name(&self, cpp_name: &str) -> Option<&IfaceMethodDesc> {
        self.methods.iter()
            .find(|method| method.get_cpp_name() == cpp_name)
    }

    pub fn get_ident(&self) -> &syn::Ident {
        &self.name
    }

    pub fn get_includes(&self) -> &Vec<CppInclude> {
        &self.includes
    }

    pub fn get_methods(&self) -> &Vec<IfaceMethodDesc> {
        &self.methods
    }

    pub fn get_virtual_methods(&self) -> impl Iterator<Item = &IfaceMethodDesc> {
        self.methods.iter()
            .filter(|m| m.is_virtual())
    }

    pub fn has_impl_methods(&self) -> bool {
        self.methods.iter()
            .any(|m| m.has_base_implementation())
    }

    pub fn get_implemented_methods(&self) -> impl Iterator<Item = &IfaceMethodDesc> {
        self.methods.iter()
            .filter(|m| m.has_base_implementation())
    }

}


mod iface_keywords {
    syn::custom_keyword!(include);
}


impl syn::parse::Parse for InterfaceDesc {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut includes = Vec::new();
        let mut methods = Vec::new();

        let mut unique_funcs = BTreeSet::new();

        while !input.is_empty() {
            if CppInclude::is_for_me(input) {
                includes.push(input.parse()?);
            }
            else if input.peek(Token![fn]) || input.peek(Token![#]) {
                // TODO: have custom code of parsing instead of relying on ForeignItemFn?
                let func = input.parse::<syn::ForeignItemFn>()?;

                let attrs = &func.attrs;
                if attrs.len() > 1 {
                    return Err(syn::Error::new(attrs[1].span(), "At most one attribute is allowed"));
                }

                let kind = if let Some(attr) = attrs.first() {
                    let ident = attr.path().get_ident()
                        .ok_or_else(|| syn::Error::new(attr.span(), "Invalid attribute format"))?;

                    match ident.to_string().as_str() {
                        "pure"   => MethodKind::PureVirtual,
                        "impl"   => MethodKind::ImplVirtual,
                        "novirt" => MethodKind::NonVirtual,
                        _ => return Err(syn::Error::new(ident.span(), "Unsupported annotation token")),
                    }
                } else {
                    MethodKind::ImplVirtual
                };

                let sig = func.sig;
                let rust_name = sig.ident.to_string();
                let cpp_name = case_conv::snake_to_camel(&rust_name);
                if !unique_funcs.insert(rust_name) {
                    return Err(syn::Error::new(sig.ident.span(), "Items in interface declaration must not have duplicate"));
                }

                methods.push(IfaceMethodDesc::new(kind, sig, cpp_name));
            } else {
                return Err(input.error(format!("Unknown token type: {}", input.to_string())));
            }
        }

        Ok(InterfaceDesc{
            name: syn::Ident::new("uninitialized", Span::call_site()), // will be replaced in Self::new()
            includes,
            methods,
        })
    }
}

pub fn get_iface_desc_code(name: &str) -> Option<&'static str> {
    let iface = match name {
        //TODO: shorten with macro_rules!
        "QObject"            => include_str!("input/QObject"),
        "QAbstractItemModel" => include_str!("input/QAbstractItemModel"),
        "QAbstractListModel" => include_str!("input/QAbstractListModel"),
        _ => return None,
    };

    Some(iface)
}
