// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use cxx::UniquePtr;
use qt_type_lib::{QGuiApplication, QQmlApplicationEngine, QVariant};

/// Runs a minimal application from an embedded QML file.
///
/// This macro is a convenience helper for small applications and examples.
/// It embeds the given QML file at compile time using [`include_bytes!`],
/// creates a [`QApp`] instance, loads the QML source, and starts the Qt
/// event loop.
///
/// The macro is intended to be used directly in `main` and requires no
/// additional setup beyond a valid QML file.
///
/// # Parameters
///
/// * `$path` – Path to a QML file, relative to the source file.
///
/// # Example
///
/// ```
///# use bridge::run_simple_app;
/// fn main() {
///     run_simple_app!("qml/main.qml");
/// }
/// ```
///
/// # Notes
///
/// * The QML file is embedded into the binary at compile time.
/// * This macro does not provide access to the [`QApp`] instance and is
///   therefore best suited for simple applications.
/// * For more advanced use cases, create and configure a [`QApp`] manually.
#[macro_export]
macro_rules! run_simple_app {
    ($path:expr) => {{
        let main_qml_bytes = include_bytes!($path);
        $crate::QApp::new()
            .load_qml(main_qml_bytes)
            .run();
    }};
}

/// This struct represents a Qt QML application and acts as the entry point for all applications.
/// QApp allows running QML code and injection Rust objects into its context. Aside from the
/// initialization of the backend logic this should be the only code in your main function.
///
/// # Example
///
/// A minimal “Hello World” application without a Rust backend:
///
/// ```rust
///# use bridge::QApp;
/// let prop = 42;
///
/// QApp::new()
///     .load_qml(br#"
///         import QtQuick
///         import QtQuick.Controls
///         Text {
///             text: "Hello Rust!"
///#            Component.onCompleted: closeTimer.start()
///#            Timer {
///#                id: closeTimer
///#                interval: 1
///#                onTriggered: Qt.quit()
///#            }
///         }"#)
///     .run();
/// ```
pub struct QApp {
    engine: UniquePtr<QQmlApplicationEngine>, // engine must be first field so its dropped before app
    #[allow(dead_code)]
    app: UniquePtr<QGuiApplication>,
}

impl QApp {
    /// Creates a new application instance.
    ///
    /// The application object must be created before any QML or GUI-related
    /// functionality is used.
    pub fn new() -> Self {
        let app = QGuiApplication::new();
        let engine = QQmlApplicationEngine::new();
        Self { app, engine }
    }

    /// Enters the Qt main event loop.
    ///
    /// This function blocks until the application exits and returns
    /// the exit code provided by Qt. This function does not return a reference to
    /// self and is usually the last call in a `main` function.
    ///
    /// # Returns
    ///
    /// The application exit code.
    pub fn run(&mut self) -> i32 {
        QGuiApplication::exec()
    }

    /// Sets the initial properties of the root object in the QML application.
    ///
    /// This method passes a list of [`str`]-[`QVariant`] pairs to the engine
    /// and you can read and write the value in your QML code.
    ///
    /// # Returns
    ///
    /// A mutable reference to `self`, allowing method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    ///# use bridge::QApp;
    /// let prop = 42;
    ///
    /// QApp::new()
    ///     .with_initial_properties(&[
    ///         ("answer", prop.into()),
    ///     ])
    ///     .load_qml(br#"
    ///         import QtQuick
    ///         import QtQuick.Controls
    ///         ApplicationWindow {
    ///             required property var answer
    ///#            Component.onCompleted: closeTimer.start()
    ///#            Timer {
    ///#                id: closeTimer
    ///#                interval: 1
    ///#                onTriggered: Qt.quit()
    ///#            }
    ///         }"#)
    ///     .run();
    /// ```
    /// # Returns
    ///
    /// A mutable reference to `self`, allowing method chaining.
    pub fn with_initial_properties(&mut self, properties: &[(&str, QVariant)]) -> &mut Self {
        self.engine.pin_mut().set_initial_properties(&properties.into());
        self
    }

    /// Loads the main QML source code from an in-memory byte slice.
    ///
    /// This method loads the given QML source into the application's
    /// QML engine. It is typically used to initialize the UI before
    /// entering the Qt event loop.
    ///
    /// # Parameters
    ///
    /// * `code` – A byte slice containing the QML source.
    ///
    /// # Returns
    ///
    /// A mutable reference to `self`, allowing method chaining.
    pub fn load_qml(&mut self, code: &[u8]) -> &mut Self {
        self.engine.pin_mut().load_data(code);
        self
    }

    /// Sets the application name.
    ///
    /// # Parameters
    ///
    /// * `name` – The application name.
    ///
    /// # Returns
    ///
    /// A mutable reference to `self`, allowing method chaining.
    pub fn application_name(&mut self, name: &str) -> &mut Self {
        QGuiApplication::set_application_name(name);
        self
    }
}
