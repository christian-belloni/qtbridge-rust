// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]
mod common;

use qtbridge::qt_type_lib::{QSignalSpy, QVariant};
use qtbridge::{qobject, QObjectHolder};
use common::{MAX_SAFE_INTEGER, MIN_SAFE_INTEGER};

#[qobject]
pub mod test_object {
    #[derive(Default)]
    pub struct TestObject {
    }

    impl TestObject {
        // Pass the signal argument by value.
        #[qsignal]
        pub fn signal_no_args(&self);
        #[qsignal]
        pub fn signal_bool(&self, arg: bool);
        #[qsignal]
        pub fn signal_i8(&self, arg: i8);
        #[qsignal]
        pub fn signal_u8(&self, arg: u8);
        #[qsignal]
        pub fn signal_i16(&self, arg: i16);
        #[qsignal]
        pub fn signal_u16(&self, arg: u16);
        #[qsignal]
        pub fn signal_i32(&self, arg: i32);
        #[qsignal]
        pub fn signal_u32(&self, arg: u32);
        #[qsignal]
        pub fn signal_i64(&self, arg: i64);
        #[qsignal]
        pub fn signal_u64(&self, arg: u64);
        #[qsignal]
        pub fn signal_f32(&self, arg: f32);
        #[qsignal]
        pub fn signal_f64(&self, arg: f64);
        #[qsignal]
        pub fn signal_str(&self, arg: &str);
        #[qsignal]
        pub fn signal_string(&self, arg: String);

        // Pass the signal argument by reference.
        #[qsignal]
        pub fn signal_bool_ref(&self, arg: &bool);
        #[qsignal]
        pub fn signal_i8_ref(&self, arg: &i8);
        #[qsignal]
        pub fn signal_u8_ref(&self, arg: &u8);
        #[qsignal]
        pub fn signal_i16_ref(&self, arg: &i16);
        #[qsignal]
        pub fn signal_u16_ref(&self, arg: &u16);
        #[qsignal]
        pub fn signal_i32_ref(&self, arg: &i32);
        #[qsignal]
        pub fn signal_u32_ref(&self, arg: &u32);
        #[qsignal]
        pub fn signal_i64_ref(&self, arg: &i64);
        #[qsignal]
        pub fn signal_u64_ref(&self, arg: &u64);
        #[qsignal]
        pub fn signal_f32_ref(&self, arg: &f32);
        #[qsignal]
        pub fn signal_f64_ref(&self, arg: &f64);
        #[qsignal]
        pub fn signal_string_ref(&self, arg: &String);

        // Test with multiple arguments
        #[qsignal]
        pub fn signal_many_args(&self,
            arg1: &str,
            arg2: i32,
            arg3: f32,
            arg4: &f64,
            arg5: String,
            arg6: u16,
            arg7: bool,
            arg8: &bool,
            arg9: i32,
            arg10: i64,
        );

    }
}

pub use test_object::TestObject;

// Tests that verify that signals are emitted and detected by QSignalSpy when invoked from the Rust side.

#[test]
fn signal_is_emitted_when_called_without_arguments() {
    let obj = TestObject::default_with_attached_qobject();
    let spy = QSignalSpy::new(obj.borrow().get_qobject(), "signalNoArgs");
    obj.borrow().signal_no_args();
    assert_eq!(spy.count(), 1);
}

fn test_signal_with_arg_value<EmitFn, CheckFn>(type_suffix: &str, emit_fn: EmitFn, check_fn: CheckFn)
where
    EmitFn: FnOnce(&TestObject),
    CheckFn: FnOnce(&QVariant) -> bool
{
    let obj = TestObject::default_with_attached_qobject();
    let mut spy = QSignalSpy::new(obj.borrow().get_qobject(), &format!("signal{type_suffix}"));
    emit_fn(&obj.borrow());
    assert_eq!(spy.count(), 1);
    let args = spy.pin_mut().take_first();
    assert!(check_fn(args.first()));
}

#[test]
fn signal_is_emitted_when_called_with_bool_arg() {
    test_signal_with_arg_value("Bool",
        |obj| obj.signal_bool(true),
        |var| {
            let value: bool = var.try_into().unwrap();
            value
        });
}

#[test]
fn signal_is_emitted_when_called_with_i8_arg() {
    test_signal_with_arg_value("I8",
        |obj| obj.signal_i8(42),
        |var| {
            let arg: i8 = var.try_into().unwrap();
            arg == 42
        });
}

#[test]
fn signal_is_emitted_when_called_with_u8_arg() {
    test_signal_with_arg_value("U8",
        |obj| obj.signal_u8(43),
        |var| {
            let arg: u8 = var.try_into().unwrap();
            arg == 43
        });
}

#[test]
fn signal_is_emitted_when_called_with_i16_arg() {
    test_signal_with_arg_value("I16",
        |obj| obj.signal_i16(-44),
        |var| {
            let arg: i16 = var.try_into().unwrap();
            arg == -44
        });
}

#[test]
fn signal_is_emitted_when_called_with_u16_arg() {
    test_signal_with_arg_value("U16",
        |obj| obj.signal_u16(45),
        |var| {
            let arg: u16 = var.try_into().unwrap();
            arg == 45
        });
}

#[test]
fn signal_is_emitted_when_called_with_i32_arg() {
    test_signal_with_arg_value("I32",
        |obj| obj.signal_i32(46),
        |var| {
            let arg: i32 = var.try_into().unwrap();
            arg == 46
        });
}

#[test]
fn signal_is_emitted_when_called_with_u32_arg() {
    test_signal_with_arg_value("U32",
        |obj| obj.signal_u32(47),
        |var| {
            let arg: u32 = var.try_into().unwrap();
            arg == 47
        });

}

#[test]
fn signal_is_emitted_when_called_with_i64_arg() {
    test_signal_with_arg_value("I64",
        |obj| obj.signal_i64(48),
        |var| {
            let arg: i64 = var.try_into().unwrap();
            arg == 48
        });
}

#[test]
fn signal_is_emitted_when_called_with_u64_arg() {
    test_signal_with_arg_value("U64",
        |obj| obj.signal_u64(49),
        |var| {
            let arg: u64 = var.try_into().unwrap();
            arg == 49
        });
}

#[test]
fn signal_is_emitted_when_called_with_f32_arg() {
    test_signal_with_arg_value("F32",
        |obj| obj.signal_f32(0.5),
        |var| {
            let arg: f32 = var.try_into().unwrap();
            arg == 0.5
        });
}

#[test]
fn signal_is_emitted_when_called_with_f64_arg() {
    test_signal_with_arg_value("F64",
        |obj| obj.signal_f64(0.25),
        |var| {
            let arg: f64 = var.try_into().unwrap();
            arg == 0.25
        });
}

#[test]
fn signal_is_emitted_when_called_with_str_arg() {
    test_signal_with_arg_value("Str",
        |obj| obj.signal_str("XYZ"),
        |var| {
            let arg: String = var.try_into().unwrap();
            arg == "XYZ"
        });
}

#[test]
fn signal_is_emitted_when_called_with_string_arg() {
    test_signal_with_arg_value("String",
        |obj| obj.signal_string("ABC".to_owned()),
        |var| {
            let arg: String = var.try_into().unwrap();
            arg == "ABC"
        });
}

#[test]
fn signal_is_emitted_when_called_with_string_ref_arg() {
    let obj = TestObject::default_with_attached_qobject();
    let mut spy = QSignalSpy::new(obj.borrow().get_qobject(), "signalStringRef");
    obj.borrow().signal_string_ref(&String::from("DEF"));
    assert_eq!(spy.count(), 1);
    let args = spy.pin_mut().take_first();
    let arg: String = args.first().try_into().unwrap();
    assert_eq!(arg, "DEF");
}

#[test]
fn signal_is_emitted_when_called_with_bool_ref_arg() {
    test_signal_with_arg_value("BoolRef",
        |obj| obj.signal_bool_ref(&true),
        |var| {
            let value: bool = var.try_into().unwrap();
            value
        });
}

#[test]
fn signal_is_emitted_when_called_with_i8_ref_arg() {
    test_signal_with_arg_value("I8Ref",
        |obj| obj.signal_i8_ref(&i8::MIN),
        |var| {
            let value: i8 = var.try_into().unwrap();
            value == i8::MIN
        });
}

#[test]
fn signal_is_emitted_when_called_with_u8_ref_arg() {
    test_signal_with_arg_value("U8Ref",
        |obj| obj.signal_u8_ref(&u8::MAX),
        |var| {
            let value: u8 = var.try_into().unwrap();
            value == u8::MAX
        });
}

#[test]
fn signal_is_emitted_when_called_with_i16_ref_arg() {
    test_signal_with_arg_value("I16Ref",
        |obj| obj.signal_i16_ref(&i16::MIN),
        |var| {
            let value: i16 = var.try_into().unwrap();
            value == i16::MIN
        });
}

#[test]
fn signal_is_emitted_when_called_with_u16_ref_arg() {
    test_signal_with_arg_value("U16Ref",
        |obj| obj.signal_u16_ref(&u16::MIN),
        |var| {
            let value: u16 = var.try_into().unwrap();
            value == u16::MIN
        });
}

#[test]
fn signal_is_emitted_when_called_with_i32_ref_arg() {
    test_signal_with_arg_value("I32Ref",
        |obj| obj.signal_i32_ref(&i32::MIN),
        |var| {
            let value: i32 = var.try_into().unwrap();
            value == i32::MIN
        });
}

#[test]
fn signal_is_emitted_when_called_with_u32_ref_arg() {
    test_signal_with_arg_value("U32Ref",
        |obj| obj.signal_u32_ref(&u32::MAX),
        |var| {
            let value: u32 = var.try_into().unwrap();
            value == u32::MAX
        });}


#[test]
fn signal_is_emitted_when_called_with_i64_ref_arg() {
    test_signal_with_arg_value("I64Ref",
        |obj| obj.signal_i64_ref(&MIN_SAFE_INTEGER),
        |var| {
            let value: i64 = var.try_into().unwrap();
            value == MIN_SAFE_INTEGER
        });
}

#[test]
fn signal_is_emitted_when_called_with_u64_ref_arg() {
    test_signal_with_arg_value("U64Ref",
        |obj| obj.signal_u64_ref(&MAX_SAFE_INTEGER),
        |var| {
            let value: u64 = var.try_into().unwrap();
            value == MAX_SAFE_INTEGER
        });
}

#[test]
fn signal_is_emitted_when_called_with_a_few_arguments_ref_arg() {
    let obj = TestObject::default_with_attached_qobject();
    let mut spy = QSignalSpy::new(obj.borrow().get_qobject(), "signalManyArgs");
    obj.borrow().signal_many_args("123", 700, 0.75, &0.125, "Café".to_owned(), 65535, false, &true, -100000, -10000000000);
    assert_eq!(spy.count(), 1);
    let arg_list = spy.pin_mut().take_first();
    assert_eq!(arg_list.len(), 10);
    assert_eq!(Ok("123".into()), TryInto::<String>::try_into(&arg_list[0]));
    assert_eq!(Ok(700), TryInto::<i32>::try_into(&arg_list[1]));
    assert_eq!(Ok(0.75), TryInto::<f32>::try_into(&arg_list[2]));
    assert_eq!(Ok(0.125), TryInto::<f64>::try_into(&arg_list[3]));
    assert_eq!(Ok("Café".into()), TryInto::<String>::try_into(&arg_list[4]));
    assert_eq!(Ok(65535), TryInto::<u16>::try_into(&arg_list[5]));
    assert_eq!(Ok(false), TryInto::<bool>::try_into(&arg_list[6]));
    assert_eq!(Ok(true), TryInto::<bool>::try_into(&arg_list[7]));
    assert_eq!(Ok(-100000), TryInto::<i32>::try_into(&arg_list[8]));
    assert_eq!(Ok(-10000000000), TryInto::<i64>::try_into(&arg_list[9]));
}
