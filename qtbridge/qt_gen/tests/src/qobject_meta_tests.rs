// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#![cfg(test)]
use quote::quote;
use qt_gen_impl::qobject_impl::qobject_impl;
use crate::tst_assert::assert_tokens_eq;
use qt_gen_common::type_qualified_mapping::CallOrigin;

#[test]
fn require_that_qobject_impl_macro_handles_signals_slots_and_properties() {
    let input = quote! {
        impl SomeStruct {

            qproperty!("this_value", Read = get_value, Write = set_value, Notify = "thisValueChanged");
            qproperty!("otherValue", Member = otherValueVar, Notify = "otherValueChanged");
            qclass_info!(Name = "DefaultProperty", Value = "this_value");

            #[qsignal(qml_name = "thisValueChanged")]
            fn this_value_changed(&self, value: &String);

            #[qsignal]
            fn other_value_changed(&self, value: f32)
            {}

            #[qsignal]
            pub fn this_string_value_changed_by_ref(&self, value: &String);

            #[qsignal]
            pub fn that_string_value_changed_by_value(&self, value: String);

            #[qslot(qml_name = "onThatValueChanged")]
            pub fn on_that_value_changed(&mut self, value: &String) {
            }

            pub fn just_struct_method(&mut self, some_arg: f32) {
                do_something_important();
            }

            pub fn just_struct_associated_function(some_arg: u64) {
                do_something_hacky_here();
            }

            pub fn get_value(&self) -> String {
                return self.value;
            }

            pub fn set_value(&mut self, v: &String) {
                self.value = v;
            }
        }
    };

    let expected = quote! {
        impl qtbridge::bridge::QMetaInfo for SomeStruct {
        fn class_name() -> &'static str {
            ::std::any::type_name::<SomeStruct>()
        }
        fn get_static_meta_object() -> &'static qtbridge::qt_type_lib::QMetaObject {
            some_struct_impl_details::ProxyRust::get_static_meta_object()
        }
        fn register_meta(
            mut meta_obj: std::pin::Pin<&mut qtbridge::bridge::DynamicMetaObjectData_Rust>,
        ) {
            use qt_type_lib::get_meta_type_id_of_fn_return_value;
            use qt_type_lib::QMetaTypeGet;
            use qt_type_lib::{QMetaType, QMetaTypeId};
            use qtbridge::bridge::metacallbacks::{
                property_read_callback_for, property_write_callback_for, slot_callback_for,
            };
            use qtbridge::qt_type_lib;
            meta_obj.as_mut()
                .register_signal("thisValueChanged", &[qt_type_lib::QString::get_qmetatype()]);
            meta_obj.as_mut()
                .register_signal("otherValueChanged", &[f32::get_qmetatype()]);
            meta_obj.as_mut()
                .register_signal("thisStringValueChangedByRef", &[qt_type_lib::QString::get_qmetatype()]);
            meta_obj.as_mut()
                .register_signal("thatStringValueChangedByValue", &[qt_type_lib::QString::get_qmetatype()]);
            meta_obj.as_mut().register_slot(
                "onThatValueChanged",
                &[qt_type_lib::QString::get_qmetatype()],
                slot_callback_for::<SomeStruct>(|this, params| {
                    let arg_0 = params.get_String(0usize);
                    this.on_that_value_changed(&arg_0);
                }),
            );
            meta_obj.as_mut().register_property(
                "this_value",
                &QMetaType::new(QMetaTypeId::QString as i32),
                property_read_callback_for::<SomeStruct>(|this| this.get_value().into()),
                property_write_callback_for::<SomeStruct>(|this, value| {
                    let Ok(value) = TryInto::try_into(value) else {
                        panic!(
                            "Failed to convert value '{}' to type '{}' in qproperty '{}'",
                            value.to_string(),
                            "String",
                            "this_value"
                        );
                    };
                    this.set_value(&value);
                }),
                "thisValueChanged",
            );
            meta_obj.as_mut().register_property(
                "otherValue",
                &QMetaType::new(
                    get_meta_type_id_of_fn_return_value(|this: &Self| &this.otherValueVar) as i32,
                ),
                property_read_callback_for::<SomeStruct>(|this| (&this.otherValueVar).into()),
                property_write_callback_for::<SomeStruct>(|this, value| {
                    let Ok(value) = value.try_into() else {
                        panic!(
                            "Failed to convert value '{}' to type '{}' in qproperty '{}'",
                            value.to_string(),
                            std::any::type_name_of_val(&this.otherValueVar),
                            "otherValue"
                        );
                    };
                    if this.otherValueVar != value {
                        this.otherValueVar = value;
                        this.other_value_changed(this.otherValueVar.clone());
                    }
                }),
                "otherValueChanged",
            );
            meta_obj
                .as_mut()
                .add_class_info("DefaultProperty", "this_value");
            meta_obj.as_mut().end_meta_registration();
        }
        fn get_shared_dynamic_meta_object_data() -> &'static qtbridge::bridge::DynamicMetaObjectData_Rust {
            use std::any::TypeId;
            use std::cell::RefCell;
            use std::collections::HashMap;
            thread_local ! (static DYNAMIC_META_MAP : RefCell < HashMap < TypeId , * const qtbridge :: bridge :: DynamicMetaObjectData_Rust >> = RefCell :: new (HashMap :: new ()));
            let type_id = TypeId::of::<SomeStruct>();
            {
                let meta_data_ptr = DYNAMIC_META_MAP.with_borrow(|dynamic_meta_data_map| {
                    dynamic_meta_data_map
                        .get(&type_id)
                        .copied()
                        .unwrap_or_default()
                });
                if let Some(meta_data_ref) = unsafe { meta_data_ptr.as_ref() } {
                    return meta_data_ref;
                }
            }
            let meta_data_ptr = qtbridge::bridge::create_dynamic_meta_object_data_for_type::<SomeStruct>();
            let meta_data_ref = unsafe { meta_data_ptr.as_ref() }.unwrap();
            DYNAMIC_META_MAP.with_borrow_mut(|dynamic_meta_data_map| {
                dynamic_meta_data_map.insert(type_id, meta_data_ptr);
            });
            meta_data_ref
        }
        fn get_list_meta_type() -> qtbridge::qt_type_lib::QMetaType {
            some_struct_impl_details::ProxyRust::get_qmetatype_list_of_cpp_proxy()
        }
    }

    };

    let output = qobject_impl(input, quote!{}, &CallOrigin::External)
        .unwrap()
        .qmeta_info_impl;
    assert_tokens_eq(&output, &expected);
}
