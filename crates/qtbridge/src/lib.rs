// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#![doc = include_str!("../README.md")]

pub mod type_support {
    //! This module lists the types supported in qtbridge.
    //!
    //! ## Supported Types
    //!
    //! The following types are supported in [`qsignal`](crate::qsignal), [`qslot`](crate::qslot) or [`qproperty`](crate::qproperty):
    //! - **Scalar types**: [`i8`], [`u8`], [`i16`], [`u16`], [`i32`], [`u32`], [`i64`], [`u64`], [`isize`], [`usize`], [`f32`], [`f64`].
    //! - **String types**: [`String`] and [`&str`].
    //! - **Collections**: [`Vec<T>`], where `T` is one of the supported scalar types or [`String`].
}

#[doc(hidden)]
pub use qtbridge_runtime;
pub use qtbridge_runtime::QModelItem;
pub use qtbridge_runtime::invoke_method;
#[doc(hidden)]
pub use qtbridge_gen;
#[doc(hidden)]
pub use qtbridge_interfaces;
#[doc(hidden)]
pub use qtbridge_type_lib;
#[doc(hidden)]
pub use qtbridge_build_common;

/// Annotate an `impl` block to make its struct accessible from QML.
///
/// The macro implements a range of traits that are enable bridging from Rust
/// to QML. The mechanism is based on the implementation of various traits with
/// some code generated at macro expandsion time. You should not implement these
/// traits yourself. As a user, you should only interact with:
///
/// * [`QObjectHolder`]
/// * [`QmlRegister`] (only non-generic types)
///
/// This macro makes it possible to declare the following items within the
/// `impl` block:
///
/// * signals with the [`qsignal`] attribute macro
/// * invokable functions with the [`qslot`] attribute macro
/// * struct properties with the [`qproperty`] macro
///
/// Further, it allows the struct to implement traits to fulfill specific QML purposes.
/// These are called `Base` traits. The available base traits are:
///
/// * [`QListModel`] to make a `struct` accessible by QML ListView, QML Repeater, or similar.
/// * [`QTableModel`] to make a `struct` accessible by QML TableView.
///
/// Only one of those traits can be implemented at the same time.
///
/// # Usage
///
/// The [`qobject_impl`] macro must be applied to a `impl` block of the
/// target `struct`. Only a single `impl` block can be annotated with this macro and
/// all applications of [`qsignal`], [`qslot`] and [`qproperty`] have to be limited to
/// this block.
///
/// In order to communicate with QML, the macro creates briding objects that are attached
/// to the respective structs. Therefore, objects created with [`qobject_impl`] should be
/// created with [`default_with_attached_qobject`](QObjectHolder::default_with_attached_qobject)
/// or expanded with [`attach_qobject`](QObjectHolder::attach_qobject). This is not necessary if
/// the struct is instantiated in QML.
///
/// The macro creates, when [`register`](QmlRegister::register) is called a QML module with name
/// matching your Cargo package name. So for a Cargo.toml with
/// ```toml
/// [package]
///  name = "hello_world"
///
/// [dependencies]
/// qtbridge
/// ```
/// the QML file has to contain
///
/// ```toml
/// import hello_world
/// ```
///
/// ## Requirements
///
/// A `struct` annotated with [`qobject_impl`] must implement the [`Default`] trait.
/// The static function [`register`](QmlRegister::register) has to be called at the start of the
/// main function to make this `struct` instantiable from QML.
/// The macro implements [`Drop`] to call [`detach_qobject`](QObjectHolder::detach_qobject),
/// cleaning up the QML parts of the object. You can implement [`Drop`] yourself when using the
/// `NoDrop` option (see below). In that case, [`detach_qobject`](QObjectHolder::detach_qobject)
/// has to be called manually.
///
/// ## Parameters
///
/// Parameters to adjust the macro behaviour are passed as comma-separated keywords or keyword-value pairs.
///
/// **Base = BaseTrait**
///
/// Set the 'base' trait. Requires that the specified trait is implemented for the corresponding `struct`.
///
/// **NoQmlElement**
///
/// Do not implement [`QmlRegister`]. [`QmlRegister`] registers the `struct` in the QML type system,
/// allowing you to instantiate this type in QML. The `NoQmlElement` option can be useful to turn off
/// instantiatability within QML or to provide a manual implementation of this trait with better control
/// over naming and versioning.
///
/// **Singleton**
///
/// Implement [`QmlRegister`] as a [singleton](https://doc.qt.io/qt-6/qml-singleton.html). A singleton
/// is accessed from QML as a single shared instance of the type, using the type name as identifier.
/// This is useful for application-wide data, global settings, or service objects.
///
/// **NoDrop**
///
/// Do not implement [`Drop`] in the macro. This option has to be set when a custom [`Drop`]
/// implementation is required. The function [`attach_qobject`](QObjectHolder::attach_qobject)
/// has to be called manually to avoid memory leaks.
///
/// **LinkMe**
///
/// This option calls [`register`](QmlRegister::register) at application without any additional code.
/// The crate [`Linkme`](https://crates.io/crates/linkme) is used for this purpose and needs to be
/// added to Cargo.toml.
///
/// ## Example
///
/// ```
/// use qtbridge::{QApp, qobject_impl};
///
/// #[derive(Default)]
/// pub struct Counter {
///    value: i32,
/// }
///
/// #[qobject_impl(Singleton)]
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
/// }
///
/// const QML_CODE: &str =
/// r#"
///     import QtQuick
///     import QtQuick.Controls
///     import QtQuick.Layouts
///     import qtbridge // must match your cargo package name
///
///     ApplicationWindow {
///         visible: true
///         title: qsTr("Counter QML app")
/// #       Component.onCompleted: closeTimer.start()
/// #       Timer {
/// #           id: closeTimer
/// #           interval: 1
/// #           onTriggered: Qt.quit()
/// #       }
///         RowLayout {
///             anchors.centerIn: parent
///             Button {
///                 text: "-"
///                 onClicked: Counter.changeValue(false)
///             }
///             Button {
///                 text: "+"
///                 onClicked: Counter.changeValue(true)
///             }
///         }
///     }
/// "#;
///
/// fn main() {
///     QApp::new()
///         .register::<Counter>()
///         .load_qml(QML_CODE.as_bytes())
///         .run();
/// }
/// ```
///
#[doc(inline)]
pub use qtbridge_gen::qobject_impl;

/// Annotate a `mod` block to make its struct accessible from QML.
///
/// The mod block must contain a single `struct` and it's  `impl` blocks. The
/// impl blocks are treated as if they had the [`qobject_impl`] annotation.
///
/// Similar to [`qobject_impl`], this macro implements the following traits:
///
/// * [`QObjectHolder`]
/// * [`QmlRegister`] (only non-generic types)
///
/// and a QML module that fits the package name of your Cargo.toml.
///
/// This macro has the same parameters as [`qobject_impl`] and behaves the same way.
/// In contrast to [`qobject_impl`], this macro tries to identify an existing
/// [`Drop`] implementation and will inject [`detach_qobject`](QObjectHolder::detach_qobject)
/// when found. If no [`Drop`] implementation is found, the macro will generate one.
/// To surpess this injection, you can use the `NoDrop` option.
///
/// ## Example
///
/// ```
/// use qtbridge::{QApp, qobject};
///
/// #[qobject(Singleton)]
/// pub mod backend {
///     #[derive(Default)]
///     pub struct Counter {
///        value: i32,
///     }
///
///     impl Counter {
///         qproperty!("value", Member = value, Notify = "valueChanged");
///
///         #[qsignal]
///         fn value_changed(&self);
///
///         #[qslot]
///         fn change_value(&mut self, inc: bool) {
///             self.value = match inc {
///                 true => self.value.saturating_add(1),
///                 false => self.value.saturating_sub(1),
///             };
///             self.value_changed();
///         }
///     }
/// }
///
/// const QML_CODE: &str =
/// r#"
///     import QtQuick
///     import QtQuick.Controls
///     import QtQuick.Layouts
///     import qtbridge // must match your cargo package name
///
///     ApplicationWindow {
///         visible: true
///         title: qsTr("Counter QML app")
/// #       Component.onCompleted: closeTimer.start()
/// #       Timer {
/// #           id: closeTimer
/// #           interval: 1
/// #           onTriggered: Qt.quit()
/// #       }
///         RowLayout {
///             anchors.centerIn: parent
///             Button {
///                 text: "-"
///                 onClicked: Counter.changeValue(false)
///             }
///             Button {
///                 text: "+"
///                 onClicked: Counter.changeValue(true)
///             }
///         }
///     }
/// "#;
///
/// fn main() {
///     QApp::new()
///         .register::<backend::Counter>()
///         .load_qml(QML_CODE.as_bytes())
///         .run();
/// }
/// ```
#[doc(inline)]
pub use qtbridge_gen::qobject;


/// Annotates a function as a signal that can be handled in QML.
///
/// Signals can be called from Rust and the signal handler can be defined in QML. This is the
/// recommended way to invoke QML code from Rust.
///
/// ### Requirements
///
/// - The signal must be defined within a `mod` or `impl` block, annotated with [`qobject`]
/// or [`qobject_impl`], respectively.
/// - The first argument of the annotated function must be `&self` or `&mut self`.
/// - All other parameter types and the return type must be one of the
/// [supported types][crate::type_support].
/// - The function must not have a body (end with semicolon or have an empty curly braces).
///
/// ```
/// # use qtbridge::qobject_impl;
/// # #[derive(Default)]
/// # pub struct Backend {
/// # }
/// #
/// #[qobject_impl]
/// impl Backend {
///     #[qsignal]
///     fn value_changed(&self, new_value: i32);
///     #[qsignal]
///     fn event_triggered(&self){}
/// }
/// ```
///
/// To receive a notification on the QML side, the object definition has to declare a signal handler named
/// `on<Signal>`, where `<Signal>` is the name of the signal, with the first letter capitalized.
///
/// ```qml,ignore
/// Backend {
///     onValue_changed: console.log("Value changed");
/// }
/// ```
/// Alternatively you can instantiate a `Connection` object with the respective signal handler.
/// ```qml,ignore
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
/// ***qml_name***
///
/// The signal name as seen in QML. Defaults to the Rust function name.
///
#[doc(inline)]
pub use qtbridge_gen::qsignal;

/// Annotates a function as invokable from QML.
///
/// In addition to being invokable from QML, the function can also act as a slot for
/// [signal-slot connections](#signals-and-slots) when used in Qt signal bindings.
///
/// ### Requirements
///
/// - Has to be defined within a `mod` or `impl` block, annotated with [`qobject`]
/// or [`qobject_impl`], respectively.
/// - The annotated function must have a body.
/// - The first argument of the annotated function must be `&self` or `&mut self`.
/// - All other types and the return type must be in the list of [supported types][crate::type_support].
///
/// ### Example
/// ```
/// # use qtbridge::qobject_impl;
/// # #[derive(Default)]
/// # pub struct Backend {
/// #     value: i32,
/// # }
/// #
/// # #[qobject_impl]
/// # impl Backend {
/// #[qslot]
/// fn set_value(&mut self, new_value: i32) {
///     self.value = new_value;
/// }
/// # }
/// ```
///
/// ### Parameters
///
/// **qml_name**
///
/// The function name as seen from QML. Defaults to the Rust function name.
#[doc(inline)]
pub use qtbridge_gen::qslot;

// TODO: Remove name mangling from doc snippets.
/// Registers a property to be accessible from QML.
///
/// ### Requirements
///
/// - The property must be defined within a `mod` or `impl` block, annotated with [`qobject`]
/// or [`qobject_impl`], respectively.
/// - The first parameter is the property name. It must begin with a lower case letter and
/// can only contain letters, numbers and underscores.
/// - The property must be one of the [supported types][crate::type_support].
/// - The return value of the getter (specified via `Read` parameter) must match the property type
/// - The value parameter of the setter (specified via `Write` parameter) must match the property type
/// - The member of the `struct` (specified via `Member` parameter) must match the property type
/// - A signal indicating any property changes (specified via `Notify` parameter) needs to be
/// emitted by the changing function
/// - Getter and setter methods must be defined within the same `impl` block in which the property
/// is declared.
///
/// A property may be **accessor-based** or **member-based** or mix of both (see the [syntax](#qproperty-syntax) section for details).
///
/// ### Accessor based property
///
/// A pure accessor-based property can be declared together with a range of functions:
/// ```
/// # use qtbridge::qobject_impl;
/// # #[derive(Default)]
/// # pub struct Backend {
/// #     value: i32,
/// # }
/// #
/// # #[qobject_impl]
/// # impl Backend {
/// qproperty!("myProperty", Read = get_value, Write = set_value, Notify = "myPropertyChanged");
///
/// pub fn get_value(&self) -> i32 { self.value }
/// pub fn set_value(&mut self, value: i32) {
///     self.value = value;
///     self.my_property_changed();
/// }
/// #[qsignal]
/// pub fn my_property_changed(&self);
/// # }
/// ```
/// The getter method that returns the current value of the property, the setter (if provided) must
/// take the input value of the property as its first argument (after `&mut self`).
///
/// ### Member based property
///
/// Member based properties do not require setter nor getter and Qml will directly read and write
/// to the member. A `Notify` signal has to be provided and it has to be triggered whenever the
/// member is changed.
///
/// A `struct` containing a member-based property may look like:
/// ```
/// # use qtbridge::qobject_impl;
/// #[derive(Default)]
/// struct Text {
///     msg: String
/// }
///
/// #[qobject_impl]
/// impl Text {
///     qproperty!("message", Member = msg, Notify = "messageChanged");
///
///     #[qsignal]
///     fn message_changed(&self);
/// }
/// ```
///
/// More information about Qt properties: <https://doc.qt.io/qt-6/properties.html>.
///
/// ### Parameters of `qproperty!`
///
/// **Name**
///
/// The first argument is a string literal specifying the name of the Qt property.
/// This is the name under which the property is exposed to QML and should follow the naming rules from [requirements](#requirements-2).
///
/// **Read**
///
/// Specifies the getter method for the property in the format `Read = getter_name`.
///
/// **Write**
///
/// Specifies the setter method for the property in the format `Write = setter_name`.
///
/// **Member**
///
/// Specifies the struct member variable that will be accessed if no getter or setter are provided.
/// Expected format: `Member = var_name`.
///
/// **Notify**
///
/// Specifies the name of the signal that has to be emitted when the property changes.
/// Expected format: `Notify = "signal_name"`.
///
/// **Constant**
///
/// A constant property is not allowed to have `Write` or `Notify` parameter.
/// Expected as a single keyword without assignment expression.
///
/// **Default**
///
/// QML writes to the default property if a property is defined within a object but not assigned to any property.
/// For more information see <https://doc.qt.io/qt-6/qtqml-syntax-objectattributes.html>
///
#[doc(inline)]
pub use qtbridge_gen::qproperty;

pub use qtbridge_runtime::{QApp, run_simple_app, qresource};

/// Enable access to C++ and QML.
///
/// This trait is automatically implemented by  [`qobject`] and [`qobject_impl`]
/// and should never be implemented manually.
///
#[doc(inline)]
pub use qtbridge_runtime::QObjectHolder;

/// QmlRegister enables QML to instantiate types of this trait.
///
/// The trait is usually implemented by [`qobject`] and [`qobject_impl`]. If you
/// want to implement this trait manually, you have to add the `NoQmlElement`
/// option.
///
/// [`QmlRegister`] defines the [`ELEMENT_NAME`](QmlRegister::ELEMENT_NAME)
/// with which the `struct` can be instantiated in QML and the module name,
/// [`URI`](QmlRegister::URI), which has to be used as import in QML to
/// use this `struct`.
///
/// [`QmlRegister`] knows two ways of registering a type. The ordinary way
/// is to register as an element that can be instantiated in QML:
///
/// ```
/// # use qtbridge::qobject_impl;
/// # #[derive(Default)]
/// # pub struct Backend {
/// # }
/// #
/// #[qobject_impl(NoQmlElement)]
/// impl Backend {
///     #[qslot]
///     fn say_hello(&self) {
///         println!("Hello World!")
///     }
/// }
/// impl qtbridge::qtbridge_runtime::QmlRegister for Backend {
///     const URI: &str = "rust_backend";
///     const ELEMENT_NAME: &str = "Backend";
///     const MINOR_VERSION: u8 = 0u8;
///     const MAJOR_VERSION: u8 = 1u8;
///     const IS_SINGLETON: bool = false;
/// }
/// ```
///
/// ```qml
/// import rust_backend
/// Backend {
///     id: backend
/// }
/// Button {
///     anchors.centerIn: parent
///     text: "Hello World!"
///     onClicked: backend.sayHello()
/// }
/// ```
///
/// Alternatively, by setting [`IS_SINGLETON`](QmlRegister::IS_SINGLETON)
/// to true, the type is registered as a singleton. That means that only
/// one instance can be created. It can be accessed with the
/// [`ELEMENT_NAME`](QmlRegister::ELEMENT_NAME):
///
/// ```
/// # use qtbridge::qobject_impl;
/// # #[derive(Default)]
/// # pub struct Backend {
/// # }
/// #
/// #[qobject_impl(NoQmlElement)]
/// impl Backend {
///     #[qslot]
///     fn say_hello(&self) {
///         println!("Hello World!")
///     }
/// }
/// impl qtbridge::qtbridge_runtime::QmlRegister for Backend {
///     const URI: &str = "rust_backend";
///     const ELEMENT_NAME: &str = "Backend";
///     const MINOR_VERSION: u8 = 0u8;
///     const MAJOR_VERSION: u8 = 1u8;
///     const IS_SINGLETON: bool = true;
/// }
/// ```
///
/// ```qml
/// import rust_backend
/// Button {
///     anchors.centerIn: parent
///     text: "Hello World!"
///     onClicked: Backend.sayHello()
/// }
/// ```
///
/// Further, [`MAJOR_VERSION`](QmlRegister::MAJOR_VERSION) and
/// [`MINOR_VERSION`](QmlRegister::MINOR_VERSION) define the version of the
/// QML module. These fields are mandatory but QML can load a module without
/// specifying the version
///
#[doc(inline)]
pub use qtbridge_runtime::QmlRegister;

pub use qtbridge_gen::QModelItem;

pub use qtbridge_gen::include_bytes_qml;

pub use qtbridge_interfaces::{QListModel, QListModelBase};
pub use qtbridge_interfaces::{QTableModel, QTableModelBase};

#[doc(hidden)]
pub use qtbridge_interfaces::{QAbstractItemModel, QAbstractItemModelBase};
#[doc(hidden)]
pub use qtbridge_interfaces::{QAbstractListModel, QAbstractListModelBase};
