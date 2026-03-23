// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::path::Path;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use qt_gen_common::multi_type_mapping::MultiTypeMapping;
use qt_gen_common::path_utils::relative_input_file_path_to_path_qualified;
use qt_gen_common::type_mapping_nested::TypeMappingNested;
use qt_gen_common::type_to_string::path_to_ident_str;

use crate::function::Function;
use crate::generic_instantiation_decl::GenericInstantiationTypes;
use crate::module::Module;
use crate::structure::BridgeStruct;
use crate::submodule_type_tokens::SubmoduleTypeTokens;
use crate::trait_impl::TraitImpl;

pub fn get_monomorped_struct_ident(struct_: &BridgeStruct, inst: &GenericInstantiationTypes) -> syn::Result<syn::Ident> {
    let name = get_monomorped_struct_name(struct_, inst)?;
    Ok(syn::Ident::new(&name, struct_.ident().span()))
}

fn get_monomorped_struct_name(struct_: &BridgeStruct, inst: &GenericInstantiationTypes) -> syn::Result<String> {
    let src_ident = struct_.ident();
    let inst_suffix = inst.list().iter()
        .map(path_to_ident_str)
        .collect::<syn::Result<Vec<_>>>()?
        .join("_");

    if inst_suffix.is_empty() {
        Ok(src_ident.to_string())
    }
    else {
        Ok(format!("{src_ident}_{inst_suffix}"))
    }
}

pub fn get_submod_ident(module: &Module, inst: Option<&GenericInstantiationTypes>) -> syn::Result<syn::Ident> {
    if let Some(struct_) = module.structure() {
        let mut str = match inst {
            Some(inst) => get_monomorped_struct_name(struct_, inst)?,
            None => struct_.ident().to_string(),
        };
        str = str.to_ascii_lowercase();
        return Ok(syn::Ident::new(&str, struct_.ident().span()))
    }

    Ok(module.ident().clone())
}

pub fn get_traits_substituted<'a>(src: impl Iterator<Item = &'a TraitImpl>, type_map: &TypeMappingNested<MultiTypeMapping>) -> syn::Result<Vec<TraitImpl>> {
    let result = src
        .map(|tr| tr.get_instantiations_with_types_substituted(type_map))
        .collect::<syn::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect();
    Ok(result)
}

pub fn get_functions_substituted(src: &[Function], self_type: &syn::Path, type_map: &TypeMappingNested<MultiTypeMapping>) -> syn::Result<Vec<Function>> {
    src.iter()
        .map(|func| func.substitute_types(type_map, self_type))
        .collect::<syn::Result<_>>()
}

pub fn get_unresolved_type_dependencies(type_tokens: &SubmoduleTypeTokens) -> Vec<syn::Path> {
    type_tokens.all()
        .iter_unclassified().cloned()
        .collect()
}

pub fn file_path_to_qualified_path_before_name(src: &Path) -> Result<String, String> {
    file_path_str_to_qualified_path_before_name(src.to_string_lossy().as_ref())
}

pub fn file_path_str_to_qualified_path_before_name(src: &str) -> Result<String, String> {
    let comps = relative_input_file_path_to_path_qualified(src)?;
    Ok(comps.join("::"))
}

pub fn get_derive_attr(derive_traits: &[String]) -> Option<TokenStream> {
    (!derive_traits.is_empty())
        .then(|| {
            let traits = derive_traits.iter()
                .map(|st| format_ident!("{st}"));
            quote! { #[derive(#(#traits)*)] }
        })
}
