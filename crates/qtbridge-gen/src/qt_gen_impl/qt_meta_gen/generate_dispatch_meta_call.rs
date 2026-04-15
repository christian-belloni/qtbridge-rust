// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
use quote::quote;
use syn::visit_mut::VisitMut;

use qtbridge_gen_common::type_qualified_mapping::{CallOrigin, TypeQualifiedMapping};

use crate::qt_gen_impl::qt_meta_gen::{QPropertyInfo, QSignalInfo, QSlotInfo, traits::find_by_qml_name};

pub fn generate_dispatch_meta_call(struct_ident: &syn::Ident, generics: &syn::Generics,
    signals: &[QSignalInfo], slots: &[QSlotInfo], properties: &[QPropertyInfo], origin: &CallOrigin) -> syn::Result<syn::ItemImpl> {

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let type_library = origin.type_module();
    let bridge_library = origin.bridge_module();

    let slot_handlers = slots.iter()
        .map(|slot| {
            let id = slot.id();
            let code = slot.get_invoke_code()?;
            Ok(quote! {
                #id => {
                    #code
                }
            })
        })
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
            let signal = prop.get_notify_signal()
                .and_then(|name_lit| find_by_qml_name(&name_lit.value(), signals));
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
            fn invoke_slot(&mut self, slot_id: u32, inputs: &[*const u8], outputs: &[*mut u8]) {
                let this = self;
                match slot_id {
                    #(#slot_handlers),*
                    _ => panic!("Unhandled slot id {slot_id}")
                }
            }
            fn read_property(&self, prop_id: u32) -> #type_library::QVariant {
                let this = self;
                match prop_id {
                    #(#prop_read_handlers),*
                    _ => panic!("Unhandled property id {prop_id}")
                }
            }
            fn write_property(&mut self, prop_id: u32, value: &#type_library::QVariant) {
                let this = self;
                match prop_id {
                    #(#prop_write_handlers),*
                    _ => panic!("Unhandled property id {prop_id}")
                }
            }
        }
    };

    let mut result = syn::parse2(code)?;
    let mut map = TypeQualifiedMapping::new(origin.clone());
    map.visit_item_impl_mut(&mut result);

    Ok(result)
}
