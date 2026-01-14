// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::TokenStream;
use quote::quote;

use qt_gen_common::naming;
use qt_gen_common::type_dependencies::qt_types_to_rust_import_paths;
use qt_gen_common::type_tokens::TypeTokens;

use crate::InterfaceDesc;

/// Structure that generate a trait representing the virtual functions of the original C++ interface.
/// Produce a TokenStream that will be written to the dedicated 'interface_trait.rs' file.
/// See 'qtbridge/qt_ifaces/src/generated/*/interface_trait.rs' for an example of the generated code.
/// See sequence diagrams in 'qtbridge/qt_ifaces/docs/uml/' illustrating the structure at a higher level.
pub struct IfaceTraitGenerator<'a> {
    iface: &'a InterfaceDesc,
}

impl<'a> IfaceTraitGenerator<'a> {
    /// Create instance.
    pub fn new(iface: &'a InterfaceDesc) -> Self {
        Self {
            iface
        }
    }

    /// Generate trait consisting of virtual function from original C++ interface.
    pub fn generate(&self) -> syn::Result<TokenStream> {
        let iface = self.iface;
        let iface_name = iface.get_ident();

        let qt_type_lib_imports = self.generate_qt_type_lib_imports()?;
        let name = naming::rust::traits::iface_trait(iface_name);

        let signatures = iface.get_virtual_methods()
            .map(|m| m.get_signature());

        Ok(quote!{
            #qt_type_lib_imports

            pub trait #name {
                #(#signatures;)*
            }
        })
    }

    /// Get list of types used in arguments or function return
    fn get_rust_type_tokens_used_in_signatures(&self) -> syn::Result<TypeTokens> {
        let mut tokens = TypeTokens::default();
        self.iface.get_virtual_methods()
            .try_for_each(|m| tokens.collect_from_signature(m.get_signature()))?;
        Ok(tokens)
    }

    /// Generate imports for types used in function signatures
    fn generate_qt_type_lib_imports(&self) -> syn::Result<TokenStream> {
        let tokens = self.get_rust_type_tokens_used_in_signatures()?;
        qt_types_to_rust_import_paths(tokens.iter_qt())
    }
}
