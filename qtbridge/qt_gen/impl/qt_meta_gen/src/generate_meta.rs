// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qt_gen_common::type_qualified_mapping::CallOrigin;

use quote::{quote, ToTokens};
use proc_macro2::TokenStream;

use crate::traits::find_by_qml_name;
use crate::{QClassInfo, QPropertyInfo, QSignalInfo, QSlotInfo};

pub struct QMetaInfoContext<'a> {
    pub struct_ident: &'a syn::Ident,
    pub generics: &'a syn::Generics,
    pub cpp_iface_name: &'a str,
    pub signals: &'a [QSignalInfo],
    pub slots: &'a [QSlotInfo],
    pub properties: &'a [QPropertyInfo],
    pub class_infos: &'a [QClassInfo],
}

pub fn generate_qmetainfo_trait_impl(ctx: &QMetaInfoContext, origin: &CallOrigin) -> syn::Result<TokenStream> {
    let generics = &ctx.generics;
    let use_block = generate_meta_reg_use_block(ctx.signals, ctx.slots, ctx.properties, origin);
    let signals_meta_reg = generate_signals_meta_registration(ctx.signals)?;
    let slots_meta_reg = generate_slots_meta_registration(ctx.struct_ident, ctx.slots)?;
    let properties_meta_reg = generate_properties_meta_registration(ctx.struct_ident, ctx.properties, ctx.signals)?;
    let class_infos_reg = generate_class_infos_meta_registration(ctx.class_infos)?;

    let struct_ident = &ctx.struct_ident;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let bridge_library = origin.bridge_module();

    Ok(quote! {
        impl #impl_generics #bridge_library::QMetaInfo for #struct_ident #type_generics #where_clause {
            fn register_meta(mut meta_obj: std::pin::Pin<&mut #bridge_library::DynamicMetaObjectBuilder>) {
                #use_block

                #signals_meta_reg
                #slots_meta_reg
                #properties_meta_reg
                #class_infos_reg

                meta_obj.as_mut().end_meta_registration();
            }

            fn get_shared_dynamic_meta_object() -> &'static #bridge_library::DynamicMetaObjectBuilder {
                use std::any::TypeId;
                use std::cell::RefCell;
                use std::collections::HashMap;

                thread_local!(static DYNAMIC_META_MAP: RefCell<HashMap<TypeId, *const #bridge_library::DynamicMetaObjectBuilder>> =
                    RefCell::new(HashMap::new()));

                let type_id = TypeId::of::<#struct_ident #type_generics>();
                {
                    let meta_data_ptr = DYNAMIC_META_MAP.with_borrow(|dynamic_meta_data_map| {
                        dynamic_meta_data_map.get(&type_id)
                            .copied()
                            .unwrap_or_default()
                    });
                    if let Some(meta_data_ref) = unsafe { meta_data_ptr.as_ref() } {
                        return meta_data_ref;
                    }
                }

                let meta_data_ptr = #bridge_library::create_dynamic_meta_object_builder_for_type::<#struct_ident #type_generics>();
                let meta_data_ref = unsafe { meta_data_ptr.as_ref() }.unwrap();
                DYNAMIC_META_MAP.with_borrow_mut(|dynamic_meta_data_map| {
                    dynamic_meta_data_map.insert(type_id, meta_data_ptr);
                });

                meta_data_ref
            }
        }
    })
}

fn generate_meta_reg_use_block(signals: &[QSignalInfo], slots: &[QSlotInfo], properties: &[QPropertyInfo], origin: &CallOrigin) -> TokenStream {
    // generate code like 'use module::submodule;' to reduce amount of boiler plate
    // when accessing functions from other modules

    if signals.is_empty() && slots.is_empty() && properties.is_empty() {
        return TokenStream::new();
    }

    let type_library = origin.type_module();
    let bridge_library = origin.bridge_module();

    let import_type_lib = match origin {
        CallOrigin::Internal => None,
        CallOrigin::External => Some(quote! {use #type_library;})
    };

    let import_meta_type =  quote! {
        use qt_type_lib::{QMetaType, QMetaTypeId};
    };

    let is_qmeta_type_get_used = properties.iter()
        .any(|p| p.is_type_deduced_from_member());
    let import_get_meta_type_id = is_qmeta_type_get_used
        .then(|| quote! { use qt_type_lib::get_meta_type_id_of_fn_return_value; });

    let is_typed_arg_present =
        (!properties.is_empty() && properties.iter()
            .any(|p| !p.is_type_deduced_from_member())) ||
        signals.iter()
            .any(|s| s.get_typed_arg_count() > 0) ||
        slots.iter()
            .any(|s| s.get_typed_arg_count() > 0 || s.has_return());
    let import_qmeta_type_get = is_typed_arg_present
        .then(|| quote! { use qt_type_lib::QMetaTypeGet; });

    let mut meta_callbacks = Vec::new();
    if !slots.is_empty() {
        meta_callbacks.push(quote! {slot_callback_for});
    }
    if !properties.is_empty() {
        meta_callbacks.push(quote! {property_read_callback_for});
        if properties.iter().any(|p| !p.is_read_only()) {
            meta_callbacks.push(quote! {property_write_callback_for});
        }
    }

    let import_metacallbacks = match meta_callbacks.len() {
        0 => quote!{},
        1 => {
            let mcb = meta_callbacks.first().unwrap();
            quote!{ use #bridge_library::metacallbacks::#mcb; }
        },
        2.. => quote!{ use #bridge_library::metacallbacks::{#(#meta_callbacks),*};},
    };

    quote! {
        #import_type_lib
        #import_meta_type
        #import_get_meta_type_id
        #import_qmeta_type_get
        #import_metacallbacks
    }
}

//TODO: move to generic function and introduce trait for signal, slot, properties (e.g. 'RegisterMeta')
fn generate_signals_meta_registration(signals: &[QSignalInfo]) -> syn::Result<TokenStream> {
    let mut result = TokenStream::new();

    for signal in signals {
        let register_signal = signal.get_meta_registration_code()?;
        register_signal.to_tokens(&mut result);
    }

    Ok(result)
}

fn generate_slots_meta_registration(struct_ident: &syn::Ident, slots: &[QSlotInfo]) -> syn::Result<TokenStream> {
    let mut result = TokenStream::new();

    for slot in slots {
        let register_slot = slot.get_meta_registration_code(struct_ident)?;
        register_slot.to_tokens(&mut result);
    }

    Ok(result)
}

fn generate_properties_meta_registration(struct_ident: &syn::Ident, properties: &[QPropertyInfo], signals: &[QSignalInfo]) -> syn::Result<TokenStream> {
    let mut result = TokenStream::new();

    for property in properties {
        let mut signal = None;
        if let Some(notify_signal) = property.get_notify_signal() {
            let notify_signal_name = notify_signal.value();
            signal = find_by_qml_name(&notify_signal_name, &signals);
            if signal.is_none() {
                return Err(syn::Error::new(notify_signal.span(), format!("Failed to find signal with name '{notify_signal_name}'")));
            }
        }
        let register_property = property.get_meta_registration_code(struct_ident, signal)?;
        register_property.to_tokens(&mut result);
    }

    Ok(result)
}

fn generate_class_infos_meta_registration(class_infos: &[QClassInfo]) -> syn::Result<TokenStream> {
    let mut result = TokenStream::new();

    for class_info in class_infos {
        let register_class_info = class_info.get_meta_registration_code()?;
        register_class_info.to_tokens(&mut result);
    }

    Ok(result)
}
