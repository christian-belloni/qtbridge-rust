// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]
use qtbridge::{QApp, qobject};

#[qobject]
pub mod test_object {
    #[derive(Default)]
    pub struct TestObject {
        pub arg_vec_bool: Option<Vec<bool>>,
        pub arg_vec_i8: Option<Vec<i8>>,
        pub arg_vec_u8: Option<Vec<u8>>,
        pub arg_vec_i16: Option<Vec<i16>>,
        pub arg_vec_u16: Option<Vec<u16>>,
        pub arg_vec_i32: Option<Vec<i32>>,
        pub arg_vec_u32: Option<Vec<u32>>,
        pub arg_vec_i64: Option<Vec<i64>>,
        pub arg_vec_u64: Option<Vec<u64>>,
        pub arg_vec_f32: Option<Vec<f32>>,
        pub arg_vec_f64: Option<Vec<f64>>,
        pub arg_vec_string: Option<Vec<String>>,
    }

    impl TestObject {
        // Slots taking the argument by value
        #[qslot]
        fn slot_vec_bool(&mut self, arg: Vec<bool>) {
            self.arg_vec_bool = Some(arg);
        }
        #[qslot]
        fn slot_vec_i8(&mut self, arg: Vec<i8>) {
            self.arg_vec_i8 = Some(arg);
        }
        #[qslot]
        fn slot_vec_u8(&mut self, arg: Vec<u8>) {
            self.arg_vec_u8 = Some(arg);
        }
        #[qslot]
        fn slot_vec_i16(&mut self, arg: Vec<i16>) {
            self.arg_vec_i16 = Some(arg);
        }
        #[qslot]
        fn slot_vec_u16(&mut self, arg: Vec<u16>) {
            self.arg_vec_u16 = Some(arg);
        }
        #[qslot]
        fn slot_vec_i32(&mut self, arg: Vec<i32>) {
            self.arg_vec_i32 = Some(arg);
        }
        #[qslot]
        fn slot_vec_u32(&mut self, arg: Vec<u32>) {
            self.arg_vec_u32 = Some(arg);
        }
        #[qslot]
        fn slot_vec_i64(&mut self, arg: Vec<i64>) {
            self.arg_vec_i64 = Some(arg);
        }
        #[qslot]
        fn slot_vec_u64(&mut self, arg: Vec<u64>) {
            self.arg_vec_u64 = Some(arg);
        }
        #[qslot]
        fn slot_vec_f32(&mut self, arg: Vec<f32>) {
            self.arg_vec_f32 = Some(arg);
        }
        #[qslot]
        fn slot_vec_f64(&mut self, arg: Vec<f64>) {
            self.arg_vec_f64 = Some(arg);
        }
        #[qslot]
        fn slot_vec_string(&mut self, arg: Vec<String>) {
            self.arg_vec_string = Some(arg);
        }

        // Slots taking the argument by reference
        #[qslot]
        fn slot_vec_bool_ref(&mut self, arg: &Vec<bool>) {
            self.arg_vec_bool = Some(arg.clone());
        }
        #[qslot]
        fn slot_vec_i8_ref(&mut self, arg: &Vec<i8>) {
            self.arg_vec_i8 = Some(arg.clone());
        }
        #[qslot]
        fn slot_vec_u8_ref(&mut self, arg: &Vec<u8>) {
            self.arg_vec_u8 = Some(arg.clone());
        }
        #[qslot]
        fn slot_vec_i16_ref(&mut self, arg: &Vec<i16>) {
            self.arg_vec_i16 = Some(arg.clone());
        }
        #[qslot]
        fn slot_vec_u16_ref(&mut self, arg: &Vec<u16>) {
            self.arg_vec_u16 = Some(arg.clone());
        }
        #[qslot]
        fn slot_vec_i32_ref(&mut self, arg: &Vec<i32>) {
            self.arg_vec_i32 = Some(arg.clone());
        }
        #[qslot]
        fn slot_vec_u32_ref(&mut self, arg: &Vec<u32>) {
            self.arg_vec_u32 = Some(arg.clone());
        }
        #[qslot]
        fn slot_vec_i64_ref(&mut self, arg: &Vec<i64>) {
            self.arg_vec_i64 = Some(arg.clone());
        }
        #[qslot]
        fn slot_vec_u64_ref(&mut self, arg: &Vec<u64>) {
            self.arg_vec_u64 = Some(arg.clone());
        }
        #[qslot]
        fn slot_vec_f32_ref(&mut self, arg: &Vec<f32>) {
            self.arg_vec_f32 = Some(arg.clone());
        }
        #[qslot]
        fn slot_vec_f64_ref(&mut self, arg: &Vec<f64>) {
            self.arg_vec_f64 = Some(arg.clone());
        }
        #[qslot]
        fn slot_vec_string_ref(&mut self, arg: &Vec<String>) {
            self.arg_vec_string = Some(arg.clone());
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
                testObject.slotVec{slot_type_suffix}({arg_value});
            }}
        }}
    "#)
}

fn test_type<F>(arg_type_suffix: &str, arg_value: &str, check_fn: F)
    where F: FnOnce(&TestObject) -> bool
{
    // Capitalize the first char of `arg_type_suffix`.
    let mut chars = arg_type_suffix.chars();
    let suffix = format!("{}{}", chars.next().unwrap().to_uppercase(), chars.as_str());
    // Run QApp with QML code for the given slot.
    let obj = TestObject::default_with_attached_qobject();
    let mut app = QApp::new();
    app.add_initial_property("testObject", &obj.borrow().as_qvariant())
        .load_qml(get_qml_code_for(&suffix, arg_value).as_bytes());
    assert!(check_fn(&obj.borrow()), "failing slot type: {arg_type_suffix}");
}

fn test_slot_types_vec_values() {
    test_type("bool", "[true, false, true]",               |obj| obj.arg_vec_bool == Some(vec![true, false, true]));
    test_type("i8", "[0, 1, 2, -128, 127]",                |obj| obj.arg_vec_i8 == Some(vec![0, 1, 2, -128, 127]));
    test_type("u8", "[0, 1, 2, 128, 255]",                 |obj| obj.arg_vec_u8 == Some(vec![0, 1, 2, 128, 255]));
    test_type("i16", "[0, 1, 2, -32768, 32767]",           |obj| obj.arg_vec_i16 == Some(vec![0, 1, 2, -32768, 32767]));
    test_type("u16", "[0, 1, 2, 32767, 65535]",            |obj| obj.arg_vec_u16 == Some(vec![0, 1, 2, 32767, 65535]));
    test_type("i32", "[0, 1, 2, -2147483648, 2147483647]", |obj| obj.arg_vec_i32 == Some(vec![0, 1, 2, -2147483648, 2147483647]));
    test_type("u32", "[0, 1, 2, 2147483648, 4294967295]",  |obj| obj.arg_vec_u32 == Some(vec![0, 1, 2, 2147483648, 4294967295]));
    test_type("i64", "[0, 1, 2, Number.MIN_SAFE_INTEGER, Number.MAX_SAFE_INTEGER]", |obj| obj.arg_vec_i64 == Some(vec![0, 1, 2, -9007199254740991, 9007199254740991]));
    test_type("u64", "[0, 1, 2, Number.MAX_SAFE_INTEGER]", |obj| obj.arg_vec_u64 == Some(vec![0, 1, 2, 9007199254740991]));
    test_type("string", "[\"abc\", \"def\", \"ghi\"]",     |obj| obj.arg_vec_string == Some(vec!["abc".into(), "def".into(), "ghi".into()]));
}

fn test_slot_types_vec_references() {
    test_type("boolRef", "[true, false, true]",               |obj| obj.arg_vec_bool == Some(vec![true, false, true]));
    test_type("i8Ref", "[0, 1, 2, -128, 127]",                |obj| obj.arg_vec_i8 == Some(vec![0, 1, 2, -128, 127]));
    test_type("u8Ref", "[0, 1, 2, 128, 255]",                 |obj| obj.arg_vec_u8 == Some(vec![0, 1, 2, 128, 255]));
    test_type("i16Ref", "[0, 1, 2, -32768, 32767]",           |obj| obj.arg_vec_i16 == Some(vec![0, 1, 2, -32768, 32767]));
    test_type("u16Ref", "[0, 1, 2, 32767, 65535]",            |obj| obj.arg_vec_u16 == Some(vec![0, 1, 2, 32767, 65535]));
    test_type("i32Ref", "[0, 1, 2, -2147483648, 2147483647]", |obj| obj.arg_vec_i32 == Some(vec![0, 1, 2, -2147483648, 2147483647]));
    test_type("u32Ref", "[0, 1, 2, 2147483648, 4294967295]",  |obj| obj.arg_vec_u32 == Some(vec![0, 1, 2, 2147483648, 4294967295]));
    test_type("i64Ref", "[0, 1, 2, Number.MIN_SAFE_INTEGER, Number.MAX_SAFE_INTEGER]", |obj| obj.arg_vec_i64 == Some(vec![0, 1, 2, -9007199254740991, 9007199254740991]));
    test_type("u64Ref", "[0, 1, 2, Number.MAX_SAFE_INTEGER]", |obj| obj.arg_vec_u64 == Some(vec![0, 1, 2, 9007199254740991]));
    test_type("stringRef", "[\"abc\", \"def\", \"ghi\"]",     |obj| obj.arg_vec_string == Some(vec!["abc".into(), "def".into(), "ghi".into()]));
}

fn main() {
    test_slot_types_vec_values();
    test_slot_types_vec_references();
}
