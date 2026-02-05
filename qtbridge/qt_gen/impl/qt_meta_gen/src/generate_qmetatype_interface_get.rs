// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::TokenStream;
use quote::{quote};

use qt_gen_common::type_qualified_mapping::CallOrigin;

pub fn generate_qmeta_type_interface_get(struct_ident: &syn::Ident, generics: &syn::Generics, origin: &CallOrigin) -> syn::Result<TokenStream> {
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let type_generics_turbofish = type_generics.as_turbofish();
    let type_library = origin.type_module();
    let bridge_library = origin.bridge_module();

    let code = quote! {
        impl #impl_generics #type_library::QMetaTypeInterfaceGet for #struct_ident #type_generics #where_clause {
            fn get_qmetatype_interface() -> &'static #type_library::QMetaTypeInterface {
                // TODO: generate simpler code (avoid using RefCell<HashMap<>>> in the non-generic case)
                use std::any::TypeId;
                use std::cell::RefCell;
                use std::collections::HashMap;
                use #type_library::{QMetaTypeFlag, QMetaTypeInterface};

                thread_local!(static IFACE_MAP: RefCell<HashMap<TypeId, *const QMetaTypeInterface>> =
                    RefCell::new(HashMap::new())
                );

                let type_id = TypeId::of::<#struct_ident #type_generics>();
                {
                    let iface_ptr = IFACE_MAP.with_borrow(|iface_map| {
                        iface_map.get(&type_id)
                            .copied()
                            .unwrap_or_default()
                    });
                    if let Some(iface_ref) = unsafe { iface_ptr.as_ref() } {
                        return iface_ref
                    }
                }

                let flags: u32 = (QMetaTypeFlag::NeedsConstruction as u32) |
                                 (QMetaTypeFlag::NeedsDestruction as u32) |
                                 (QMetaTypeFlag::NeedsCopyConstruction as u32) |
                                 (QMetaTypeFlag::NeedsMoveConstruction as u32) |
                                 (QMetaTypeFlag::PointerToQObject as u32);

                let class_name = std::ffi::CString::new(std::any::type_name::<#struct_ident #type_generics>())
                    .expect("CString::new failed")
                    .into_bytes_with_nul()
                    .leak();

                pub extern "C"
                fn meta_object_fn #impl_generics(_iface: *const #type_library::QMetaTypeInterface) -> *mut #type_library::QMetaObject
                #where_clause
                {
                    let meta_obj_data = <#struct_ident #type_generics as #bridge_library::QMetaInfo>::get_shared_dynamic_meta_object();
                    meta_obj_data.get_dynamic_qmetaobject().cast_mut()
                }

                pub extern "C"
                fn default_ctor #impl_generics (_iface: *const #type_library::QMetaTypeInterface, addr: *mut u8)
                #where_clause
                {
                    let instance = std::rc::Rc::new(std::cell::RefCell::new(<#struct_ident #type_generics_turbofish as Default>::default()));
                    <#struct_ident #type_generics_turbofish as #bridge_library::QObjectHolder>::register_instance_in_map_with_cpp_proxy_at(addr, instance);
                }

                pub extern "C"
                fn dtor(_iface: *const #type_library::QMetaTypeInterface, obj: *mut u8) {
                    #type_library::QObject::destruct(obj.cast());
                }

                let iface = #type_library::QMetaTypeInterface::fill_fields(
                    <Self as #bridge_library::QObjectHolder>::get_align_of_cpp_proxy(),
                    <Self as #bridge_library::QObjectHolder>::get_size_of_cpp_proxy(),
                    flags,
                    class_name,
                    meta_object_fn #type_generics_turbofish as usize,
                    default_ctor #type_generics_turbofish as usize,
                    dtor as usize,
                );

                let iface_ref = Box::leak(Box::new(iface));
                let iface_ptr = std::ptr::from_ref(iface_ref);
                IFACE_MAP.with_borrow_mut(|iface_map| iface_map.insert(type_id, iface_ptr));
                iface_ref
            }
        }
    };

    Ok(code)
}
