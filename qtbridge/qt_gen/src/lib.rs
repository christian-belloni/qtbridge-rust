// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
pub use qt_gen_macro;
pub use qt_gen_macro::*;

#[macro_export]
macro_rules! qproperty {
    // This macro does nothing but offer an entry point for Rust doc
    (
        $name:literal
        , Read = $read:ident
        $(, Write = $write:ident)?
        $(, Notify = $notify:literal)?
        $(, Constant)?
        $(, Default)?
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
        $(, Default)?
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
