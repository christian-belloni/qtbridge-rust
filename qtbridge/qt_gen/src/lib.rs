// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro::TokenStream;
use qt_gen_common::type_qualified_mapping::CallOrigin;

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

#[proc_macro_attribute]
pub fn qsignal(_: TokenStream, _: TokenStream) -> TokenStream {
    // This macro does nothing but offer an entry point for Rust doc
    panic!("#[qsignal] proc macro called outside #[qobject] or #[qobject_impl].")
}

#[proc_macro_attribute]
pub fn qslot(_: TokenStream, _: TokenStream) -> TokenStream {
    // This macro does nothing but offer an entry point for Rust doc
    panic!("#[qslot] proc macro called outside #[qobject] or #[qobject_impl].")
}

#[proc_macro]
pub fn qproperty(_: TokenStream) -> TokenStream {
    // This macro does nothing but offer an entry point for Rust doc
    panic!("qproperty! macro called outside #[qobject] or #[qobject_impl].");
}
