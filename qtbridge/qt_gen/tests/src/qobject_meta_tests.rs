// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#![cfg(test)]
use quote::quote;
use qt_gen_impl::qobject_impl::qobject_impl;
use crate::tst_assert::assert_tokens_eq;
use qt_gen_common::type_qualified_mapping::CallOrigin;
use qt_gen_common::format_code::strip_docs;

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

    let output = qobject_impl(input, quote!{}, &CallOrigin::External)
        .unwrap()
        .qmeta_info_impl;

    /* Uncomment to update the baseline
    use std::fs;
    use std::path::Path;
    use qt_gen_common::format_code::format_rust_code;

    let baseline_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("test_baselines")
        .join("somestruct_meta.rs");

    fs::write(baseline_path,format_rust_code(&strip_docs(output.clone())).unwrap()).unwrap();
    */

    let baseline_src = include_str!("test_baselines/somestruct_meta.rs");
    let baseline: proc_macro2::TokenStream =
        baseline_src.parse().expect("baseline is not valid Rust");


    assert_tokens_eq(&strip_docs(output),
                   &strip_docs(baseline));
}
