// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum MethodKind {
    /// A pure virtual function.
    Pure,

    /// A virtual function with an implementation.
    Implemented,

    /// A non-virtual function defined on the C++ side that is called from Rust.
    NonVirtual,
}

// TODO: move MethodDesc to dedicated file
pub struct IfaceMethodDesc {
    kind: MethodKind,
    sig: syn::Signature,
    cpp_name: String,
}

impl IfaceMethodDesc {
    pub(crate) fn new(kind: MethodKind, sig: syn::Signature, cpp_name: String) -> Self {
        Self{ kind, sig, cpp_name }
    }

    pub fn is_virtual(&self) -> bool {
        matches!(self.kind, MethodKind::Pure | MethodKind::Implemented)
    }

    pub fn is_pure_virtual(&self) -> bool {
        self.kind == MethodKind::Pure
    }

    pub fn has_base_implementation(&self) -> bool {
        matches!(self.kind, MethodKind::Implemented | MethodKind::NonVirtual)
    }

    pub fn get_cpp_name(&self) -> &str {
        self.cpp_name.as_str()
    }

    pub fn get_signature(&self) -> &syn::Signature {
        &self.sig
    }

    pub fn get_receiver(&self) -> syn::Result<&syn::Receiver> {
        match self.sig.inputs.first() {
            Some(syn::FnArg::Receiver(receiver)) => Ok(receiver),
            _ => Err(syn::Error::new(self.sig.ident.span(), "Function does not have reciever argument (self)")),
        }
    }

}
