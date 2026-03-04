// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::TokenStream;
use qt_gen_common_no_types::type_mapping_nested::TypeMappingNested;
use quote::{ToTokens, quote};
use syn::parse::Parse;

use qt_gen_common_no_types::multi_type_mapping::MultiTypeMapping;
use qt_gen_common_no_types::type_to_string::path_to_ident_str;
use crate::generic_instantiation_decl::{GenericInstantiationTypes, GenericInstantiationsList};
use crate::function::Function;
use crate::generic_types_instantiations::GenericTypesInstantiations;
use crate::trait_impl_attributes::TraitImplAttributes;
use crate::trait_impl_generics::TraitImplGenericList;


#[derive(Clone)]
pub struct TraitImpl {
    name: syn::Path,
    generics: TraitImplGenericList,
    self_type: syn::Path,
    other_items: Vec<syn::ImplItem>,
    funcs: Vec<Function>,
    attrs: Option<TraitImplAttributes>,
}

impl TraitImpl {
    pub fn is_for_me(input: syn::parse::ParseStream) -> bool {
        input.peek(syn::Token![impl])
    }

    pub fn generics(&self) -> &TraitImplGenericList {
        &self.generics
    }

    pub fn self_type(&self) -> &syn::Path {
        &self.self_type
    }

    pub fn functions(&self) -> &[Function] {
        &self.funcs
    }

    pub fn is_included_for_struct_instantiations(&self, struct_inst: &GenericInstantiationTypes) -> bool {
        let Some(attr) = self.attrs.as_ref() else {
            return true;
        };

        if let Some(incl) = attr.instantiation_inclusions() {
            return incl.list().iter()
                        .any(|incl_types| incl_types == struct_inst)
        }
        else if let Some(excl) = attr.instantiation_exclusions() {
            return excl.list().iter()
                        .all(|excl_types| excl_types != struct_inst)
        }

        true
    }

    pub fn get_instantiations(&self) -> Option<&GenericInstantiationsList> {
        self.attrs.as_ref()
            .and_then(|attr| attr.instantiations())
    }

    pub fn get_instantiations_with_types_substituted(&self, struct_type_map: &TypeMappingNested<MultiTypeMapping>) -> syn::Result<Vec<Self>> {
        if let Some(insts) = self.get_instantiations() {
            let trait_gen_types_insts = GenericTypesInstantiations::new(self.generics().idents(), insts)?;

            let result = trait_gen_types_insts.iter_type_maps()
                .map(|mut trait_type_map| {
                    trait_type_map.extend(struct_type_map.get_impl().iter());
                    self.substitute_types(&TypeMappingNested::new(trait_type_map))
                })
                .collect::<syn::Result<_>>()?;

            Ok(result)
        }
        else {
            Ok(vec![self.substitute_types(struct_type_map)?])
        }
    }

    pub fn set_attributes(&mut self, attributes: &[syn::Attribute]) -> syn::Result<()> {
        self.attrs = Some(TraitImplAttributes::new(attributes)?);
        Ok(())
    }

    /// Replace generic idents with concrete types
    /// E.g., T -> i32
    fn substitute_types(&self, type_map: &TypeMappingNested<MultiTypeMapping>) -> syn::Result<Self> {
        let name = type_map.map_path(&self.name);
        let generics = self.generics().clone_generics();
        let self_type = type_map.map_path(&self.self_type);

        let other_items = self.other_items.iter()
            .map(|item| type_map.map_impl_item(item))
            .collect();

        let funcs = self.funcs.iter()
            .map(|f| f.substitute_types(type_map, &self_type))
            .collect::<syn::Result<_>>()?;

        Ok(Self {
            name,
            generics,
            self_type,
            other_items,
            funcs,
            attrs: None,
        })
    }

    /// Replace generic QtTypes with concrete type in contained inlined C++ functions
    /// E.g., QHash<i32, f64> => QHash_i32_f64
    pub fn substitute_generic_qt_types_in_cpp_functions(&mut self) -> syn::Result<()> {
        self.funcs.iter_mut()
            .try_for_each(|func| func.substitute_generic_qt_types_in_cpp_functions())
    }

    pub fn get_rust_code(&self, func_name_prefix: &str) -> TokenStream {
        let mut func_tokens = TokenStream::new();
        for func in self.functions() {
            func.get_rust_func(func_name_prefix)
                .to_tokens(&mut func_tokens);
        }

        let Self { name, generics, self_type, other_items, .. } = self;
        let const_generics: Vec<_> = generics.consts()
            .collect();

        let impl_generics = (!const_generics.is_empty())
            .then(|| quote! { <#(#const_generics)*> });

        quote! {
            impl #impl_generics #name for #self_type {
                #(#other_items)*
                #func_tokens
            }
        }
    }

    pub fn get_inline_trait_functions_default_prefix(&self) -> syn::Result<String> {
        let func_prefix = Function::get_inline_functions_default_prefix();
        let name = path_to_ident_str(&self.name)?;
        let self_type = path_to_ident_str(&self.self_type)?;
        Ok(format!("{func_prefix}TraitImpl_{name}_for_{self_type}_"))
    }

}

impl Parse for TraitImpl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _impl: syn::Token![impl] = input.parse()?;
        let generics = if input.peek(syn::Token![<]) {
            input.parse()?
        }
        else {
            TraitImplGenericList::default()
        };
        let name = input.parse()?;

        let _for: syn::Token![for] = input.parse()?;
        let self_type = input.parse()?;

        let content;
        let _braces = syn::braced!(content in input);

        let mut funcs = Vec::<Function>::new();
        let mut other_items = Vec::new();

        while !content.is_empty() {
            if Function::is_for_me(&content) {
                funcs.push(content.parse()?);
            }
            else {
                other_items.push(content.parse()?);
            }
        }

        Ok(TraitImpl {
            name,
            generics,
            self_type,
            funcs,
            other_items,
            attrs: None,
        })
    }
}
