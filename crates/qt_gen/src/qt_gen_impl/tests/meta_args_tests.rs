#![cfg(test)]
use qtbridge_gen_common::type_registry::meta_types::get_qmetatype_support_for_type;

fn check_if_returns_expected_type(src_type_str: &str, exp_type_str: Option<&str>) {
    let src_type: syn::Type = syn::parse_str(src_type_str)
        .unwrap();
    let expected_type: Option<syn::Type> = exp_type_str.map(|str| syn::parse_str(str).unwrap());
    let result_type = get_qmetatype_support_for_type(&src_type)
        .unwrap();
    assert_eq!(expected_type, result_type);
}

#[test]
pub fn tst_qmetatype_support_for_primitives()
{
    let inputs = [
        ("i8",    None),
        ("u8",    None),
        ("i16",   None),
        ("u16",   None),
        ("i32",   None),
        ("u32",   None),
        ("i64",   None),
        ("u64",   None),
        ("isize", None),
        ("usize", None),
        ("f32",   None),
        ("f64",   None),
        ("bool",  None),
    ];

    for (src_type_str, exp_type_type) in inputs {
        check_if_returns_expected_type(src_type_str, exp_type_type)
    }
}

#[test]
pub fn tst_qmetatype_support_for_strings()
{
    qt_type_lib::init();
    let inputs = [
        ("str",    Some("qt_type_lib::QString")),
        ("String", Some("qt_type_lib::QString")),
    ];

    for (src_type_str, exp_type_type) in inputs {
        check_if_returns_expected_type(src_type_str, exp_type_type)
    }
}

#[test]
pub fn tst_qmetatype_support_for_vectors() {
    qt_type_lib::init();
    let inputs = [
        ("Vec<i8>",      Some("qt_type_lib::QList_i8")),
        ("Vec<u8>",      Some("qt_type_lib::QList_u8")),
        ("Vec<i16>",     Some("qt_type_lib::QList_i16")),
        ("Vec<u16>",     Some("qt_type_lib::QList_u16")),
        ("Vec<i32>",     Some("qt_type_lib::QList_i32")),
        ("Vec<u32>",     Some("qt_type_lib::QList_u32")),
        ("Vec<i64>",     Some("qt_type_lib::QList_i64")),
        ("Vec<u64>",     Some("qt_type_lib::QList_u64")),
        ("Vec<f32>",     Some("qt_type_lib::QList_f32")),
        ("Vec<f64>",     Some("qt_type_lib::QList_f64")),
        ("Vec<String>",  Some("qt_type_lib::QList_QString")),
    ];

    for (src_type_str, exp_type_type) in inputs {
        check_if_returns_expected_type(src_type_str, exp_type_type)
    }
}
