// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge::{include_bytes_qml, QApp};

#[test]
fn test_qresource() {
    use std::env;
    use std::path::PathBuf;

    include_bytes_qml!("example.txt");
    include_bytes_qml!("text/example.txt");
    include_bytes_qml!("example.txt", "samefolder");
    include_bytes_qml!("text/example.txt", "subfolder");
    include_bytes_qml!("example.txt", "samefolder/addedfolder");
    include_bytes_qml!("text/example.txt", "subfolder/addedfolder");

    let mut input_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("qml");
    let input_folder = input_path.to_string_lossy().to_string();

    println!("Running quick test with qml files in \"{}\"", &input_folder);

    let args: Vec<String> = vec![
        file!().to_string(),
        "-input".into(),
        input_folder,
    ];

    let result = quicktest::quick_test_main(&args, &"test_qresource".into());
    assert_eq!(result, 0, "quick_test failed with code {}", result);
}

fn main() {

    // Images from https://rustacean.net/ by Karen Rustad Tölva.
    // Public domain.
    include_bytes_qml!("rustacean-orig-noshadow.png", "data");
    include_bytes_qml!("rustacean-flat-happy.png", "data");
    include_bytes_qml!("rustacean-orig-noshadow.png", "data2");
    include_bytes_qml!("text/example.txt", "data");
    include_bytes_qml!("example.txt", "text/data");

    QApp::new()
        .load_qml(include_bytes!("main.qml"))
        .run();
}
