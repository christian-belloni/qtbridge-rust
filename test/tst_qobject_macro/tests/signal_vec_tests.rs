// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]
mod common;

use qtbridge::{QApp, QObjectHolder, qobject};
use common::{capitalize_first_char, get_type_name};

#[qobject]
pub mod test_object {
    #[derive(Default)]
    pub struct TestObject {
        pub signal_arg_vec_bool: Vec<bool>,
        pub signal_arg_vec_i8: Vec<i8>,
        pub signal_arg_vec_u8: Vec<u8>,
        pub signal_arg_vec_i16: Vec<i16>,
        pub signal_arg_vec_u16: Vec<u16>,
        pub signal_arg_vec_i32: Vec<i32>,
        pub signal_arg_vec_u32: Vec<u32>,
        pub signal_arg_vec_i64: Vec<i64>,
        pub signal_arg_vec_u64: Vec<u64>,
        pub signal_arg_vec_isize: Vec<isize>,
        pub signal_arg_vec_usize: Vec<usize>,
        pub signal_arg_vec_f32: Vec<f32>,
        pub signal_arg_vec_f64: Vec<f64>,
        pub signal_arg_vec_string: Vec<String>,
    }

    impl TestObject {
        #[qsignal]
        pub fn signal_vec_bool(&self, arg: &Vec<bool>);
        #[qsignal]
        pub fn signal_vec_i8(&self, arg: Vec<i8>);
        #[qsignal]
        pub fn signal_vec_u8(&self, arg: &Vec<u8>);
        #[qsignal]
        pub fn signal_vec_i16(&self, arg: &Vec<i16>);
        #[qsignal]
        pub fn signal_vec_u16(&self, arg: &Vec<u16>);
        #[qsignal]
        pub fn signal_vec_i32(&self, arg: &Vec<i32>);
        #[qsignal]
        pub fn signal_vec_u32(&self, arg: Vec<u32>);
        #[qsignal]
        pub fn signal_vec_i64(&self, arg: &Vec<i64>);
        #[qsignal]
        pub fn signal_vec_u64(&self, arg: &Vec<u64>);
        #[qsignal]
        pub fn signal_vec_isize(&self, arg: &Vec<isize>);
        #[qsignal]
        pub fn signal_vec_usize(&self, arg: &Vec<usize>);
        #[qsignal]
        pub fn signal_vec_f32(&self, arg: &Vec<f32>);
        #[qsignal]
        pub fn signal_vec_f64(&self, arg: &Vec<f64>);
        #[qsignal]
        pub fn signal_vec_string(&self, arg: Vec<String>);

        qproperty!("signalArgBool", Member = signal_arg_vec_bool);
        qproperty!("signalArgI8", Member = signal_arg_vec_i8);
        qproperty!("signalArgU8", Member = signal_arg_vec_u8);
        qproperty!("signalArgI16", Member = signal_arg_vec_i16);
        qproperty!("signalArgU16", Member = signal_arg_vec_u16);
        qproperty!("signalArgI32", Member = signal_arg_vec_i32);
        qproperty!("signalArgU32", Member = signal_arg_vec_u32);
        qproperty!("signalArgI64", Member = signal_arg_vec_i64);
        qproperty!("signalArgU64", Member = signal_arg_vec_u64);
        qproperty!("signalArgIsize", Member = signal_arg_vec_isize);
        qproperty!("signalArgUsize", Member = signal_arg_vec_usize);
        qproperty!("signalArgF32", Member = signal_arg_vec_f32);
        qproperty!("signalArgF64", Member = signal_arg_vec_f64);
        qproperty!("signalArgString", Member = signal_arg_vec_string);
    }
}

pub use test_object::TestObject;


fn get_qml_code_for(type_suffix: &str) -> String {
    format!(r#"
        import QtQuick
        Item {{
            required property var testObject

            Timer {{
                id: timer
                interval: 1
                property var signalValue
                onTriggered: {{
                    testObject.signalArg{type_suffix} = signalValue
                    Qt.quit()
                }}
            }}

            Connections {{
                target: testObject
                function onSignalVec{type_suffix}(v) {{
                    timer.signalValue = v
                    timer.start()
                }}
            }}
        }}
    "#)
}

fn test_type<T>(emit_fn: fn(&TestObject), check_fn: fn(&TestObject) -> bool)
{
    let type_str = get_type_name::<T>();
    let suffix = capitalize_first_char(&type_str);

    // Patch qml code.
    let qml = get_qml_code_for(&suffix);

    // Init QApp with QML code for the given signal.
    let obj = TestObject::default_with_attached_qobject();

    let mut app = QApp::new();
    let obj_var = obj.borrow().as_qvariant();
    app.add_initial_property("testObject", &obj_var)
       .load_qml(qml.as_bytes());

    // Emit the signal.
    emit_fn(&obj.borrow());

    // Handle delayed signal
    app.run();

    // Check that the corresponding property contains the value the signal was called with.
    assert!(check_fn(&obj.borrow()), "failing signal type: {type_str}");
}

fn test_that_signals_work_with_vec_arguments() {
    test_type::<bool>(
        |obj| obj.signal_vec_bool(&vec![true, true, false, true]),
        |obj| obj.signal_arg_vec_bool == [true, true, false, true]);
    test_type::<i8>(
        |obj| obj.signal_vec_i8(vec![0, 1, 2, i8::MAX, i8::MIN]),
        |obj| obj.signal_arg_vec_i8 == [0, 1, 2, i8::MAX, i8::MIN]);
    test_type::<i8>(
        |obj| obj.signal_vec_i8(vec![]),
        |obj| obj.signal_arg_vec_i8.is_empty());
    test_type::<u8>(
        |obj| obj.signal_vec_u8(&vec![0, 1, 2, 127, u8::MAX]),
        |obj| obj.signal_arg_vec_u8 == [0, 1, 2, 127, u8::MAX]);
    test_type::<i16>(
        |obj| obj.signal_vec_i16(&vec![0, 1, 2, 3, i16::MAX, i16::MIN]),
        |obj| obj.signal_arg_vec_i16 == [0, 1, 2, 3, i16::MAX, i16::MIN]);
    test_type::<u16>(
        |obj| obj.signal_vec_u16(&vec![0, 1, 2, 3, 32767, u16::MAX]),
        |obj| obj.signal_arg_vec_u16 == [0, 1, 2, 3, 32767, u16::MAX]);
    test_type::<i32>(
        |obj| obj.signal_vec_i32(&vec![0, 1, 2, 3, -1, i32::MIN, i32::MAX]),
        |obj| obj.signal_arg_vec_i32 == [0, 1, 2, 3, -1, i32::MIN, i32::MAX]);
    test_type::<u32>(
        |obj| obj.signal_vec_u32(vec![0, 1, 2, 3, u32::MAX]),
        |obj| obj.signal_arg_vec_u32 == [0, 1, 2, 3, u32::MAX]);
    test_type::<i64>(
        |obj| obj.signal_vec_i64(&vec![0, 1, 2, 3, -1, i64::MIN, i64::MAX]),
        |obj| obj.signal_arg_vec_i64 == [0, 1, 2, 3, -1, i64::MIN, i64::MAX]);
    test_type::<u64>(
        |obj| obj.signal_vec_u64(&vec![0, 1, 2, 3, u64::MAX]),
        |obj| obj.signal_arg_vec_u64 == [0, 1, 2, 3, u64::MAX]);
    test_type::<isize>(
        |obj| obj.signal_vec_isize(&vec![0, 1, 2, 3, -1, isize::MIN, isize::MAX]),
        |obj| obj.signal_arg_vec_isize == [0, 1, 2, 3, -1, isize::MIN, isize::MAX]);
    test_type::<usize>(
        |obj| obj.signal_vec_usize(&vec![0, 1, 2, 3, usize::MAX]),
        |obj| obj.signal_arg_vec_usize == [0, 1, 2, 3, usize::MAX]);
    test_type::<f32>(
        |obj| obj.signal_vec_f32(&vec![0.0, 0.5, -0.25, 0.125, f32::MIN, f32::MAX]),
        |obj| obj.signal_arg_vec_f32 == [0.0, 0.5, -0.25, 0.125, f32::MIN, f32::MAX]);
    test_type::<f64>(
        |obj| obj.signal_vec_f64(&vec![0.0, 0.5, 0.25, -0.125, f64::MIN, f64::MAX]),
        |obj| obj.signal_arg_vec_f64 == [0.0, 0.5, 0.25, -0.125, f64::MIN, f64::MAX]);
    test_type::<String>(
        |obj| obj.signal_vec_string(vec!["One".into(), "Two".into(), "Four".into()]),
        |obj| obj.signal_arg_vec_string == ["One", "Two", "Four"]);
}

fn main() {
    if cfg!(miri) {
        return;
    }
    test_that_signals_work_with_vec_arguments()
}
