// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge::{QmlRegister, qobject_impl};

#[derive(Default)]
pub struct Backend {
}

#[qobject_impl(NoQmlElement)]
impl Backend {
    #[qslot]
    fn answer_to_everything(&self) -> i32 {
        42
    }
}

impl QmlRegister for Backend {
    const URI: &str = "tst_qml_element";
    const ELEMENT_NAME: &str = "Backend";
    const MINOR_VERSION: u8 = 1u8;
    const MAJOR_VERSION: u8 = 0u8;
    const IS_SINGLETON: bool = false;
}


#[derive(Default)]
pub struct SingletonBackend {
}

#[qobject_impl(NoQmlElement)]
impl SingletonBackend {
    #[qslot]
    fn answer_to_everything(&self) -> i32 {
        42
    }
}

impl QmlRegister for SingletonBackend {
    const URI: &str = "tst_qml_element";
    const ELEMENT_NAME: &str = "SingletonBackend";
    const MINOR_VERSION: u8 = 1u8;
    const MAJOR_VERSION: u8 = 0u8;
    const IS_SINGLETON: bool = true;
}

pub fn test_qml_element() {

    <Backend as QmlRegister>::register();
    <SingletonBackend as QmlRegister>::register();

    use std::env;
    use std::path::PathBuf;

    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    input_path.push("qml");
    let input_folder = input_path.to_str().unwrap().to_string();

    println!("Running quick test with qml files in \"{}\"", &input_folder);

    let args: Vec<String> = vec![
        file!().to_string(),
        "-input".to_string(),
        input_folder,
    ];

    use quicktest::quick_test_main;

    let result = quick_test_main(&args, &"test_qml_element".to_string());

    assert_eq!(result, 0, "quick_test failed with code {}", result);
}

