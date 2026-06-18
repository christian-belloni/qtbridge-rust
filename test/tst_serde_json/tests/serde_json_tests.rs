// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]

use qtbridge::{QApp, QObjectHolder, qobject};

#[qobject]
pub mod json_backend {
    #[derive(Default)]
    pub struct JsonBackend {
        pub received_value: Option<serde_json::Value>,
        pub received_array: Option<Vec<serde_json::Value>>,
        pub json_value: serde_json::Value,
        pub json_array: Vec<serde_json::Value>,
    }

    impl JsonBackend {
        qproperty!("jsonValue", Member = json_value);
        qproperty!("jsonArray", Member = json_array);

        #[qslot]
        fn receive_value(&mut self, arg: serde_json::Value) {
            self.received_value = Some(arg);
        }

        #[qslot]
        fn receive_array(&mut self, arg: Vec<serde_json::Value>) {
            self.received_array = Some(arg);
        }
    }
}
pub use json_backend::JsonBackend;

fn slot_receives_js_object() {
    // JS numbers are always doubles; 3 arrives as Float(3.0) not PosInt(3).
    check_receive_value(r#"{"name": "test", "count": 3}"#, serde_json::json!({"name": "test", "count": 3.0}));
    check_receive_value(r#"{}"#, serde_json::json!({}));
    check_receive_value(r#"{"x": 1.5, "active": false, "label": ""}"#, serde_json::json!({"x": 1.5, "active": false, "label": ""}));
}

fn slot_receives_js_array_as_value() {
    check_receive_value(r#"["a", "b", "c"]"#, serde_json::json!(["a", "b", "c"]));
    check_receive_value(r#"[]"#, serde_json::json!([]));
    check_receive_value(r#"[true, 0, "mixed"]"#, serde_json::json!([true, 0.0, "mixed"]));
}

fn slot_receives_js_array_as_vec() {
    let obj = JsonBackend::default_with_attached_qobject();
    let var = obj.borrow().as_qvariant();

    QApp::new()
        .add_initial_property("backend", &var)
        .load_qml(
            r#"import QtQuick
               Item {
                   required property var backend
                   Component.onCompleted: backend.receive_array(["x", "y", "z"])
               }"#
            .as_bytes(),
        );

    let received = obj.borrow().received_array.clone().unwrap();
    assert_eq!(received.len(), 3);
    assert_eq!(received[0], serde_json::json!("x"));
    assert_eq!(received[2], serde_json::json!("z"));
}

fn slot_receives_nested_object() {
    check_receive_value(
        r#"({"user": {"name": "Alice", "active": true}, "tags": ["rust", "qt"]})"#,
        serde_json::json!({"user": {"name": "Alice", "active": true}, "tags": ["rust", "qt"]}),
    );
}

fn property_value_written_from_qml() {
    check_json_value_property(r#"{"key": "hello"}"#, serde_json::json!({"key": "hello"}));
    check_json_value_property(r#"{}"#, serde_json::json!({}));
    check_json_value_property(r#"[1, 2, 3]"#, serde_json::json!([1.0, 2.0, 3.0]));
    check_json_value_property("true", serde_json::Value::Bool(true));
    check_json_value_property("42", serde_json::json!(42.0));
    check_json_value_property("\"hello\"", serde_json::json!("hello"));
}

fn property_array_written_from_qml() {
    let obj = JsonBackend::default_with_attached_qobject();
    let var = obj.borrow().as_qvariant();

    QApp::new()
        .add_initial_property("backend", &var)
        .load_qml(
            r#"import QtQuick
               Item {
                   required property var backend
                   Component.onCompleted: backend.jsonArray = [10, 20, 30]
               }"#
            .as_bytes(),
        );

    let prop = obj.borrow().json_array.clone();
    assert_eq!(prop.len(), 3);
    // JS numbers arrive as Float
    assert_eq!(prop[0].as_f64(), Some(10.0));
    assert_eq!(prop[2].as_f64(), Some(30.0));
}

fn check_json_value_property(qml_value: &str, expected: serde_json::Value) {
    let obj = JsonBackend::default_with_attached_qobject();
    let var = obj.borrow().as_qvariant();
    QApp::new()
        .add_initial_property("backend", &var)
        .load_qml(
            format!(
                r#"import QtQuick
                   Item {{
                       required property var backend
                       Component.onCompleted: backend.jsonValue = {qml_value}
                   }}"#
            )
            .as_bytes(),
        );
    assert_eq!(obj.borrow().json_value.clone(), expected);
}

fn check_receive_value(qml_value: &str, expected: serde_json::Value) {
    let obj = JsonBackend::default_with_attached_qobject();
    let var = obj.borrow().as_qvariant();
    QApp::new()
        .add_initial_property("backend", &var)
        .load_qml(
            format!(
                r#"import QtQuick
                   Item {{
                       required property var backend
                       Component.onCompleted: backend.receive_value({qml_value})
                   }}"#
            )
            .as_bytes(),
        );
    assert_eq!(obj.borrow().received_value.clone().unwrap(), expected);
}

fn slot_receives_primitives() {
    check_receive_value("true", serde_json::Value::Bool(true));
    check_receive_value("false", serde_json::Value::Bool(false));
    check_receive_value("0", serde_json::json!(0.0));
    check_receive_value("42", serde_json::json!(42.0));
    check_receive_value("-7.5", serde_json::json!(-7.5));
    check_receive_value("1e6", serde_json::json!(1_000_000.0));
    check_receive_value("\"hello\"", serde_json::json!("hello"));
    check_receive_value("\"\"", serde_json::json!(""));
}

fn main() {
    if cfg!(miri) {
        return;
    }
    slot_receives_js_object();
    slot_receives_js_array_as_value();
    slot_receives_js_array_as_vec();
    slot_receives_nested_object();
    property_value_written_from_qml();
    property_array_written_from_qml();
    slot_receives_primitives();
}
