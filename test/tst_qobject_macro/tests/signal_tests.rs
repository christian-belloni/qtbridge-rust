// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]
use qtbridge::qt_type_lib::QSignalSpy;
use qtbridge::qobject;
use qtbridge::QObjectHolder;

#[qobject]
pub mod test_object {
    #[derive(Default)]
    pub struct TestObject {
    }

    impl TestObject {
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
        #[qsignal]
        pub fn signal_string_ref(&self, arg: &String);
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

#[test]
fn signal_is_emitted_when_called_with_bool_arg() {
    let obj = TestObject::default_with_attached_qobject();
    let mut spy = QSignalSpy::new(obj.borrow().get_qobject(), "signalBool");
    obj.borrow().signal_bool(true);
    assert_eq!(spy.count(), 1);
    let args = spy.pin_mut().take_first();
    let arg: bool = args.first().try_into().unwrap();
    assert!(arg);
}

#[test]
fn signal_is_emitted_when_called_with_i8_arg() {
    let obj = TestObject::default_with_attached_qobject();
    let mut spy = QSignalSpy::new(obj.borrow().get_qobject(), "signalI8");
    obj.borrow().signal_i8(42);
    assert_eq!(spy.count(), 1);
    let args = spy.pin_mut().take_first();
    let arg: i8 = args.first().try_into().unwrap();
    assert_eq!(arg, 42);
}

#[test]
fn signal_is_emitted_when_called_with_u8_arg() {
    let obj = TestObject::default_with_attached_qobject();
    let mut spy = QSignalSpy::new(obj.borrow().get_qobject(), "signalU8");
    obj.borrow().signal_u8(43);
    assert_eq!(spy.count(), 1);
    let args = spy.pin_mut().take_first();
    let arg: u8 = args.first().try_into().unwrap();
    assert_eq!(arg, 43);
}

#[test]
fn signal_is_emitted_when_called_with_i16_arg() {
    let obj = TestObject::default_with_attached_qobject();
    let mut spy = QSignalSpy::new(obj.borrow().get_qobject(), "signalI16");
    obj.borrow().signal_i16(-44);
    assert_eq!(spy.count(), 1);
    let args = spy.pin_mut().take_first();
    let arg: i16 = args.first().try_into().unwrap();
    assert_eq!(arg, -44);
}

#[test]
fn signal_is_emitted_when_called_with_u16_arg() {
    let obj = TestObject::default_with_attached_qobject();
    let mut spy = QSignalSpy::new(obj.borrow().get_qobject(), "signalU16");
    obj.borrow().signal_u16(45);
    assert_eq!(spy.count(), 1);
    let args = spy.pin_mut().take_first();
    let arg: u16 = args.first().try_into().unwrap();
    assert_eq!(arg, 45);
}

#[test]
fn signal_is_emitted_when_called_with_i32_arg() {
    let obj = TestObject::default_with_attached_qobject();
    let mut spy = QSignalSpy::new(obj.borrow().get_qobject(), "signalI32");
    obj.borrow().signal_i32(46);
    assert_eq!(spy.count(), 1);
    let args = spy.pin_mut().take_first();
    let arg: i32 = args.first().try_into().unwrap();
    assert_eq!(arg, 46);
}

#[test]
fn signal_is_emitted_when_called_with_u32_arg() {
    let obj = TestObject::default_with_attached_qobject();
    let mut spy = QSignalSpy::new(obj.borrow().get_qobject(), "signalU32");
    obj.borrow().signal_u32(47);
    assert_eq!(spy.count(), 1);
    let args = spy.pin_mut().take_first();
    let arg: u32 = args.first().try_into().unwrap();
    assert_eq!(arg, 47);
}

#[test]
fn signal_is_emitted_when_called_with_i64_arg() {
    let obj = TestObject::default_with_attached_qobject();
    let mut spy = QSignalSpy::new(obj.borrow().get_qobject(), "signalI64");
    obj.borrow().signal_i64(48);
    assert_eq!(spy.count(), 1);
    let args = spy.pin_mut().take_first();
    let arg: i64 = args.first().try_into().unwrap();
    assert_eq!(arg, 48);
}

#[test]
fn signal_is_emitted_when_called_with_u64_arg() {
    let obj = TestObject::default_with_attached_qobject();
    let mut spy = QSignalSpy::new(obj.borrow().get_qobject(), "signalU64");
    obj.borrow().signal_u64(49);
    assert_eq!(spy.count(), 1);
    let args = spy.pin_mut().take_first();
    let arg: u64 = args.first().try_into().unwrap();
    assert_eq!(arg, 49);
}

#[test]
fn signal_is_emitted_when_called_with_f32_arg() {
    let obj = TestObject::default_with_attached_qobject();
    let mut spy = QSignalSpy::new(obj.borrow().get_qobject(), "signalF32");
    obj.borrow().signal_f32(0.5);
    assert_eq!(spy.count(), 1);
    let args = spy.pin_mut().take_first();
    let arg: f32 = args.first().try_into().unwrap();
    assert_eq!(arg, 0.5);
}

#[test]
fn signal_is_emitted_when_called_with_f64_arg() {
    let obj = TestObject::default_with_attached_qobject();
    let mut spy = QSignalSpy::new(obj.borrow().get_qobject(), "signalF64");
    obj.borrow().signal_f64(0.25);
    assert_eq!(spy.count(), 1);
    let args = spy.pin_mut().take_first();
    let arg: f64 = args.first().try_into().unwrap();
    assert_eq!(arg, 0.25);
}

#[test]
fn signal_is_emitted_when_called_with_str_arg() {
    let obj = TestObject::default_with_attached_qobject();
    let mut spy = QSignalSpy::new(obj.borrow().get_qobject(), "signalStr");
    obj.borrow().signal_str("XYZ");
    assert_eq!(spy.count(), 1);
    let args = spy.pin_mut().take_first();
    let arg: String = args.first().try_into().unwrap();
    assert_eq!(arg, "XYZ");
}

#[test]
fn signal_is_emitted_when_called_with_string_arg() {
    let obj = TestObject::default_with_attached_qobject();
    let mut spy = QSignalSpy::new(obj.borrow().get_qobject(), "signalString");
    obj.borrow().signal_string(String::from("ABC"));
    assert_eq!(spy.count(), 1);
    let args = spy.pin_mut().take_first();
    let arg: String = args.first().try_into().unwrap();
    assert_eq!(arg, "ABC");
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

