// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::collections::BTreeMap;

use quote::{format_ident, quote};
use syn::{spanned::Spanned, Token};

use qtbridge_gen_common::case_conv;
use qtbridge_gen_common::function_bridge::CppFunctionBridge;
use qtbridge_gen_common::multi_type_mapping::MultiTypeMapping;
use qtbridge_gen_common::naming;
use qtbridge_gen_common::signature_utils::change_first_arg;
use qtbridge_gen_common::type_mapping_nested::TypeMappingNested;
use qtbridge_gen_common::type_utils::ident_to_type;

use crate::cpp_fun::CppFun;
use crate::cpp_fun_processor::CppFunProcessor;
use crate::function_attributes::FunctionAttributes;
use crate::self_type_mapping::SelfTypeMapping;

#[derive(Clone)]
pub struct Function {
    attrs: Option<FunctionAttributes>,
    rust_func: syn::ItemFn,
    cpp_funcs: Vec<CppFun>,
}

impl Function {
    pub fn is_for_me(input: syn::parse::ParseStream) -> bool {
        let fork = input.fork();
        if let Ok(_maybe_pub) = fork.parse::<Option<Token![pub]>>() &&
           let Ok(_maybe_unsafe) = fork.parse::<Option<Token![unsafe]>>() &&
           let Ok(_fn) = fork.parse::<Token![fn]>()
        {
            return true
        }
        false
    }

    /// Replace generic idents with concrete types
    /// E.g., T -> i32
    pub fn substitute_types(&self, type_map: &TypeMappingNested<MultiTypeMapping>, self_type: &syn::Type) -> syn::Result<Self> {
        let src_func = &self.rust_func;
        let src_sig = &src_func.sig;

        if !src_sig.generics.params.is_empty() {
            return Err(syn::Error::new(src_sig.span(), "Function with generic parameters (that are not part of generic struct) are not currently supported"));
        }

        // TODO: add mapping for Self here locally?
        let new_cpp_funcs = self.cpp_funcs.iter()
            .map(|cpp_func| cpp_func.substitute_types(type_map, self_type))
            .collect::<syn::Result<_>>()?;

        Ok(Self {
            attrs: self.attrs.clone(),
            rust_func: type_map.map_item_fn(src_func)?,
            cpp_funcs: new_cpp_funcs,
        })
    }

    /// Replace generic QtTypes with argument with the concrete type in contained inlined C++ functions
    /// E.g., QHash<i32, f64> => QHash_i32_f64
    pub fn substitute_generic_qt_types_in_cpp_functions(&mut self) -> syn::Result<()> {
        self.cpp_funcs.iter_mut()
            .try_for_each(|cpp_fn| cpp_fn.substitute_generic_qt_types())
    }

    pub fn cpp_functions(&self) -> &[CppFun] {
        &self.cpp_funcs
    }

    pub fn signature(&self) -> &syn::Signature {
        &self.rust_func.sig
    }

    pub fn visibility(&self) -> &syn::Visibility{
        &self.rust_func.vis
    }

    pub fn set_attributes(&mut self, attributes: &[syn::Attribute]) -> syn::Result<()> {
        let attr = FunctionAttributes::new(attributes)?;
        self.attrs = Some(attr);
        Ok(())
    }

    pub fn docs(&self) -> &[syn::Attribute] {
        self.attrs.as_ref()
            .map(|attr| attr.docs())
            .unwrap_or_default()
    }

    pub fn get_rust_func(&self, name_prefix: &str) -> syn::Result<syn::ItemFn> {

        // replace inline function names
        let ident_map: BTreeMap<syn::Ident, syn::Type> = (0..self.cpp_funcs.len())
            .map(|idx| {
                let from_str = case_conv::camel_to_snake(&CppFunProcessor::inline_function_cpp_name_for_num(idx));
                let to_str = self.get_cpp_func_name(name_prefix, idx).1;
                let from = format_ident!("{from_str}");
                let to = ident_to_type(format_ident!("{to_str}"));
                (from, to)
            })
            .collect();

        let type_map = TypeMappingNested::new(MultiTypeMapping::new(ident_map));
        let mut result = type_map.map_item_fn(&self.rust_func)?;
        result.attrs = self.docs().into();
        Ok(result)
    }

    pub fn get_inline_functions_default_prefix() -> String {
        format!("{}_", naming::cpp::function::inline_function_prefix())
    }

    pub fn get_cpp_funcs_bridges(&self, name_prefix: &str, self_type: Option<&syn::Type>, is_opaque_struct: bool) -> syn::Result<Vec<CppFunctionBridge>> {
        let mut result = Vec::new();
        for fn_idx in 0..self.cpp_funcs.len() {
            let sign = self.get_cpp_func_bridge_signature(fn_idx, name_prefix, self_type, is_opaque_struct)?;

            let rust_name = case_conv::camel_to_snake(&sign.ident.to_string());
            let bridge = CppFunctionBridge::new(rust_name.into(), sign)?;
            result.push(bridge);
        }
        Ok(result)
    }

    pub fn get_cpp_funcs_cpp_code(&self, name_prefix: &str) -> syn::Result<(String, String)> {
        let mut decls = String::new();
        let mut defs = String::new();
        for (cpp_fn_num, cpp_fn) in self.cpp_funcs.iter().enumerate() {
            let cpp_name = self.get_cpp_func_name(name_prefix, cpp_fn_num).0;
            let (decl, def) = cpp_fn.get_code(&cpp_name)?;

            decls.push_str(&format!("\n{decl}"));
            defs.push_str(&format!("\n{def}"));
        }

        Ok((decls, defs))
    }

    fn get_cpp_func_bridge_signature(&self, cpp_fn_num: usize, name_prefix: &str, self_type: Option<&syn::Type>, is_opaque_struct: bool) -> syn::Result<syn::Signature> {
        let cpp_fn = self.cpp_funcs.get(cpp_fn_num).unwrap();

        // Replace Self with definite type
        // Replace 'self' with '_obj'

        let mut new_sign = cpp_fn.signature().clone();
        new_sign.ident = format_ident!("{}", self.get_cpp_func_name(name_prefix, cpp_fn_num).0);
        if let Some(receiver) = new_sign.receiver() {
            let syn::Type::Reference(receiver_type_ref) = receiver.ty.as_ref() else {
                return Err(syn::Error::new(receiver.span(), "self argument expected to be passed by reference"));
            };

            let new_arg_tok = if receiver_type_ref.mutability.is_some() {
                if is_opaque_struct {
                    quote!{ _obj: Pin<&mut #self_type> }
                }
                else {
                    quote! { _obj: &mut #self_type }
                }
            }
            else {
                quote!{ _obj: &#self_type }
            };

            let new_arg = syn::parse2(new_arg_tok)?;
            new_sign.inputs = change_first_arg(new_arg, &new_sign.inputs);
        }

        // Replace Self in return type and other arguments
        if let Some(self_type) = self_type {
            let self_type_map = TypeMappingNested::new(SelfTypeMapping::new(self_type.clone()));
            new_sign = self_type_map.map_signature(&new_sign)?;
        }

        Ok(new_sign)
    }

    fn get_cpp_func_name(&self, name_prefix: &str, cpp_fn_num: usize) -> (String, String) {
        let rust_func_ident = &self.rust_func.sig.ident;
        let mut cpp_name = format!("{name_prefix}{rust_func_ident}");
        if self.cpp_funcs.len() > 1 {
            cpp_name.push_str(&format!("_{cpp_fn_num}"));
        }
        let rust_name = case_conv::camel_to_snake(&cpp_name);

        (cpp_name, rust_name)
    }

}

impl syn::parse::Parse for Function {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let vis: syn::Visibility = input.parse()?;
        let sig: syn::Signature = input.parse()?;

        let content;
        let brace_token = syn::braced!(content in input);
        let stmts = content.call(syn::Block::parse_within)?;
        let block = syn::Block { stmts, brace_token };

        let mut proc = CppFunProcessor::new();
        let new_block = proc.process(&block)?;
        let cpp_funcs = proc.get_inlined_cpp_funcs();

        let func = syn::ItemFn {
            attrs: Vec::new(),
            vis,
            sig,
            block: Box::new(new_block),
        };

        Ok(Self {
            attrs: None,
            rust_func: func,
            cpp_funcs,
        })
    }
}
