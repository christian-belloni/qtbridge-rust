// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qt_gen_common::cpp_fn_sign::CppFnSign;
use qt_gen_common::signature_utils::{is_arg_self_ref, ExpectSelfRef};

// Data structures that describe (in a very simplified way) C++ class function to be generated

pub struct CppMethodSignature {
    func: CppFnSign,
    is_constant: bool,
}

impl CppMethodSignature {
    pub fn new_from_rust_sig(sig: &syn::Signature, cpp_name: Option<String>) -> syn::Result<Self> {
        let func = CppFnSign::new_from_rust_sig(sig, cpp_name, ExpectSelfRef::Yes)?;
        let is_constant = is_arg_self_ref(&sig.inputs[0], Some(false));

        Ok(Self{ func, is_constant })
    }

    pub fn get_maybe_return_op(&self) -> &'static str {
        self.func.get_maybe_return_op()
    }

    pub fn to_declaration_str(&self) -> String {
        let func_decl = self.func.to_declaration_string(false);
        let maybe_const = self.get_maybe_const();
        format!("{func_decl}{maybe_const}")
    }

    pub fn to_definition_str(&self, class_name: &str) -> String {
        let f = &self.func;
        let return_type = f.get_return_type();
        let name        = f.get_name();
        let args        = f.get_typed_argument_list();
        let maybe_const = self.get_maybe_const();
        format!("{return_type} {class_name}::{name}({args}){maybe_const}")
    }

    pub fn get_name(&self) -> &str {
        self.func.get_name()
    }

    pub fn get_arguments_forward(&self) -> String {
        self.func.get_typed_arguments_forward()
    }

    fn get_maybe_const(&self) -> &str {
        if self.is_constant { " const" } else { "" }
    }
}

