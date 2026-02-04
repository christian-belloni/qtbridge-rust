// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]
use qtbridge::qobject;
use qtbridge::QObjectHolder;

#[qobject]
pub mod test_object {
    #[derive(Default)]
    pub struct TestObject {
        pub value_bool: bool,
        pub value_i8: i8,
        pub value_u8: u8,
        pub value_i16: i16,
        pub value_u16: u16,
        pub value_i32: i32,
        pub value_u32: u32,
        pub value_i64: i64,
        pub value_u64: u64,
        pub value_f32: f32,
        pub value_f64: f64,
        pub value_string: String,
        pub value_string_list: Vec<String>,
    }

    impl TestObject {
        qproperty!("propertyBool", Member = value_bool);
        qproperty!("propertyI8", Member = value_i8);
        qproperty!("propertyU8", Member = value_u8);
        qproperty!("propertyI16", Member = value_i16);
        qproperty!("propertyU16", Member = value_u16);
        qproperty!("propertyI32", Member = value_i32);
        qproperty!("propertyU32", Member = value_u32);
        qproperty!("propertyI64", Member = value_i64);
        qproperty!("propertyU64", Member = value_u64);
        qproperty!("propertyF32", Member = value_f32);
        qproperty!("propertyF64", Member = value_f64);
        qproperty!("propertyString", Member = value_string, Write = set_string);
        qproperty!("propertyStringList", Read = get_string_list, Member = value_string_list);

        fn set_string(&mut self, value: &String) {
            self.value_string = value.into();
        }

        fn get_string_list(&self) -> &Vec<String> {
            &self.value_string_list
        }
    }
}

use test_object::TestObject;

// Tests checking that properties of certain types can be read via QObject::property()

#[test]
fn qproperty_of_type_bool_can_be_read() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().value_bool = true;
    let var = obj.borrow().get_qobject().property("propertyBool");
    let value: bool = var.try_into().unwrap();
    assert!(value);
}

#[test]
fn qproperty_of_type_i8_can_be_read() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().value_i8 = 10;
    let var = obj.borrow().get_qobject().property("propertyI8");
    let value: i8 = var.try_into().unwrap();
    assert_eq!(value, 10);
}

#[test]
fn qproperty_of_type_u8_can_be_read() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().value_u8 = 11;
    let var = obj.borrow().get_qobject().property("propertyU8");
    let value: u8 = var.try_into().unwrap();
    assert_eq!(value, 11);
}

#[test]
fn qproperty_of_type_i16_can_be_read() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().value_i16 = 12;
    let var = obj.borrow().get_qobject().property("propertyI16");
    let value: i16 = var.try_into().unwrap();
    assert_eq!(value, 12);
}

#[test]
fn qproperty_of_type_u16_can_be_read() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().value_u16 = 13;
    let var = obj.borrow().get_qobject().property("propertyU16");
    let value: u16 = var.try_into().unwrap();
    assert_eq!(value, 13);
}

#[test]
fn qproperty_of_type_i32_can_be_read() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().value_i32 = 14;
    let var = obj.borrow().get_qobject().property("propertyI32");
    let value: i32 = var.try_into().unwrap();
    assert_eq!(value, 14);
}

#[test]
fn qproperty_of_type_u32_can_be_read() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().value_u32 = 15;
    let var = obj.borrow().get_qobject().property("propertyU32");
    let value: u32 = var.try_into().unwrap();
    assert_eq!(value, 15);
}

#[test]
fn qproperty_of_type_i64_can_be_read() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().value_i64 = -16;
    let var = obj.borrow().get_qobject().property("propertyI64");
    let value: i64 = var.try_into().unwrap();
    assert_eq!(value, -16);
}

#[test]
fn qproperty_of_type_u64_can_be_read() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().value_u64 = 17;
    let var = obj.borrow().get_qobject().property("propertyU64");
    let value: u64 = var.try_into().unwrap();
    assert_eq!(value, 17);
}

#[test]
fn qproperty_of_type_f32_can_be_read() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().value_f32 = 0.5;
    let var = obj.borrow().get_qobject().property("propertyF32");
    let value: f32 = var.try_into().unwrap();
    assert_eq!(value, 0.5);
}

#[test]
fn qproperty_of_type_f64_can_be_read() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().value_f64 = 0.25;
    let var = obj.borrow().get_qobject().property("propertyF64");
    let value: f64 = var.try_into().unwrap();
    assert_eq!(value, 0.25);
}

#[test]
fn qproperty_of_type_string_can_be_read() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().value_string = "こんにちは".into();
    let var = obj.borrow().get_qobject().property("propertyString");
    let value: String = var.try_into().unwrap();
    assert_eq!(value, "こんにちは");
}

#[test]
fn qproperty_of_type_vec_of_strings_can_be_read() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().value_string_list = vec!["jeden".into(), "dva".into(), "tri".into()];
    let var = obj.borrow().get_qobject().property("propertyStringList");
    let value: Vec<String> = var.try_into().unwrap();
    assert_eq!(value, ["jeden", "dva", "tri"]);
}


// Tests checking that properties of certain types can be written using QObject::setProperty().

#[test]
fn qproperty_of_type_bool_can_be_written() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().get_qobject().set_property("propertyBool", true.into());
    assert!(obj.borrow().value_bool);
}

#[test]
fn qproperty_of_type_i8_can_be_written() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().get_qobject().set_property("propertyI8", 11.into());
    assert_eq!(obj.borrow().value_i8, 11);
}

#[test]
fn qproperty_of_type_u8_can_be_written() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().get_qobject().set_property("propertyU8", 12.into());
    assert_eq!(obj.borrow().value_u8, 12);
}

#[test]
fn qproperty_of_type_i16_can_be_written() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().get_qobject().set_property("propertyI16", 13.into());
    assert_eq!(obj.borrow().value_i16, 13);
}

#[test]
fn qproperty_of_type_u16_can_be_written() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().get_qobject().set_property("propertyU16", 14.into());
    assert_eq!(obj.borrow().value_u16, 14);
}

#[test]
fn qproperty_of_type_i32_can_be_written() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().get_qobject().set_property("propertyI32", 15.into());
    assert_eq!(obj.borrow().value_i32, 15);
}

#[test]
fn qproperty_of_type_u32_can_be_written() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().get_qobject().set_property("propertyU32", 16.into());
    assert_eq!(obj.borrow().value_u32, 16);
}

#[test]
fn qproperty_of_type_i64_can_be_written() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().get_qobject().set_property("propertyI64", 17.into());
    assert_eq!(obj.borrow().value_i64, 17);
}

#[test]
fn qproperty_of_type_u64_can_be_written() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().get_qobject().set_property("propertyU64", 18.into());
    assert_eq!(obj.borrow().value_u64, 18);
}

#[test]
fn qproperty_of_type_string_can_be_written() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().get_qobject().set_property("propertyString", "Привіт, світе!".into());
    assert_eq!(obj.borrow().value_string, "Привіт, світе!");
}

#[test]
fn qproperty_of_type_vec_of_strings_can_be_written() {
    let obj = TestObject::default_with_attached_qobject();
    obj.borrow_mut().get_qobject().set_property("propertyStringList", vec!["Como".to_owned(), "vai".to_owned(), "você".to_owned()].into());
    assert_eq!(obj.borrow().value_string_list, ["Como", "vai", "você"]);
}
