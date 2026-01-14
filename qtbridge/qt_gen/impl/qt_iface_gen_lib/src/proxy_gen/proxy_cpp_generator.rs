// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::collections::BTreeSet;

use qt_gen_common::{cpp_include::CppInclude, type_tokens::TypeTokens};
use qt_gen_common::{case_conv, naming};
use qt_gen_common::type_dependencies::type_tokens_to_cpp_includes;

use crate::{IfaceMethodDesc, InterfaceDesc};
use super::cpp_method::CppMethodSignature;

/// Struct responsible for generation C++ proxy of given interface.
pub struct CppProxyGenerator<'a> {
    iface: &'a InterfaceDesc,
    base_name: String,
    class_name: String,
}

impl<'a> CppProxyGenerator<'a> {
    /// Create a new instance.
    pub fn new(iface: &'a InterfaceDesc) -> Self {
        let base_name = iface.get_ident().to_string();
        let class_name = naming::cpp::class::proxy_cpp(&base_name).to_string();
        Self { iface, base_name, class_name }
    }

    /// Run the code generation.
    /// Produces strings with the code for header and cpp file.
    /// See C++ source files in 'qtbridge/qt_ifaces/src/generated/*/cpp/' for an example of the generated code.
    /// See diagrams in 'qtbridge/qt_ifaces/docs/uml/' illustrating the structure at a higher level.
    pub fn generate(&self) -> syn::Result<(String, String)> {

        let Self { base_name, class_name, .. } = &self;

        let include_guard_id = format!("_{class_name}_RUST_BRIDGE__H_").to_ascii_uppercase();
        let includes = self.get_proxy_includes()?;
        let namespace = naming::cpp::namespace::bridge();

        let (ctor_decl, ctor_def) = self.generate_constructor_code();
        let (dtor_decl, dtor_def) = self.generate_destructor_code();
        let member_vars = self.generate_member_variables_declaration();

        // Declarations and definitions of the class methods

        let (mut virt_func_decl, mut virt_func_def) = self.generate_cpp_to_rust_forwarding_virtual_methods()?;
        if !virt_func_decl.is_empty() {
            virt_func_decl = format!("\n    // Virtual methods\n{virt_func_decl}");
            virt_func_def = format!("\n\n// Virtual methods\n{virt_func_def}");
        }

        let (mut virt_base_decl, mut virt_base_def) = self.generate_implemented_functions(true)?;
        if !virt_base_decl.is_empty() {
            virt_base_decl = format!("\n    // Access to base implementation of virtual functions\n{virt_base_decl}");
            virt_base_def = format!("\n\n// Access to base implementation of virtual functions\n{virt_base_def}");
        }

        let (mut non_virt_base_decl, mut non_virt_base_def) = self.generate_implemented_functions(false)?;
        if !non_virt_base_decl.is_empty() {
            non_virt_base_decl = format!("\n    // Access to base implementation of non virtual functions\n{non_virt_base_decl}");
            non_virt_base_def = format!("\n\n// Access to base implementation of non virtual functions\n{non_virt_base_def}");
        }

        let (creation_decl, creation_def) = self.generate_create_functions();

        // Generate header code
        let header = format!(
r#"#ifndef {include_guard_id}
#define {include_guard_id}
{includes}

namespace {namespace} {{

class {class_name} : public {base_name}, public RustObjectGetter
{{
    using Base = {base_name};

public:
{ctor_decl}{dtor_decl}{virt_func_decl}{virt_base_decl}{non_virt_base_decl}{member_vars}}};

{creation_decl}
}} // namespace {namespace}

#endif // {include_guard_id}
"#);

        // Generate code of cpp file
        let header_name = naming::cpp::filename::proxy_header(&base_name);

        let cpp = format!(
r#"#include "{header_name}"

namespace {namespace} {{

{ctor_def}{dtor_def}{virt_func_def}{virt_base_def}{non_virt_base_def}

{creation_def}
}} // namespace {namespace}
"#);

        Ok((header, cpp))
    }

    /// Generate includes that will be inserted at the top of header file.
    fn get_proxy_includes(&self) -> syn::Result<String> {
        // Includes from interface declaration
        let mut includes = self.iface.get_includes().iter()
            .map(|i| i.clone())
            .collect::<BTreeSet<_>>();

        // Includes obtained from methods of interfaces
        let type_tokens = self.get_rust_type_tokens_used_in_methods()?;
        includes.extend(type_tokens_to_cpp_includes(&type_tokens)?);
        includes.insert(CppInclude::new_in_brackets("QMetaObject"));
        includes.insert(CppInclude::new_in_brackets("QQmlListProperty"));
        includes.insert(CppInclude::new_in_quotes("bridge/src/cpp/rustobjectgetter.h"));

        // Include header with Rust proxy bridge
        {
            let module_path = naming::rust::path::generated_module_dir(&self.base_name);
            let filename = naming::rust::filename::proxy_rust_bridge();
            let rust_proxy_include = format!("{module_path}{filename}.h");
            includes.insert(CppInclude::new_in_quotes(&rust_proxy_include));
        }

        Ok(includes.into_iter()
            .fold(String::new(), |acc, i| acc + &i.to_cpp_code()))
    }

    /// Return types used in interface functions (both virtual and not).
    fn get_rust_type_tokens_used_in_methods(&self) -> syn::Result<TypeTokens> {
        let mut tokens = TypeTokens::default();
        self.iface.get_methods().iter()
            .try_for_each(|f| tokens.collect_from_signature(f.get_signature()))?;
        Ok(tokens)
    }

    /// Generate declaration and definition of proxy constructor.
    fn generate_constructor_code(&self) -> (String, String) {
        let class_name = &self.class_name;
        let rust_proxy_type = naming::cpp::class::proxy_rust(&self.base_name);
        let rust_impl_var_name = naming::cpp::class_variables::proxy::rust_proxy();

        let declaration = format!("    {class_name}(uint8_t* rustObj, {rust_proxy_type}* rustProxy);\n");
        let definition = format!(
r#"{class_name}::{class_name}(uint8_t* rustObj, {rust_proxy_type}* rustProxy)
    :  RustObjectGetter(rustObj)
    , {rust_impl_var_name}(rustProxy)
{{}}
"#);

        (declaration, definition)
    }

    /// Generate declaration and definition of proxy destructor.
    fn generate_destructor_code(&self) -> (String, String) {
        let class_name = &self.class_name;
        let class_proxy_rust = naming::cpp::class::proxy_rust(&self.base_name);
        let drop_self = naming::cpp::function::drop_self();
        let rust_proxy_var = naming::cpp::class_variables::proxy::rust_proxy();
        let rust_obj_var = naming::cpp::class_variables::proxy::rust_obj();

        let declaration = format!("    ~{class_name}();\n");
        let definition = format!(
r#"{class_name}::~{class_name}()
{{
    {class_proxy_rust}::{drop_self}({rust_proxy_var}, {rust_obj_var});
}}"#);
        (declaration, definition)
    }

    /// Generate code with declaration of proxy member variables.
    fn generate_member_variables_declaration(&self) -> String {
        let rust_proxy_type_name = naming::cpp::class::proxy_rust(&self.base_name);
        let rust_impl_var_name = naming::cpp::class_variables::proxy::rust_proxy();

        format!(r#"
private:
    {rust_proxy_type_name}* {rust_impl_var_name};
"#)
    }

    /// Generate declaration and definition of virtual methods of given interface
    /// forwarding the call to Rust proxy.
    fn generate_cpp_to_rust_forwarding_virtual_methods(&self) -> syn::Result<(String, String)> {
        let proxy_name = naming::cpp::class::proxy_cpp(&self.base_name);

        let mut declarations = String::new();
        let mut definitions = String::new();

        for method in self.iface.get_virtual_methods() {
            let sig = method.get_signature();
            let (decl, def) = self.generate_cpp_to_rust_forwarding_virtual_method(sig, &proxy_name.to_string())?;

            declarations.push_str(&format!("    {decl}\n"));
            definitions.push_str(&format!("{def}"));
        }

        Ok((declarations, definitions))
    }


    /// Generate declaration and definition of certain virtual method
    /// forwarding the call to Rust proxy.
    fn generate_cpp_to_rust_forwarding_virtual_method(&self, sign: &syn::Signature, proxy_name: &str) -> syn::Result<(String, String)> {
        let cpp_sign = CppMethodSignature::new_from_rust_sig(sign, None)?;
        let func_name = cpp_sign.get_name();
        let maybe_return = cpp_sign.get_maybe_return_op();
        let rust_impl_var_name = naming::cpp::class_variables::proxy::rust_proxy();

        let forward_args = cpp_sign.get_arguments_forward();

        let declaration = cpp_sign.to_declaration_str() + " override;";
        let def_sign = cpp_sign.to_definition_str(proxy_name);
        let definition = format!(
r#"{def_sign}
{{
    {maybe_return}{rust_impl_var_name}->{func_name}({forward_args});
}}
"#);
        Ok((declaration, definition))
    }

    /// Generate declaration and definition of functions
    /// forwarding call to the implementation the in base class.
    fn generate_implemented_functions(&self, for_virtual: bool) -> syn::Result<(String, String)> {
        let mut decls = String::new();
        let mut defs = String::new();

        for method in self.iface.get_implemented_methods() {
            if method.is_virtual() != for_virtual {
                continue
            }
            let (decl, def) = self.generate_implemented_function(method)?;
            decls.push_str(&format!("    {decl}\n"));
            defs.push_str(&format!("{def}"));
        }

        Ok((decls, defs))
    }

    /// Generate declaration and definition of function
    /// forwarding call to the base class implementation.
    fn generate_implemented_function(&self, method: &IfaceMethodDesc) -> syn::Result<(String, String)> {
        let sign = method.get_signature();
        let name_callee = case_conv::snake_to_camel(&sign.ident.to_string());
        let mut name_caller = name_callee.clone();
        if method.is_virtual() {
            name_caller = naming::cpp::function::base(&name_caller).to_string();
        }

        let cpp_sign = CppMethodSignature::new_from_rust_sig(sign, Some(name_caller))?;
        let maybe_return = cpp_sign.get_maybe_return_op();
        let forward_args = cpp_sign.get_arguments_forward();

        let declaration = cpp_sign.to_declaration_str() + ";";
        let def_sign = cpp_sign.to_definition_str(&self.class_name);
        let definition = format!(
r#"{def_sign}
{{
    {maybe_return}Base::{name_callee}({forward_args});
}}
"#);
        Ok((declaration, definition))
    }

    /// Generate functions that will be used when constructing proxies in different scenarios.
    pub fn generate_create_functions(&self) -> (String, String) {

        let Self { base_name, class_name, .. } = &self;

        let rust_proxy_type_name = naming::cpp::class::proxy_rust(&self.base_name);
        let create_func_name = naming::cpp::function::create_proxy_cpp(&class_name);
        let create_at_func_name = naming::cpp::function::create_proxy_cpp_at(&class_name);
        let static_meta_func_name  = naming::cpp::function::static_meta_object(&class_name);
        let sizeof_func_name = naming::cpp::function::sizeof_proxy_cpp(&class_name);
        let alignof_func_name = naming::cpp::function::alignof_proxy_cpp(&class_name);
        let qmetatype_list_func_name = naming::cpp::function::qmetatype_list(&class_name);

        let declaration = format!(r#"// Functions for object construction
{class_name}* {create_func_name}(uint8_t* rustObj, {rust_proxy_type_name}* rustProxy);
{class_name}* {create_at_func_name}(uint8_t* addr, uint8_t* rustObj, {rust_proxy_type_name}* rustProxy);
const QMetaObject& {static_meta_func_name}();
size_t {sizeof_func_name}();
size_t {alignof_func_name}();
QMetaType {qmetatype_list_func_name}();
"#);

        let definition = format!(r#"
// Functions for object construction

{class_name}* {create_func_name}(uint8_t* rustObj, {rust_proxy_type_name}* rustProxy)
{{
    return new {class_name}(rustObj, rustProxy);
}}

{class_name}* {create_at_func_name}(uint8_t* addr, uint8_t* rustObj, {rust_proxy_type_name}* rustProxy)
{{
    return new (addr) {class_name}(rustObj, rustProxy);
}}

const QMetaObject& {static_meta_func_name}()
{{
    return {base_name}::staticMetaObject;
}}

size_t {sizeof_func_name}()
{{
    return sizeof({class_name});
}}

size_t {alignof_func_name}()
{{
    return alignof({class_name});
}}

QMetaType {qmetatype_list_func_name}()
{{
    return QMetaType::fromType<QQmlListProperty<{class_name}>>();
}}
"#);

        (declaration, definition)
    }

}
