// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro::TokenStream;

use qt_gen_common::type_qualified_mapping::CallOrigin;

// TODO: add documentation here.
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

/// The macro that enables Qt meta-object features, allowing a Rust `struct` to be exposed to QML.
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
/// Rust allows to add multiple `impl` blocks for the same `struct`. However, if you apply the `#[qobject_impl]` macro to one of them,
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
/// # Supported annotations
///
/// The macro supports [signals](#qsignal) and [slots](#qslot) as method-level attributes.
/// In addition, it provides a function-like macro for declaring [properties](#qproperty).
///
/// ## Signals and Slots
///
/// Signals and slots are a core concept of the Qt framework. They are used for communication
/// between objects in a type-safe and decoupled manner. A signal is emitted (called) when a particular event occurs.
///
/// Other objects can connect their slots to this signal, and those slots get automatically invoked when the signal is emitted.
///
/// Unlike Rust function pointers or functional traits, signals do not require you to manage the signal-receiver object
/// or functor at a low level. In other words,
/// you do not need to manually hold a reference to the receiver or a pointer to a function
/// or closure associated with the signal.
///
/// Slots are callable methods that can be invoked from QML. A slot can also be called as a regular method from Rust code.
///
/// More information about signals, slots and connections: <https://doc.qt.io/qt-6/signalsandslots.html>, <https://doc.qt.io/qt-6/qml-qtqml-connections.html>.
///
/// ## `#[qsignal]`
///
/// Annotates a method as a **Qt signal**.
///
/// ### Requirements
///
/// - The first argument of a signal signature must be `&self` or `&mut self`.
/// - The return type and types of all parameters following `self` must be in the list of [supported types](#type-support).
/// - A function annotated with `#[qsignal]` must not have a body (end with semicolon or have an empty curly braces):
///
/// ```ignore
/// #[qsignal]
/// fn value_changed(&self, new_value: i32);
/// ```
///
/// or
///
/// ```ignore
/// #[qsignal]
/// fn value_changed(&self, new_value: i32)
/// {}
/// ```
///
/// Rust and QML use different naming conventions: Rust prefers *snake_case*, while QML typically uses *camelCase*.
///
/// By default, the name of a signal or slot on QML side is the name of the corresponding Rust method, but converted to *camelCase*.
/// For example, a signal declared as
/// ```ignore
/// #[qsignal]
/// fn outside_temperature_changed();
/// ```
/// results in a signal named "outsideTemperatureChanged" on QML side.
///
/// The signal name exposed to QML can be explicitly overridden by `qml_name` parameter of the `#[qsignal]` annotation as shown below.
/// The Rust method name remains unchanged.
///
/// ```ignore
/// #[qsignal(qml_name = "configurationChanged")]
/// fn cfg_changed();
/// ```
///
/// To see how signals are handled on the QML side, look at: <https://doc.qt.io/qt-6/qtqml-syntax-signals.html>.
///
/// ## `#[qslot]`
///
/// Annotates a method as a **Qt slot**. In addition to being part of [signal-slot connection](#signals-and-slots), a slot also is a convenient way to invoke Rust code from QML.
///
/// ### Requirements
///
/// - The annotated function must have a body.
/// - The first argument of a slot must be `&self` or `&mut self`.
/// - The return type and types of all parameters following `self` must be in the list of [supported types](#type-support).
///
/// ### Example
/// ```ignore
/// #[qslot]
/// fn on_control_changed(&mut self, new_value: i32) {
///     if new_value != self.value {
///         self.value = new_value;
///         self.update();
///     }
/// }
/// ```
///
/// Similarly to signals, the name of a slot exposed to QML can be explicitly controlled using the `qml_name` parameter:
///
/// ```ignore
/// #[qslot(qml_name = "myControlChanged")]
/// fn on_control_changed(&mut self, new_value: i32) {
///    self.update_relevant_parameters(new_value);
/// }
/// ```
///
/// ## `qproperty!`
///
/// `qproperty!` declares a Qt property and registers it in the Qt meta-object data of the `struct` it belongs to.
///
/// ### Requirements
///
/// - A property must have a name specified as a string literal in the first argument.
/// Property names must begin with a lower case letter and can only contain letters, numbers and underscores.
/// - The return value of the getter (specified via `Read` parameter)
/// and/or the value parameter of the setter (specified via `Write` parameter)
/// and/or the type of the `struct` field (specified via `Member` parameter)
/// must have the same type and this type must be supported by the Qt type system (See [supported types](#type-support)).
/// - Getters and setters methods of the property must be defined within the same `impl` block in which the property is declared.
///
/// A property may be **accessor-based** or **member-based** or mix of both (see the [syntax](#qproperty-syntax) section for details).
///
/// A pure accessor-based property can be declared as follows:
/// ```ignore
/// qproperty!("myProperty", Read = get_value, Write = set_value, Notify = "myPropertyChanged");
/// ```
///
/// An accessor-based property must have a getter method that returns the current value of the property.
///
/// ```ignore
/// fn get_value(&self) -> i32 {
///     self.prop_value
/// }
/// ```
///
/// A setter (if provided) must take the input value of the property as its first argument (after `&mut self`).
///
/// ```ignore
/// fn set_value(&mut self, value: i32) {
///     if self.prop_value != value {
///         self.prop_value = value;
///         self.my_property_changed();
///     }
/// }
/// ```
///
/// For member-based properties, the getter and/or setter code is generated automatically.
/// The generated code provides read/write access to the struct field with the specified name.
/// If the property declaration includes a `Notify` parameter,
/// the generated setter automatically emits the specified signal when the value changes.
///
/// A `struct` containing a member-based property may look like:
/// ```ignore
/// use qt_bridge::qobject_impl;
///
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
///
/// To make a property work as expected, the macro performs several tasks under the hood.
/// It deduces the property type from the getter’s return type, the setter’s first
/// typed parameter, or the member field type for member-based properties.
/// If the types inferred from property's `Read`/`Write`/`Member` are not consistent, a compilation error is emitted.
/// The macro also generates member-based getters and setters if they are not provided by the user.
/// And it registers the property in the type’s metadata.
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
/// Specifies the struct member variable that will be accessed by automatically generated getter and setter code.
/// Expected format: `Member = var_name`.
///
/// **Notify**
///
/// Specifies the name of the signal that is emitted when the property value changes.
/// Expected format: `Notify = "signal_name"`.
///
/// **Constant**
///
/// Marks the property value as constant. A constant property is not allowed to have `Write` or `Notify` parameter.
/// Expected as a single keyword without assignment expression.
///
/// ### Notes on `qproperty!`
///
/// - Property name is case-sensitive.
/// - Parameters are separated by commas.
/// - The order of property parameters after the property name is not significant.
/// - The `qproperty!` must be declared inside a `impl` block annotated with `#[qobject_impl]`.
///
/// ### `qproperty!` syntax
///
/// The syntax of properties in Rust can be expressed in the following general form:
///
/// ```ignore
/// qproperty!(
///     name,
///     (Read = get_function, [Write = set_function,] |
///      Member = field_name, [(Read = get_function | Write = set_function),])
///     [NOTIFY = "notifySignalName",]
///     [Constant,]
/// );
/// ```
///
/// More information about Qt properties: <https://doc.qt.io/qt-6/properties.html>.
///
/// # Type Support
///
/// The set of types supported in signatures of functions marked as [signal](#qsignal), [slot](#qslot) or [qproperty!](#qproperty) accessors is limited.
/// The currently supported types are:
/// - **Scalar types**: `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`, `isize`, `usize`, `f32`, `f64`.
/// - **String types**: `String` and `&str`.
/// - **Collections**: `Vec<String>`.
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
/// There are two ways to construct object in Rust to send it further to QML:
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

/// The macro that registers a user-defined Rust `struct` type as a QML [object type](https://doc.qt.io/qt-6/qtqml-typesystem-objecttypes.html).
///
/// Once a `struct` is registered in the QML type system, it can be imported and instantiated from QML like any other object type.
/// In QML syntax, the initialization of an object can be done by specifying the type name followed by curly brackets that contain the object's properties initializers.
/// For example, QML initialization can look like:
///
/// ### Example
///
/// ```javascript,ignore
/// import QtQuick
/// import QtQuick.Controls
/// import address_book
///
/// ApplicationWindow {
///     ...
///     property Address address: Address {
///         city: "London"
///         street: "Baker Street"
///         house: 221
///     }
///     ...
/// }
/// ```
///
/// for the `struct` defined on the Rust side as follows:
///
/// ```rust,ignore
/// #[qml_element]
/// struct Address {
///     city: String,
///     street: String,
///     house: i32,
/// }
/// #[qobject_impl]
/// impl Address {
///     qproperty!("city", Member = city, Notify = "cityChanged");
///     qproperty!("street", Member = street, Notify = "streetChanged");
///     qproperty!("house", Member = house, Notify = "houseChanged");
///
///     #[qsignal]
///     fn city_changed(&self);
///     #[qsignal]
///     fn street_changed(&self);
///     #[qsignal]
///     fn house_changed(&self);
/// }
/// ```
///
/// Currently, the name of the QML module that must be imported in QML code is derived
/// from the `name` value in the `package` section of the corresponding manifest file ('Cargo.toml'),
/// by applying the following transformations:
/// - All leading digits are removed.
/// - All characters that are neither alphabetic not digits are replaced with '_'.
///
/// Once an instance is created in QML, its exposed method and properties can be accessed and manipulated directly from QML code.
#[proc_macro_attribute]
pub fn qml_element(args: TokenStream, input: TokenStream) -> TokenStream {
    let output = match qt_gen_impl::qml_element(args.into(), input.into()) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    };
    output.into()
}
