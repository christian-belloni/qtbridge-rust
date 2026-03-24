// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#![cfg(test)]
use insta::assert_snapshot;
use quote::quote;
use crate::qt_gen_impl::qobject_impl::qobject_impl;
use qt_gen_common::type_qualified_mapping::CallOrigin;
use qt_gen_common::format_code::{format_rust_code, strip_docs};

#[test]
fn test() {
    let input = quote! {
        impl SomeStruct {

            qproperty!("this_value", Read = get_value, Write = set_value, Notify = "thisValueChanged", Default);
            qproperty!("otherValue", Member = otherValueVar, Notify = "otherValueChanged");
            qclass_info!(Name = "Author", Value = "The Qt Company");

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

    let output = qobject_impl(input, quote!{}, &CallOrigin::External)
        .unwrap()
        .qmeta_info_impl;
    let formatted = format_rust_code(&strip_docs(output)).unwrap();
    assert_snapshot!(formatted);
}
