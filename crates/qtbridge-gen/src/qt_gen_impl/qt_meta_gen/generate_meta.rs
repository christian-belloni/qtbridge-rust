// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge_gen_common::type_qualified_mapping::CallOrigin;
use qtbridge_gen_common::naming;

use quote::{ToTokens, format_ident, quote};
use proc_macro2::TokenStream;

use crate::qt_gen_impl::qt_meta_gen;
use qt_meta_gen::traits::find_by_qml_name;
use qt_meta_gen::{QClassInfo, QPropertyInfo, QSignalInfo, QSlotInfo};

pub struct QMetaInfoContext<'a> {
    pub struct_ident: &'a syn::Ident,
    pub iface_ident: &'a syn::Ident,
    pub generics: &'a syn::Generics,
    pub signals: &'a [QSignalInfo],
    pub slots: &'a [QSlotInfo],
    pub properties: &'a [QPropertyInfo],
    pub class_infos: &'a [QClassInfo],
}

pub fn generate_qmetainfo_trait_impl(ctx: &QMetaInfoContext, origin: &CallOrigin) -> syn::Result<TokenStream> {
    let generics = &ctx.generics;
    let use_block = generate_meta_reg_use_block(ctx.signals, ctx.slots, ctx.properties, origin);
    let signals_meta_reg = generate_signals_meta_registration(ctx.signals)?;
    let slots_meta_reg = generate_slots_meta_registration(ctx.slots)?;
    let properties_meta_reg = generate_properties_meta_registration(ctx.properties, ctx.signals)?;
    let class_infos_reg = generate_class_infos_meta_registration(ctx.class_infos)?;

    let iface_module = naming::rust::module::from_struct_name(ctx.iface_ident);
    let proxy_cpp = naming::rust::structure::proxy_cpp(ctx.iface_ident);

    let struct_ident = &ctx.struct_ident;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let bridge_library = origin.bridge_module();
    let iface_library = origin.iface_module();

    let has_generics = !generics.params.is_empty();
    let get_dyn_meta_object_body = if has_generics {
        quote! {
            #bridge_library::qmetainfo::dynamic_meta_object_data_for_generic::<Self>()
        }
    } else {
        quote! {
            use std::sync::OnceLock;
            thread_local! {
                static DYNAMIC_META_OBJECT: OnceLock<&'static #bridge_library::DynamicMetaObjectData> = OnceLock::new();
            }

            DYNAMIC_META_OBJECT.with(|cell| {
                *cell.get_or_init(|| {
                    let ptr = Self::create_dynamic_meta_object_data_for_type();
                    unsafe { ptr.as_ref() }.unwrap()
                })
            })
        }
    };

    Ok(quote! {
        impl #impl_generics #bridge_library::QMetaInfo for #struct_ident #type_generics #where_clause {

            type CppProxy = #iface_library::#iface_module::#proxy_cpp;

            fn build_dynamic_meta_type(mut meta_obj: std::pin::Pin<&mut #bridge_library::DynamicMetaObjectBuilder>) {
                #use_block

                #signals_meta_reg
                #slots_meta_reg
                #properties_meta_reg
                #class_infos_reg

                meta_obj.as_mut().end_meta_registration();
            }

            fn get_shared_dynamic_meta_object_data() -> &'static #bridge_library::DynamicMetaObjectData {
                #get_dyn_meta_object_body
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

    let is_property_with_not_deduced_type =
        properties.iter()
            .any(|p| !p.is_type_deduced());
    let is_qmeta_type_used =
        is_property_with_not_deduced_type ||
        slots.iter()
            .any(|s| !s.has_return());
    let is_qmeta_type_get_used =
        properties.iter()
            .any(|p| p.is_type_deduced()) ||
        signals.iter()
            .any(|s| s.get_typed_arg_count() > 0) ||
        slots.iter()
            .any(|s| s.get_typed_arg_count() > 0 || s.has_return());

    let mut type_lib_imports = Vec::new();
    if is_qmeta_type_used {
        type_lib_imports.push(format_ident!("QMetaType"));
    }
    if is_qmeta_type_get_used {
        type_lib_imports.push(format_ident!("QMetaTypeGet"));
    }

    let type_lib_imports = match type_lib_imports.len() {
        0 => quote!{},
        1 => {
            let use_ident = &type_lib_imports[0];
            quote!{
                use #type_library;
                use qtbridge_type_lib::#use_ident;
            }
        },
        _ => {
            type_lib_imports.sort();
            quote! {
                use #type_library;
                use qtbridge_type_lib::{#(#type_lib_imports),*};
            }
        }

    };

    let mut bridge_imports = Vec::new();
    if is_property_with_not_deduced_type {
        bridge_imports.push(quote!{ get_meta_type_of_fn_return_value });
    }

    let bridge_imports = (!bridge_imports.is_empty())
        .then(|| quote!{
            #(use #bridge_library::#bridge_imports;)*
        });

    quote! {
        #type_lib_imports
        #bridge_imports
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

fn generate_slots_meta_registration(slots: &[QSlotInfo]) -> syn::Result<TokenStream> {
    let mut result = TokenStream::new();

    for slot in slots {
        let register_slot = slot.get_meta_registration_code()?;
        register_slot.to_tokens(&mut result);
    }

    Ok(result)
}

fn generate_properties_meta_registration(properties: &[QPropertyInfo], signals: &[QSignalInfo]) -> syn::Result<TokenStream> {
    let mut result = TokenStream::new();

    for property in properties {
        let mut signal = None;
        if let Some(notify_signal) = property.get_notify_signal() {
            let notify_signal_name = notify_signal.value();
            signal = find_by_qml_name(&notify_signal_name, signals);
            if signal.is_none() {
                return Err(syn::Error::new(notify_signal.span(), format!("Failed to find signal with name '{notify_signal_name}'")));
            }
        }
        let register_property = property.get_meta_registration_code(signal)?;
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
