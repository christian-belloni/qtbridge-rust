// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
use quote::quote;

use qtbridge_gen_common::type_qualified_mapping::CallOrigin;

pub fn generate_dispatch_meta_call(struct_ident: &syn::Ident, generics: &syn::Generics, origin: &CallOrigin) -> syn::Result<syn::ItemImpl> {
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let type_library = origin.type_module();
    let bridge_library = origin.bridge_module();

    let code = quote! {
        impl #impl_generics #bridge_library::DispatchMetaCall for #struct_ident #type_generics
        #where_clause
        {
            fn invoke_slot(&mut self, slot_id: u32, inputs: &[*const u8], outputs: &[*mut u8]) {
                unimplemented!()
            }
            fn read_property(&self, prop_id: u32) -> #type_library::QVariant {
                unimplemented!()
            }
            fn write_property(&mut self, prop_id: u32, value: &#type_library::QVariant) {
                unimplemented!()
            }
        }
    };
    syn::parse2(code)
}
