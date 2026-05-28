// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
use proc_macro2::TokenStream;
use quote::quote;
use syn::visit_mut::VisitMut;
use qtbridge_gen_common::type_qualified_mapping;
use type_qualified_mapping::{TypeQualifiedMapping, CallOrigin, crate_names};

use super::{QPropertyInfo, QSignalInfo, QSlotInfo};

pub fn generate_dispatch_meta_call(struct_ident: &syn::Ident, generics: &syn::Generics,
    signals: &[QSignalInfo], slots: &[QSlotInfo], properties: &[QPropertyInfo]) -> syn::Result<syn::ItemImpl> {

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let type_library = crate_names::type_module();
    let bridge_library = crate_names::bridge_module();

    let (slots_mut, slots_const): (Vec<_>, Vec<_>) = slots.iter()
        .partition(|s| s.is_mut());

    let slot_mut_handlers = slots_mut.into_iter()
        .map(get_slot_handler_code)
        .collect::<syn::Result<Vec<_>>>()?;
    let slot_const_handlers = slots_const.into_iter()
        .map(get_slot_handler_code)
        .collect::<syn::Result<Vec<_>>>()?;

    let prop_read_handlers = properties.iter()
        .map(|prop| {
            let id = prop.id();
            let code = prop.get_read_code()?;
            Ok(quote! {
                #id => {
                    #code
                }
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;

    let prop_write_handlers = properties.iter()
        .map(|prop| {
            let id = prop.id();
            let signal = signals.iter().find(|s| {
                prop.get_notify_signal()
                    .map_or(false, |notify| s.get_rust_name() == *notify)
            });
            let code = prop.get_write_code(signal)?;
            Ok(quote! {
                #id => {
                    #code
                }
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;

    let code = quote! {
        impl #impl_generics #bridge_library::DispatchMetaCall for #struct_ident #type_generics
        #where_clause
        {
            fn invoke_slot(&self, slot_id: u32, inputs: &[*const u8], outputs: &[*mut u8]) {
                match slot_id {
                    #(#slot_const_handlers),*
                    _ => panic!("Unhandled slot id {slot_id}")
                }
            }
            fn invoke_slot_mut(&mut self, slot_id: u32, inputs: &[*const u8], outputs: &[*mut u8]) {
                match slot_id {
                    #(#slot_mut_handlers),*
                    _ => panic!("Unhandled slot id {slot_id}")
                }
            }
            fn read_property(&self, prop_id: u32) -> #type_library::QVariant {
                match prop_id {
                    #(#prop_read_handlers),*
                    _ => panic!("Unhandled property id {prop_id}")
                }
            }
            fn write_property(&mut self, prop_id: u32, value: &#type_library::QVariant) {
                match prop_id {
                    #(#prop_write_handlers),*
                    _ => panic!("Unhandled property id {prop_id}")
                }
            }
        }
    };

    let mut result = syn::parse2(code)?;
    let mut map = TypeQualifiedMapping::new(CallOrigin::External);
    map.visit_item_impl_mut(&mut result);

    Ok(result)
}

fn get_slot_handler_code(slot: &QSlotInfo) -> syn::Result<TokenStream> {
    let id = slot.id();
    let invoke_code = slot.get_invoke_code()?;
    let code = quote! {
        #id => {
            #invoke_code
        }
    };
    Ok(code)
}
