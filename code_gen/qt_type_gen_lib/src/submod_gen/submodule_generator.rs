// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::TokenStream;

use crate::reexport::Reexport;

// Common interface to deal with submodules without knowing its type
pub trait SubmoduleGenerator {

    fn generate_rust(&self) -> syn::Result<TokenStream>;
    fn generate_cpp(&self) -> syn::Result<(String, String)>;

    fn register_type(&self) -> syn::Result<()>;

    fn submod_name(&self) -> String;

    // Return input file path relative to input root "qt_type_gen/src/input"
    fn input_file_path(&self) -> String;

    fn check_unclassified_type_tokens(&mut self) -> syn::Result<()>;
    fn substitute_monomorphed_types_if_needed(&mut self) -> syn::Result<()>;
    fn get_non_bridge_reexport(&self) -> syn::Result<Reexport>;
    fn get_unresolved_type_dependencies(&self) -> Vec<syn::Path>;
    fn is_cxx_present(&self) -> bool;
}
