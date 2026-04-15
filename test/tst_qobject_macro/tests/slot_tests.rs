// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]
mod common;

use qtbridge::{QApp, QObjectHolder, qobject};
use common::capitalize_first_char;

#[qobject]
pub mod test_object {
    #[derive(Default)]
    pub struct TestObject {
        pub arg_unit: Option<()>,
        pub arg_bool: Option<bool>,
        pub arg_i8: Option<i8>,
        pub arg_u8: Option<u8>,
        pub arg_i16: Option<i16>,
        pub arg_u16: Option<u16>,
        pub arg_i32: Option<i32>,
        pub arg_u32: Option<u32>,
        pub arg_i64: Option<i64>,
        pub arg_u64: Option<u64>,
        pub arg_isize: Option<isize>,
        pub arg_usize: Option<usize>,
        pub arg_f32: Option<f32>,
        pub arg_f64: Option<f64>,
        pub arg_string: Option<String>,
    }

    impl TestObject {
        // Slots taking the argument by value
        #[qslot]
        fn slot_no_args(&mut self) {
            self.arg_unit = Some(());
        }
        #[qslot]
        fn slot_bool(&mut self, arg: bool) {
            self.arg_bool = Some(arg);
        }
        #[qslot]
        fn slot_i8(&mut self, arg: i8) {
            self.arg_i8 = Some(arg);
        }
        #[qslot]
        fn slot_u8(&mut self, arg: u8) {
            self.arg_u8 = Some(arg);
        }
        #[qslot]
        fn slot_i16(&mut self, arg: i16) {
            self.arg_i16 = Some(arg);
        }
        #[qslot]
        fn slot_u16(&mut self, arg: u16) {
            self.arg_u16 = Some(arg);
        }
        #[qslot]
        fn slot_i32(&mut self, arg: i32) {
            self.arg_i32 = Some(arg);
        }
        #[qslot]
        fn slot_u32(&mut self, arg: u32) {
            self.arg_u32 = Some(arg);
        }
        #[qslot]
        fn slot_i64(&mut self, arg: i64) {
            self.arg_i64 = Some(arg);
        }
        #[qslot]
        fn slot_u64(&mut self, arg: u64) {
            self.arg_u64 = Some(arg);
        }
        #[qslot]
        fn slot_isize(&mut self, arg: isize) {
            self.arg_isize = Some(arg);
        }
        #[qslot]
        fn slot_usize(&mut self, arg: usize) {
            self.arg_usize = Some(arg);
        }
        #[qslot]
        fn slot_f32(&mut self, arg: f32) {
            self.arg_f32 = Some(arg);
        }
        #[qslot]
        fn slot_f64(&mut self, arg: f64) {
            self.arg_f64 = Some(arg);
        }
        #[qslot]
        fn slot_str(&mut self, arg: &str) {
            self.arg_string = Some(arg.into());
        }
        #[qslot]
        fn slot_string(&mut self, arg: String) {
            self.arg_string = Some(arg);
        }

        // Slots taking the argument by reference
        #[qslot]
        fn slot_bool_ref(&mut self, arg: &bool) {
            self.arg_bool = Some(*arg);
        }
        #[qslot]
        fn slot_i8_ref(&mut self, arg: &i8) {
            self.arg_i8 = Some(*arg);
        }
        #[qslot]
        fn slot_u8_ref(&mut self, arg: &u8) {
            self.arg_u8 = Some(*arg);
        }
        #[qslot]
        fn slot_i16_ref(&mut self, arg: &i16) {
            self.arg_i16 = Some(*arg);
        }
        #[qslot]
        fn slot_u16_ref(&mut self, arg: &u16) {
            self.arg_u16 = Some(*arg);
        }
        #[qslot]
        fn slot_i32_ref(&mut self, arg: &i32) {
            self.arg_i32 = Some(*arg);
        }
        #[qslot]
        fn slot_u32_ref(&mut self, arg: &u32) {
            self.arg_u32 = Some(*arg);
        }
        #[qslot]
        fn slot_i64_ref(&mut self, arg: &i64) {
            self.arg_i64 = Some(*arg);
        }
        #[qslot]
        fn slot_u64_ref(&mut self, arg: &u64) {
            self.arg_u64 = Some(*arg);
        }
        #[qslot]
        fn slot_isize_ref(&mut self, arg: &isize) {
            self.arg_isize = Some(*arg);
        }
        #[qslot]
        fn slot_usize_ref(&mut self, arg: &usize) {
            self.arg_usize = Some(*arg);
        }
        #[qslot]
        fn slot_f32_ref(&mut self, arg: &f32) {
            self.arg_f32 = Some(*arg);
        }
        #[qslot]
        fn slot_f64_ref(&mut self, arg: &f64) {
            self.arg_f64 = Some(*arg);
        }
        #[qslot]
        fn slot_string_ref(&mut self, arg: &String) {
            self.arg_string = Some(arg.clone());
        }
    }
}

pub use test_object::TestObject;

fn get_qml_code_for(slot_type_suffix: &str, arg_value: &str) -> String {
    format!(r#"
        import QtQuick
        Item {{
            required property var testObject

            Component.onCompleted: {{
                testObject.slot{slot_type_suffix}({arg_value});
            }}
        }}
    "#)
}

fn test_type<F>(arg_type_suffix: &str, arg_value: &str, check_fn: F)
    where F: FnOnce(&TestObject) -> bool
{
    let suffix = capitalize_first_char(arg_type_suffix);

    // Run QApp with QML code for the given slot.
    let obj = TestObject::default_with_attached_qobject();
    let mut app = QApp::new();
    let obj_var = obj.borrow().as_qvariant();
    app.add_initial_property("testObject", &obj_var)
        .load_qml(get_qml_code_for(&suffix, arg_value).as_bytes());
    assert!(check_fn(&obj.borrow()), "failing slot type: {arg_type_suffix}");
}

fn test_slot_types_values() {
    test_type("NoArgs", "",              |obj| obj.arg_unit.is_some());
    test_type("bool", "true",            |obj| obj.arg_bool == Some(true));
    test_type("i8", "10",                |obj| obj.arg_i8 == Some(10));
    test_type("u8", "20",                |obj| obj.arg_u8 == Some(20));
    test_type("i16", "-30",              |obj| obj.arg_i16 == Some(-30));
    test_type("u16", "40",               |obj| obj.arg_u16 == Some(40));
    test_type("i32", "-50",              |obj| obj.arg_i32 == Some(-50));
    test_type("u32", "60",               |obj| obj.arg_u32 == Some(60));
    test_type("i64", "70",               |obj| obj.arg_i64 == Some(70));
    test_type("u64", "80",               |obj| obj.arg_u64 == Some(80));
    test_type("isize", "90",             |obj| obj.arg_isize == Some(90));
    test_type("usize", "95",             |obj| obj.arg_usize == Some(95));
    test_type("f32", "0.5",              |obj| obj.arg_f32 == Some(0.5));
    test_type("f64", "0.25",             |obj| obj.arg_f64 == Some(0.25));
    test_type("str", "\"test\"",         |obj| obj.arg_string == Some("test".into()));
    test_type("string", "\"test2\"",     |obj| obj.arg_string == Some("test2".into()));
}

fn test_slot_types_references() {
    test_type("boolRef", "true", |obj| obj.arg_bool == Some(true));
    test_type("i8Ref", "11",     |obj| obj.arg_i8 == Some(11));
    test_type("u8Ref", "21",     |obj| obj.arg_u8 == Some(21));
    test_type("i16Ref", "-31",   |obj| obj.arg_i16 == Some(-31));
    test_type("u16Ref", "41",    |obj| obj.arg_u16 == Some(41));
    test_type("i32Ref", "-51",   |obj| obj.arg_i32 == Some(-51));
    test_type("u32Ref", "61",    |obj| obj.arg_u32 == Some(61));
    test_type("i64Ref", "71",    |obj| obj.arg_i64 == Some(71));
    test_type("u64Ref", "81",    |obj| obj.arg_u64 == Some(81));
    test_type("isizeRef", "91",  |obj| obj.arg_isize == Some(91));
    test_type("usizeRef", "97",  |obj| obj.arg_usize == Some(97));
    test_type("f32Ref", "0.25",  |obj| obj.arg_f32 == Some(0.25));
    test_type("f64Ref", "0.75",  |obj| obj.arg_f64 == Some(0.75));

    test_type("stringRef", "\"test3\"",  |obj| obj.arg_string == Some("test3".into()));
}

fn main() {
    test_slot_types_values();
    test_slot_types_references();
}
