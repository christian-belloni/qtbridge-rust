// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum MethodKind {
    PureVirtual,
    ImplVirtual,
    NonVirtual,  // Non virtual function defined on C++ side that we want to call from our Rust code
}

// TODO: move MethodDesc to dedicated file
pub struct IfaceMethodDesc {
    kind: MethodKind,
    sig: syn::Signature,
    cpp_name: String,
}

impl IfaceMethodDesc {
    pub(crate) fn new(kind: MethodKind, sig: syn::Signature, cpp_name: String,) -> Self {
        Self{ kind, sig, cpp_name }
    }

    pub fn is_virtual(&self) -> bool {
        match self.kind {
            MethodKind::PureVirtual | MethodKind::ImplVirtual => true,
            _ => false,
        }
    }

    pub fn is_pure_virtual(&self) -> bool {
        self.kind == MethodKind::PureVirtual
    }

    pub fn has_base_implementation(&self) -> bool {
        match self.kind {
            MethodKind::ImplVirtual | MethodKind::NonVirtual => true,
            _ => false,
        }
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
