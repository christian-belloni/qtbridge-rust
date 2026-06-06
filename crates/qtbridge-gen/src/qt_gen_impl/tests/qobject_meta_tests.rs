// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#![cfg(test)]
use insta::assert_snapshot;
use quote::{ToTokens, quote};
use crate::qt_gen_impl::QObjectModuleBuilder;
use crate::qt_gen_impl::qobject_impl::qobject_impl;
use qtbridge_gen_common::format_code::{format_rust_code, strip_docs};

#[test]
fn test() {
    let input = quote! {
        impl SomeStruct {

            qproperty!("this_value", Read = get_value, Write = set_value, Notify = this_value_changed, Default);
            qproperty!("otherValue", Member = otherValueVar, Notify = other_value_changed);
            qclass_info!(Name = "Author", Value = "The Qt Company");

            #[qsignal(qml_name = "thisValueChanged")]
            fn this_value_changed(&mut self, value: &String);

            #[qsignal]
            fn other_value_changed(&mut self, value: f32)
            {}

            #[qsignal]
            pub fn this_string_value_changed_by_ref(&mut self, value: &String);

            #[qsignal]
            pub fn that_string_value_changed_by_value(&mut self, value: String);

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

    let output = qobject_impl(input, quote!{})
        .unwrap()
        .qmeta_info_impl;
    let formatted = format_rust_code(&strip_docs(output.to_token_stream())).unwrap();
    assert_snapshot!(formatted);
}

#[test]
fn test_case_casting() {
    let input = quote! {
        impl SomeStruct {

            qproperty!("this_value", Read = get_value, Write = set_value, Notify = this_value_changed, Default);
            qproperty!("otherValue", Member = otherValueVar, Notify = other_value_changed);
            qclass_info!(Name = "Author", Value = "The Qt Company");

            #[qsignal(qml_name = "this_value_changed")]
            fn this_value_changed(&mut self, value: &String);

            #[qsignal]
            fn other_value_changed(&mut self, value: f32)
            {}

            #[qsignal]
            pub fn this_string_value_changed_by_ref(&mut self, value: &String);

            #[qsignal]
            pub fn that_string_value_changed_by_value(&mut self, value: String);

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

    let output = qobject_impl(input, quote!{ConvertToCamelCase})
        .unwrap()
        .qmeta_info_impl;
    let formatted = format_rust_code(&strip_docs(output.to_token_stream())).unwrap();
    assert_snapshot!(formatted);
}

#[test]
fn test_dispatch_meta_call() {
    let input = quote! {
        impl SomeStruct {
            qproperty!("this_value", Member = first_value, Notify = this_value_changed);
            qproperty!("otherValue", Member = second_value);
            qproperty!("thirdValue", Member = third_value);

            #[qslot]
            fn slot_one(&self) {
            }

            #[qslot]
            fn slot_two(&self, arg: String) {
            }

            #[qslot]
            fn slot_mut_three(&mut self) {
            }

            #[qslot]
            fn slot_mut_four(&mut self) {
            }

            #[qsignal]
            fn this_value_changed(&mut self) {
            }
        }
    };

    let output = qobject_impl(input, quote!{})
        .unwrap()
        .dispatch_meta_call;
    let formatted = format_rust_code(&strip_docs(output.to_token_stream())).unwrap();
    assert_snapshot!(formatted);
}

#[test]
fn test_nested_type_in_properties() {
    let input = quote! {
        pub mod node {
            pub struct Node {
                value: i32,
                left: Option<Rc<RefCell<Node>>>,
                right: Option<Rc<RefCell<Node>>>,
            }

            impl Node {
                qproperty!("value", Member = value, Write = set_value);
                qproperty!("left", Read = get_left, Write = set_left);
                qproperty!("right", Read = get_right, Write = set_right);

                pub fn set_value(&mut self, value: i32) {
                    self.value = value;
                }

                fn get_left(&self) -> Rc<RefCell<Self>> {
                    self.left
                        .as_ref()
                        .expect("Left node is not present")
                        .clone()
                }

                fn get_right(&self) -> &Rc<RefCell<Self>> {
                    self.right
                        .as_ref()
                        .expect("Right node is not present")
                }

                pub fn set_left(&mut self, value: &Rc<RefCell<Self>>) {
                    self.left = Some(value.clone());
                }

                pub fn set_right(&mut self, value: Rc<RefCell<Self>>) {
                    self.right = Some(value);
                }
            }
        }
    };

    let mut builder = QObjectModuleBuilder::new();
    let output = builder.build_token_stream(input, quote!{})
        .expect("build_token_stream() failed");
    let formatted = format_rust_code(&strip_docs(output)).unwrap();
    assert_snapshot!(formatted);
}

#[test]
fn test_nested_type_in_signals() {
    let input = quote! {
        pub mod node {
            pub struct Node {
            }
            impl Node {
                #[qsignal]
                pub fn signal_with_nested_type_in_argument_by_value(&mut self, value: Rc<RefCell<SomeNestedType>>);
                #[qsignal]
                pub fn signal_with_nested_type_in_argument_by_reference(&mut self, value: &Rc<RefCell<SomeNestedType>>);
            }
        }
    };

    let mut builder = QObjectModuleBuilder::new();
    let output = builder.build_token_stream(input, quote!{})
        .expect("build_token_stream() failed");
    let formatted = format_rust_code(&strip_docs(output)).unwrap();
    assert_snapshot!(formatted);
}

#[test]
fn test_nested_type_slots() {
    let input = quote! {
        pub mod node {
            pub struct Node {
                nested: Rc<RefCell<Node>>,
            }
            impl Node {
                #[qslot]
                pub fn slot_with_nested_type_argument_by_value(&mut self, value: Rc<RefCell<Node>>) {
                    self.nested = value;
                }
                #[qslot]
                pub fn slot_with_nested_type_argument_by_reference(&mut self, value: &Rc<RefCell<Node>>) {
                    self.nested = value.clone();
                }
            }
        }
    };

    let mut builder = QObjectModuleBuilder::new();
    let output = builder.build_token_stream(input, quote!{})
        .expect("build_token_stream() failed");
    let formatted = format_rust_code(&strip_docs(output)).unwrap();
    assert_snapshot!(formatted);
}
