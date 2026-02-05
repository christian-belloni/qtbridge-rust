// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#![cfg(test)]
use qt_gen_impl::QObjectModuleBuilder;
use quote::{ToTokens, quote};
use crate::tst_assert::assert_tokens_eq;
use qt_gen_common::type_qualified_mapping::CallOrigin;
use qt_gen_common::format_code::strip_docs;

#[test]
pub fn require_that_qobject_macro_generates_interface_impl_code_that_agrees_with_reference() {
    let input = quote! {
        mod some_module {
            #[derive(Default)]
            struct SomeStruct {
            }

            impl SomeStruct {
                fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32 ) -> bool {
                    false
                }

                fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
                    //QVariant::default()
                }

                fn row_count(&self, parent: &QModelIndex) -> i32 {
                    1
                }
            }
        }
    };

    let input_params = quote!{
        Base = QAbstractListModel
    };

    let mut builder = QObjectModuleBuilder::new(CallOrigin::External);
    let output = builder.build(input, input_params).unwrap().to_token_stream();

    /* Uncomment to update the baseline
    use std::fs;
    use std::path::Path;
    use qt_gen_common::format_code::format_rust_code;

    let baseline_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("test_baselines")
        .join("somestruct_iface.rs");

    fs::write(baseline_path,format_rust_code(&strip_docs(output.clone())).unwrap()).unwrap();
    */

    let baseline_src = include_str!("test_baselines/somestruct_iface.rs");
    let baseline: proc_macro2::TokenStream =
        baseline_src.parse().expect("baseline is not valid Rust");

    assert_tokens_eq(&strip_docs(output),
                   &strip_docs(baseline));

}
