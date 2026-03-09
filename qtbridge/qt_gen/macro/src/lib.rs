// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro::TokenStream;

use qt_gen_common::type_qualified_mapping::CallOrigin;

/// TODO: add documentation here.
#[proc_macro_attribute]
pub fn qobject(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut builder = qt_gen_impl::QObjectModuleBuilder::new(CallOrigin::External);
    let output = match builder.build_token_stream(input.into(), args.into()) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    };
    output.into()
}

#[doc(hidden)]
#[proc_macro_attribute]
pub fn qobject_internal(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut builder = qt_gen_impl::QObjectModuleBuilder::new(CallOrigin::Internal);
    let output = match builder.build_token_stream(input.into(), args.into()) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    };
    output.into()
}

/// Annotate a struct as a Qt object that can be accessed from QML.
///
/// This macro makes it possible to declare the following items within the `impl` block:
///
/// * [signals](#qsignal)
/// * [slots](#qslot)
/// * [properties](#qproperty)
/// * functions defining Qt item model behavior (discouraged API that will be replaced in a future release).
///
/// # Usage
///
/// The `#[qobject_impl]` macro must be applied to the `impl` block of the target `struct`.
/// The contents of the `impl` block may contain macro-like [annotations](#supported-annotations) that control how data and methods are exposed to QML.
///
/// Rust allows adding multiple `impl` blocks for the same `struct`. However, if you apply the `#[qobject_impl]` macro to one of them,
/// that block must contain all the signals/slots/properties declarations for that type.
///
/// ### Example
/// ```ignore
/// # use qtbridge::{QApp, qml_element, qobject_impl};
/// #
/// #[derive(Default)]
/// #[qml_element]
/// pub struct Counter {
///    value: i32,
/// }
///
/// #[qobject_impl]
/// impl Counter {
///     qproperty!("value", Member = value, Notify = "valueChanged");
///
///     #[qsignal]
///     fn value_changed(&self);
///
///     #[qslot]
///     fn change_value(&mut self, inc: bool) {
///         self.value = match inc {
///             true => self.value.saturating_add(1),
///             false => self.value.saturating_sub(1),
///         };
///         self.value_changed();
///     }
///
///     #[qslot]
///     fn reset_value(&mut self) {
///         if self.value != 0 {
///             self.value = 0;
///             self.value_changed();
///         }
///     }
/// }
///
/// impl Drop for Counter {
///     fn drop(&mut self) {
///         self.detach_qobject();
///     }
/// }
///
/// const QML_CODE: &str =
/// r#"
///     import QtQuick
///     import QtQuick.Controls
///     import QtQuick.Layouts
///
///     import counter
///
///     ApplicationWindow {
///         visible: true
///         title: qsTr("Counter QML app")
///
///         Counter {
///             id: counter
///         }
///
///         RowLayout {
///             anchors.centerIn: parent
///
///             Button {
///                 text: "-"
///                 onClicked: counter.changeValue(false)
///             }
///
///             Button {
///                 text: "Value:" + counter.value
///                 onClicked: counter.resetValue()
///             }
///
///             Button {
///                 text: "+"
///                 onClicked: counter.changeValue(true)
///             }
///         }
///     }
/// "#;
///
/// fn main() {
///     QApp::new()
///         .load_qml(QML_CODE.as_bytes())
///         .run();
/// }
/// ```
///
/// # Type Support
///
/// The set of types supported in signatures of functions marked as [signal](#qsignal), [slot](#qslot) or [qproperty!](#qproperty) accessors is limited.
/// The currently supported types are:
/// - **Scalar types**: `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`, `isize`, `usize`, `f32`, `f64`.
/// - **String types**: `String` and `&str`.
/// - **Collections**: `Vec<T>`, where `T` is one of the supported scalar types or `String`.
/// - **Qt-specific types**: `QModelIndex` (internal implementation type).
///
/// # Object creation / destruction
///
/// A `struct` annotated with `#[qobject_impl]` must currently implement `Default` trait.
///
/// If the struct needs to be instantiable from QML, either the `struct` or its `impl` block must be annotated
/// with [`#[qml_element]`](crate::qml_element). This allows instances of the type to be default-constructed directly from QML.
///
/// Another way to make an object available to QML is to create it on the Rust side and then expose it to QML.
/// There are two ways to construct a object in Rust to send it further to QML:
/// - For a `struct` implementing `Default` trait, the simplest approach is to call
/// `Self::default_with_attached_qobject()`.
/// The return type of this function is `Rc<RefCell<Self>>`.
/// - If the `struct` must be constructed in a non-default way, create an instance as `Rc<RefCell<Self>>`
/// and call `Self::attach_qobject()`.
///
/// Once properly created, the object instance must be passed to QML engine via `QApp::with_initial_properties()`,
/// as shown in the example below.
///
/// ```ignore
///
/// #[derive(Default)]
/// struct Backend {
///     // Some fields go here.
/// }
///
/// #[qobject_impl]
/// impl Backend {
/// }
///
/// impl Drop for Backend {
///     fn drop(&mut self) {
///         self.detach_qobject();
///     }
/// }
///
/// fn main() {
///     // Default construct the instance of the struct.
///     let default: Rc<RefCell<Backend>> = Backend::default_with_attached_qobject();
///
///     // Create your object manually as Rc<RefCell<T>>.
///     let custom = Rc::new(RefCell::new(Backend {
///         // Object initialization goes here.
///     }));
///     Backend::attach_qobject(&custom);
///
///     // Prepare the list of object to be exposed to QML engine as properties.
///     let properties = [
///         ("defaultObject", default.borrow().as_qvariant()),
///         ("customObject", custom.borrow().as_qvariant()),
///     ];
///
///     // Run the application
///     QApp::new()
///         .with_initial_properties(&properties)
///         .load_qml(include_bytes!("main.qml"))
///         .run();
/// }
/// ```
/// Proper destruction of a type constructed from the Rust side currently requires calling `self.detach_qobject()` from its [`Drop`] trait implementation (see the example above).
/// This requirement will be removed in a future release.
///
/// These methods mentioned in this section are generated for the target struct:
/// - `default_with_attached_qobject()`
/// - `attach_qobject()`
/// - `detach_qobject()`
///
#[proc_macro_attribute]
pub fn qobject_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let output = match qt_gen_impl::qobject_impl(input.into(), args.into(), &CallOrigin::External) {
        Ok(o) => o.to_token_stream(),
        Err(err) => err.to_compile_error(),
    };
    output.into()
}

#[doc(hidden)]
#[proc_macro_attribute]
pub fn qobject_impl_internal(args: TokenStream, input: TokenStream) -> TokenStream {
    let output = match qt_gen_impl::qobject_impl(input.into(), args.into(), &CallOrigin::Internal) {
        Ok(o) => o.to_token_stream(),
        Err(err) => err.to_compile_error(),
    };
    output.into()
}

/// Marks a function as a signal that can be handled in QML.
///
/// Signals can be called from Rust and the signal handler can be defined in QML. This is the
/// recommended way to invoke QML code from Rust.
///
/// ### Requirements
///
/// - The signal must be defined within a `#[qobject]` (mod) or `#[qobject_impl]` (impl) block
/// - The first argument of the annotated function must be `&self` or `&mut self`.
/// - The return type and all parameter types following `self` must be in the list of [supported types](#type-support).
/// - A function annotated with `#[qsignal]` must not have a body (end with semicolon or have an empty curly braces):
///
/// ```ignore
/// #[qobject_impl]
/// impl Backend {
///     #[qsignal]
///     fn value_changed(&self, new_value: i32);
///     #[qsignal]
///     fn event_triggered(&self)
///     {}
/// }
/// ```
///
/// To receive a notification on the QML side, the object definition has to declare a signal handler named
/// `on<Signal>`, where `<Signal>` is the name of the signal, with the first letter capitalized.
///
/// ```ignore, qml
/// Backend {
///     onValue_changed: console.log("Value changed");
/// }
/// ```
/// Alternatively you can instantiate a `Connection` object with the respective signal handler.
/// ```ignore, qml
/// Connection {
///     target: backend
///     function onValue_changed() {
///         console.log("Value changed");
///     }
/// }
/// ```
///
/// For more details see <https://doc.qt.io/qt-6/qtqml-syntax-signals.html>
///
/// ### Parameters
///
/// #### qml_name
///
/// The signal name as seen from QML. Defaults to the Rust function name.
///
/// ```ignore
/// #[qsignal(qml_name = "configurationChanged")]
/// fn cfg_changed();
/// ```
///
#[proc_macro_attribute]
pub fn qsignal(_: TokenStream, _: TokenStream) -> TokenStream {
    // This macro does nothing but offer an entry point for Rust doc
    panic!("#[qsignal] proc macro called outside #[qobject] or #[qobject_impl].")
}

/// Marks a method as invokable from QML.
///
/// In addition to being invokable from QML, the function can also act as a slot for
/// [signal-slot connections](#signals-and-slots) when used in Qt signal bindings.
///
/// ### Requirements
///
/// - Has to be defined within a `#[qobject]` (mod) or `#[qobject_impl]` (impl) block
/// - The annotated function must have a body.
/// - The first argument of the annotated function must be `&self` or `&mut self`.
/// - The return type and all parameter types following `self` must be in the list of [supported types](#type-support).
///
/// ### Example
/// ```ignore
/// #[qslot]
/// fn set_value(&mut self, new_value: i32) {
///     if new_value != self.value {
///         self.value = new_value;
///         self.update();
///     }
/// }
/// ```
///
/// ### Parameters
///
/// #### qml_name
///
/// The function name as seen from QML. Defaults to the Rust function name.
///
/// ```ignore
/// #[qslot(qml_name = "setValue")]
/// fn set_value(&mut self, new_value: i32) {
///    self.value = new_value;
///    self.update();
/// }
/// ```
///
#[proc_macro_attribute]
pub fn qslot(_: TokenStream, _: TokenStream) -> TokenStream {
    // This macro does nothing but offer an entry point for Rust doc
    panic!("#[qslot] proc macro called outside #[qobject] or #[qobject_impl].")
}

// The macro that registers a user-defined Rust `struct` type as a QML [object type](https://doc.qt.io/qt-6/qtqml-typesystem-objecttypes.html).
//
// Once a `struct` is registered in the QML type system, it can be imported and instantiated from QML like any other object type.
// In QML syntax, the initialization of an object can be done by specifying the type name followed by curly brackets that contain the object's properties initializers.
// For example, QML initialization can look like:
//
// ### Example
//
// ```javascript,ignore
// import QtQuick
// import QtQuick.Controls
// import address_book
//
// ApplicationWindow {
//     ...
//    property Address address: Address {
//         city: "London"
//         street: "Baker Street"
//         house: 221
//     }
//     ...
// }
// ```
//
// for the `struct` defined on the Rust side as follows:
//
// ```rust,ignore
// #[qml_element]
// struct Address {
//     city: String,
//     street: String,
//     house: i32,
// }
// #[qobject_impl]
// impl Address {
//     qproperty!("city", Member = city, Notify = "cityChanged");
//    qproperty!("street", Member = street, Notify = "streetChanged");
//     qproperty!("house", Member = house, Notify = "houseChanged");
//
//     #[qsignal]
//     fn city_changed(&self);
//     #[qsignal]
//     fn street_changed(&self);
//     #[qsignal]
//     fn house_changed(&self);
// }
// ```
//
// Currently, the name of the QML module that must be imported in QML code is derived
// from the `name` value in the `package` section of the corresponding manifest file ('Cargo.toml'),
// by applying the following transformations:
// - All leading digits are removed.
// - All characters that are neither alphabetic not digits are replaced with '_'.
//
// Once an instance is created in QML, its exposed method and properties can be accessed and manipulated directly from QML code.
//
//
// ### Singleton support
//
// Optionally, a QML element can be registered as a [singleton](https://doc.qt.io/qt-6/qml-singleton.html) by using `#[qml_element(singleton)]`.
// In this case, QML code accesses a single shared instance of the object directly, using the **type name itself** as identifier.
// This is useful for application-wide data, global settings, or service objects.
//
// Exmaple:
//
// ```rust
// use qtbridge::{qobject_impl, qml_element};
//
// #[derive(Default)]
// pub struct Backend {
// }
//
// #[qobject_impl]
// #[qml_element(singleton)]
// impl Backend {
//     #[qslot]
//     fn say_hello(&self) {
//         println!("Hello World!")
//     }
// }
// ```
//
// Then QML can refer to it by type name "Backend":
//
// ```javascript,ignore
// import QtQuick
// import QtQuick.Controls
// import hello_world
//
// ApplicationWindow {
//     visible: true
//
//     Button {
//         anchors.centerIn: parent
//         text: "Hello World!"
//         onClicked: Backend.sayHello()
//     }
// }
// ```
//
//
// #[proc_macro_attribute]
// pub fn qml_element(args: TokenStream, input: TokenStream) -> TokenStream {
//     let output = match qt_gen_impl::qml_element(args.into(), input.into()) {
//         Ok(tokens) => tokens,
//         Err(err) => err.to_compile_error(),
//     };
//     output.into()
// }
