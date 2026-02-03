// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro::TokenStream;

use syn::{parse_macro_input, ItemFn, Ident, LitStr};
use quote::quote;
use qt_gen_common::parse_utils::parse_name_value;

struct QObjectTestData {
    class: Option<syn::Path>,
    name: Option<syn::LitStr>,
    input_folder: Option<syn::LitStr>,
}

mod qobject_test_data_keywords {
    syn::custom_keyword!(Class);
    syn::custom_keyword!(Name);
    syn::custom_keyword!(Input);
}

impl syn::parse::Parse for QObjectTestData {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut class = None;
        let mut name = None;
        let mut input_folder = None;

        while !input.is_empty() {
            match () {
                _ if input.peek(qobject_test_data_keywords::Class) => {
                    class = Some(parse_name_value::<Ident, syn::Path>(input)?.1);
                }
                _ if input.peek(qobject_test_data_keywords::Name) => {
                    name = Some(parse_name_value::<Ident, LitStr>(input)?.1);
                }
                _ if input.peek(qobject_test_data_keywords::Input) => {
                    input_folder = Some(parse_name_value::<Ident, LitStr>(input)?.1);
                }
                _ => {
                    return Err(input.error(format!(
                        "Unsupported attribute of run_quick_test macro: '{}'",
                        input
                    )));
                }
            }

            // Parse optional trailing comma once
            let _comma: Option<syn::Token![,]> = input.parse()?;
        }
        Ok(QObjectTestData {
            class,
            name,
            input_folder
        })
    }
}


#[proc_macro_attribute]
pub fn run_quick_test(attr: TokenStream, item: TokenStream) -> TokenStream {

    let mut input_fn: ItemFn = parse_macro_input!(item as ItemFn);

    input_fn.attrs.push(syn::parse_quote!(#[test]));

    let fn_name = input_fn.sig.ident.to_string();
    let parsed = syn::parse2::<QObjectTestData>(attr.into());

    match parsed {
        Ok(data) => {

            let subfolder: LitStr = data.input_folder.unwrap_or_else(|| syn::parse_quote!(""));

            let file_name = file!().to_string();

            let common_setup: proc_macro2::TokenStream = quote! {
                use std::env;
                use std::path::PathBuf;

                let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                input_path.push(&#subfolder.to_string());
                let input_folder = input_path.to_str().unwrap().to_string();

                println!("Running quick test with qml files in \"{}\"", &input_folder);

                let args: Vec<String> = vec![
                    #file_name.into(),
                    "-input".into(),
                    input_folder,
                ];
            };

            input_fn.block = if data.class.is_some() {

                let class_ident = data.class.unwrap_or_else(|| syn::parse_quote!(TestQObject));
                let name_ident: LitStr  = data.name.unwrap_or_else(|| syn::parse_quote!(""));

                syn::parse_quote!({
                    #common_setup

                    use qtbridge::qt_type_lib::QVariantMap;
                    use qtbridge::quicktest::quick_test_main_with_properties;
                    use qtbridge::QObjectHolder;

                    let test_object = #class_ident::default_with_attached_qobject();
                    let properties = QVariantMap::from((#name_ident, test_object.borrow().as_qvariant()));
                    let result = quick_test_main_with_properties(&args, &#fn_name.to_string(), &properties);

                    assert_eq!(result, 0, "quick_test failed with code {}", result);
                })
            } else {
                syn::parse_quote!({
                    #common_setup

                    use qtbridge::quicktest::quick_test_main;

                    let result = quick_test_main(&args, &#fn_name.to_string());
                    assert_eq!(result, 0, "quick_test failed with code {}", result);
                })
            };
        }
        Err(err) => panic!("Error while creating quick test: {}", err),
    }

    quote!(#input_fn).into()
}
