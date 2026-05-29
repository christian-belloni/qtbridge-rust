// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident};
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;

use qtbridge_gen_common::function_with_attributes::FunctionWithAttributes;
use qtbridge_gen_common::parse_utils::is_path_with_segments_str;
use qtbridge_gen_common::type_qualified_mapping::{CallOrigin, TypeQualifiedMapping};
use qtbridge_gen_common::type_utils::get_ident_of_last_path_segment_or_err;
use crate::qt_gen_impl::generate_qobject_holder::generate_qobject_holder;
use crate::qt_gen_impl::qt_meta_gen;
use crate::qt_gen_impl::qt_meta_gen::generate_dispatch_meta_call::generate_dispatch_meta_call;
use qt_meta_gen::generate_meta::{QMetaInfoContext, generate_qmetainfo_trait_impl};
use qt_meta_gen::generate_qmetatype_get::{generate_qmeta_type_get};
use qt_meta_gen::traits::{QmlName, find_duplicate_by_qml_name};
use qt_meta_gen::{ExpandTokens, QClassInfo, QPropertyInfo, QSignalInfo, QSlotInfo};

use crate::qt_gen_impl;
use qt_gen_impl::qobject_macro_params::QObjectMacroParams;
use qt_gen_impl::qml_element::generate_qml_register;
use qt_gen_impl::drop_impl::{adjust_drop_impl, generate_drop};

pub struct QObjectModuleBuilder {
    params: QObjectMacroParams,
    struct_ident: syn::Ident,
    struct_generics: syn::Generics,
    struct_fields: syn::FieldsNamed,
    signals: Vec<QSignalInfo>,
    slots: Vec<QSlotInfo>,
    properties: Vec<QPropertyInfo>,
    class_infos: Vec<QClassInfo>,
    other_methods: Vec<syn::Signature>,
    is_drop_found: bool,
}

impl QObjectModuleBuilder {
    pub fn struct_is_generic(&self) -> bool {
        !self.struct_generics.params.is_empty()
    }

    pub fn new() -> Self {
        qtbridge_type_lib::init();
        Self {
            params: QObjectMacroParams::default(),
            struct_ident: format_ident!("dummy"),
            struct_generics: syn::Generics::default(),
            signals: Vec::new(),
            struct_fields: syn::parse_quote!{{}},
            slots: Vec::new(),
            properties: Vec::new(),
            class_infos: Vec::new(),
            other_methods: Vec::new(),
            is_drop_found: false,
        }
    }

    pub fn build_token_stream(&mut self, input: TokenStream, params: TokenStream) -> syn::Result<TokenStream> {
        self.build(input, params)
            .map(|item_mod| item_mod.to_token_stream())
    }

    pub fn build(&mut self, input: TokenStream, params: TokenStream) -> syn::Result<syn::ItemMod> {
        // Parse input parameters of this macro,
        self.params = syn::parse2::<QObjectMacroParams>(params)
            .map_err(|err| syn::Error::new(err.span(), format!("Failed to parse input params.\nError: {err}")))?;

        // Parse input token stream as 'mod' item,
        let module = syn::parse2::<syn::ItemMod>(input)
            .map_err(|err| syn::Error::new(err.span(), format!("Failed to parse input as mod block.\nError: {err}")))?;

        // Process input module. Generate output module (after expansion of the macro).
        // Extract signals, slots, properties, class infos, and other relevant parts.
        let mut output_module_items = self.handle_item_mod(&module)?;

        self.check_duplicates()?;
        QPropertyInfo::check_single_default_property(&self.properties)?;

        // Try to deduce properties type here when we have list of potential getters/setters
        for prop in &mut self.properties {
            prop.set_type(&self.other_methods, Some(&self.struct_fields))?;
            prop.validate(&self.signals)
                .map_err(|err| syn::Error::new(err.span(), format!("Wrong property declaration: {err}")))?;
        }

        // Code generation for QObject interface implementation
        // Load interface from description file
        let iface_ident = match self.params.base.as_ref() {
            Some(base) => base.clone(),
            None => syn::parse_str::<syn::Ident>("QObject")?,
        };

        // Generate blocks of code that will be added to expanded code.

        // TODO: pass QObjectModule to generate_qmetainfo_trait_impl() instead
        let ctx = QMetaInfoContext {
            struct_ident: &self.struct_ident,
            iface_ident: &iface_ident,
            generics: &self.struct_generics,
            signals: &self.signals,
            slots: &self.slots,
            properties: &self.properties,
            class_infos: &self.class_infos,
        };

        // Generate traits implementations.
        let mut generated_traits = vec![
            ("DispatchMetaCall", generate_dispatch_meta_call(&self.struct_ident, &self.struct_generics, &self.signals, &self.slots, &self.properties)),
            ("QMetaInfo",        generate_qmetainfo_trait_impl(&ctx)),
            ("QMetaTypeGet",     generate_qmeta_type_get(&self.struct_ident, &self.struct_generics)),
            ("QObjectHolder",    generate_qobject_holder(&self.struct_ident, &iface_ident, &self.struct_generics)),
        ];

        if !self.params.no_drop &&
           let Some(drop) = self.generate_drop_trait_if_missing().transpose()
        {
            generated_traits.push(("Drop", drop));
        }

        if !self.struct_is_generic() {
            let qml_register = generate_qml_register(&self.struct_ident, &self.params)
                .map_err(|err| syn::Error::new(err.span(), format!("Failed to generate implementation of QmlRegister trait.\nError: {}", err)))?;
            if let Some(qml_reg) = qml_register {
                if let Some(qml_reg_func) = qml_reg.register_fn {
                    output_module_items.push(qml_reg_func.into());
                }
                generated_traits.push(("QmlRegister", Ok(qml_reg.register_impl)));
            }
        } else if self.params.singleton {
            return Err(syn::Error::new(self.struct_ident.span(), format!("Singleton is not available for generic structs.")));
        }

        let mut type_map = TypeQualifiedMapping::new(CallOrigin::External);
        for (trait_name, trait_impl_result) in generated_traits {
            let mut trait_impl = trait_impl_result.map_err(|err| syn::Error::new(
                err.span(),
                format!("Failed to generate implementation of '{trait_name}' trait.\nError:{err}")))?;

            // Make sure paths are properly qualified in the generated traits.
            type_map.visit_item_impl_mut(&mut trait_impl);
            output_module_items.push(trait_impl.into());
        }

        Ok(syn::ItemMod {
            content: Some((<_>::default(), output_module_items)),
            ..module.clone()
        })
    }

    fn handle_item_mod(&mut self, input: &syn::ItemMod) -> syn::Result<Vec<syn::Item>> {
        // Get items contained in the given mod block.
        let (_brace, mod_items) = input.content.as_ref()
            .ok_or_else(|| syn::Error::new(input.span(), "mod expected to contain a code block in curly brackets"))?;

        // Filter items with struct declaration.
        let struct_items: Vec<&syn::ItemStruct> = mod_items.iter()
            .filter_map(|item| match item {
                syn::Item::Struct(struct_) => Some(struct_),
                _ => None,
            })
            .collect();
        let struct_ = struct_items.first()
            .ok_or_else(|| syn::Error::new(input.span(), "mod annotation with #[qobject] must contain a struct"))?;
        if let Some(second_struct) = struct_items.get(1) {
            return Err(syn::Error::new(second_struct.span(), "Only single struct is allowed in module annotated with #[qobject]"))
        }
        self.struct_ident = struct_.ident.clone();
        self.struct_generics = struct_.generics.clone();
        self.struct_fields = match &struct_.fields {
            syn::Fields::Named(named) => named.clone(),
            _ => return Err(syn::Error::new(struct_.fields.span(), "Only named struct fields are supported"))
        };

        // Process input items of module. Expand item to output module
        let mut output_items = Vec::new();
        for item in mod_items.iter() {
            output_items.push(self.handle_item(item)?);
        }

        Ok(output_items)
    }

    fn handle_item(&mut self, input: &syn::Item) -> syn::Result<syn::Item> {
        match input {
            syn::Item::Impl(item_impl) => {
                let result = match &item_impl.trait_ {
                    Some(_) => self.handle_item_impl_trait(item_impl),
                    None => self.handle_item_impl_struct(item_impl),
                }?;
                Ok(result.into())
            }
            _ => Ok(input.clone()),
        }
    }

    /// Handle block
    /// impl SomeTrait for SomeStruct
    fn handle_item_impl_trait(&mut self, input: &syn::ItemImpl) -> syn::Result<syn::ItemImpl> {

        let path = &input.trait_.as_ref().unwrap().1;
        let last_seg_ident = get_ident_of_last_path_segment_or_err(path)?;

        match last_seg_ident.to_string().as_str() {
            "Drop" => {
                if !self.params.no_drop {
                    if path.is_ident("Drop") ||
                    is_path_with_segments_str(path, "std::ops::Drop") ||
                    is_path_with_segments_str(path, "core::ops::Drop")
                    {
                        self.is_drop_found = true;
                        return adjust_drop_impl(input)
                    }
                }
            },
            _ => {},
        }

        // Will be handled later
        Ok(input.clone())
    }

    /// Handle block
    /// impl SomeStruct
    fn handle_item_impl_struct(&mut self, input: &syn::ItemImpl) -> syn::Result<syn::ItemImpl> {
        let mut output_items = Vec::new();

        for input_item in &input.items {
            let opt_output_item = self.handle_impl_item(input_item)
                .map_err(|err |syn::Error::new(err.span(), format!("Failed to process ImplItem.\nError: {err}")))?;
            let Some(output_item) = opt_output_item else {
                continue
            };
            output_items.push(output_item);
        }

        Ok(syn::ItemImpl {
            items: output_items,
            ..input.clone()
        })
    }

    fn handle_impl_item(&mut self, input: &syn::ImplItem) -> syn::Result<Option<syn::ImplItem>> {
        match input {
            syn::ImplItem::Fn(item_fn) =>
                self.handle_impl_item_fn(item_fn)
                    .map(|opt_fn| opt_fn.map(syn::ImplItem::from)),
            syn::ImplItem::Macro(item_macro) =>
                self.handle_impl_item_macro(item_macro)
                    .map(|opt_macro| opt_macro.map(syn::ImplItem::from)),
            syn::ImplItem::Verbatim(tokens) =>
                self.handle_impl_item_verbatim(tokens)
                    .map(|opt_tokens| opt_tokens.map(syn::ImplItem::Verbatim)),
            _ => Ok(Some(input.clone())),
        }
    }

    fn handle_impl_item_fn(&mut self, input: &syn::ImplItemFn) -> syn::Result<Option<syn::ImplItemFn>> {
        if input.attrs.is_empty() {
            // No attributes - then it is not a slot, signal or method
            self.other_methods.push(input.sig.clone());
            return Ok(Some(input.clone()))
        }

        let Some(meta_attr) = Self::get_meta_attribute(&input.attrs)? else {
            return Ok(Some(input.clone()))
        };
        let func = syn::parse2::<FunctionWithAttributes>(input.to_token_stream())?;

        let output;
        if QSlotInfo::is_for_me(meta_attr) {
            let slot = QSlotInfo::new(func, self.get_slot_id(), self.params.clone())?;
            output = slot.expand_tokens()?;
            self.slots.push(slot);
        }
        else if QSignalInfo::is_for_me(meta_attr) {
            let signal = QSignalInfo::new(func,  self.params.clone())?;
            output = signal.expand_tokens()?;
            self.signals.push(signal);
        }
        else {
            unreachable!()
        }

        Ok(Some(syn::parse2(output)?))
    }

    fn handle_impl_item_macro(&mut self, input: &syn::ImplItemMacro) -> syn::Result<Option<syn::ImplItemMacro>> {
        let name = input.mac.path
            .get_ident()
            .map(syn::Ident::to_string)
            .unwrap_or_default();
        match name.as_str() {
            "qproperty" => {
                let property = QPropertyInfo::new(input, self.get_property_id())?
                    .ok_or_else(|| syn::Error::new(input.span(), "Not a qproperty"))?;
                self.properties.push(property);
                Ok(None)
            }
            "qclass_info" => {
                let class_info = QClassInfo::new(input)?
                    .ok_or_else(|| syn::Error::new(input.span(), "Not a ClassInfo"))?;
                self.class_infos.push(class_info);
                Ok(None)
            }
            _ => Ok(Some(input.clone()))
        }
    }

    fn get_slot_id(&self) -> u32 {
        self.slots.len() as u32 + 1
    }

    fn get_property_id(&self) -> u32 {
        self.properties.len() as u32 + 1
    }

    fn handle_impl_item_verbatim(&mut self, input: &TokenStream) -> syn::Result<Option<TokenStream>> {
        let func = syn::parse2::<FunctionWithAttributes>(input.clone())?;
        let Some(meta_attr) = Self::get_meta_attribute(&func.attrs)? else {
            return Ok(Some(input.clone()))
        };

        if QSignalInfo::is_for_me(meta_attr) {
            let signal = QSignalInfo::new(func,  self.params.clone())?;
            let output = signal.expand_tokens()?;
            self.signals.push(signal);
            return Ok(Some(output))
        }

        Ok(Some(input.clone()))
    }

    /// Generate impl Drop for given struct in which we delete attached qobject.
    fn generate_drop_trait_if_missing(&self) -> syn::Result<Option<syn::ItemImpl>> {
        if self.is_drop_found {
            return Ok(None)
        }

        Ok(Some(generate_drop(&self.struct_ident, &self.struct_generics)?))
    }

    fn get_meta_attribute(input: &[syn::Attribute]) -> syn::Result<Option<&syn::Attribute>> {
        let meta_attrs: Vec<_> = input.iter()
            .filter(|attr|
                QSlotInfo::is_for_me(attr) ||
                QSignalInfo::is_for_me(attr)
            )
            .take(2)
            .collect();
        match meta_attrs.len() {
            0 => Ok(None),
            1 => Ok(Some(meta_attrs[0])),
            2.. => {
                let second = meta_attrs[1];
                Err(syn::Error::new(second.span(), "The meta attribute conflicts with an attribute above"))
            }
        }
    }

    fn check_duplicates(&self) -> syn::Result<()> {
        if let Some(dup) = find_duplicate_by_qml_name(&self.signals) {
            let (name, span) = dup.get_qml_name_span();
            return Err(syn::Error::new(span, format!("Signal '{name}' was already declared")));
        }

        if let Some(dup) = find_duplicate_by_qml_name(&self.slots) {
            let (name, span) = dup.get_qml_name_span();
            return Err(syn::Error::new(span, format!("Slot '{name}' was already declared")));
        }

        if let Some(dup) = find_duplicate_by_qml_name(&self.properties) {
            let (name, span) = dup.get_qml_name_span();
            return Err(syn::Error::new(span, format!("Property '{name}' was already declared")));
        }

        Ok(())
    }
}
