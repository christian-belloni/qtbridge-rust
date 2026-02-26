// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
pub use qt_gen_macro;
pub use qt_gen_macro::*;


/// Registers a property to be accessible from QML.
///
/// ### Requirements
///
/// - The property must be defined within a `#[qobject]` (mod) or `#[qobject_impl]` (impl) block
/// - The first parameter is a string literal that specifies the property name.
/// Property names must begin with a lower case letter and can only contain letters, numbers and underscores.
/// - The return value of the getter (specified via `Read` parameter)
/// and/or the value parameter of the setter (specified via `Write` parameter)
/// and/or the member of the `struct` field (specified via `Member` parameter)
/// must have the same type and this type must be supported by the Qt type system (See [supported types](#type-support)).
/// - Getter and setter methods of the property must be defined within the same `impl` block in which the property is declared.
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
#[macro_export]
macro_rules! qproperty {
    // This macro does nothing but offer an entry point for Rust doc
    (
        $name:literal
        , Read = $read:ident
        $(, Write = $write:ident)?
        $(, Notify = $notify:literal)?
        $(, Constant)?
        $(,)?
    ) =>  {{
        #[cfg(test)]
        {
            [
                $name,
                "",
                stringify!($read),
                test_helper!(@opt $( $write )?),
                test_helper!(@notify $( $notify )?)
            ]
        }
        #[cfg(not(test))]
        {
            compile_error!("qproperty! macro called outside #[qobject] or #[qobject_impl].");
        }
    }};

    (
        $name:literal,
        Member = $member:ident
        $(, Read = $read:ident)?
        $(, Write = $write:ident)?
        $(, Notify = $notify:literal)?
        $(, Constant)?
        $(,)?
    ) => {{
        #[cfg(test)]
        {
            [
                $name,
                stringify!($member),
                test_helper!(@opt $( $read )?),
                test_helper!(@opt $( $write )?),
                test_helper!(@notify $( $notify )?)
            ]
        }
        #[cfg(not(test))]
        {
            compile_error!("qproperty! macro called outside #[qobject] or #[qobject_impl].");
        }
    }};
}

#[cfg(test)]
#[macro_export]
macro_rules! test_helper {
    (@opt $val:ident) => {
        stringify!($val)
    };

    (@opt) => {
        ""
    };

    (@notify $val:literal) => {
        $val
    };

    (@notify) => {
        ""
    };
}

mod tests {
    #[test]
    fn test_qproperty_syntax() {

        assert_eq!(
            qproperty!("value", Read = get_value, Notify = "valueChanged"),
            ["value", "", "get_value", "", "valueChanged"]);

        assert_eq!(
            qproperty!("value", Read = get_value, Write = set_value, Notify = "valueChanged"),
            ["value", "", "get_value", "set_value", "valueChanged"]);

        assert_eq!(
            qproperty!("value", Read = get_value, Write = set_value, Notify = "valueChanged", Constant),
            ["value", "", "get_value", "set_value", "valueChanged"]);

        assert_eq!(
            qproperty!("value", Member = value_prop, Notify = "valueChanged"),
            ["value", "value_prop", "", "", "valueChanged"]);

        assert_eq!(
            qproperty!("value", Member = value_prop, Notify = "valueChanged", Constant),
            ["value", "value_prop", "", "", "valueChanged"]);

        assert_eq!(
            qproperty!("value", Member = value_prop, Read = get_value, Notify = "valueChanged"),
            ["value", "value_prop", "get_value", "", "valueChanged"]);

        assert_eq!(
            qproperty!("value", Member = value_prop, Write = set_value, Notify = "valueChanged"),
            ["value", "value_prop", "", "set_value", "valueChanged"]);

        assert_eq!(
            qproperty!("value", Member = value_prop, Read = get_value, Write = set_value, Notify = "valueChanged"),
            ["value", "value_prop", "get_value", "set_value", "valueChanged"]);

    }
}
