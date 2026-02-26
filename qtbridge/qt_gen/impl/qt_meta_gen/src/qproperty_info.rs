// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{parse::Parse, spanned::Spanned};

use qt_gen_common::parse_utils::parse_name_value;
use qt_gen_common::type_registry::meta_types::get_qmetatype_support_for_type;
use qt_gen_common::type_to_string::type_to_string_fallback;
use qt_gen_common::type_utils::{ValuePass, get_take_value_code, get_type_pass, remove_ref, remove_ref_to_string};
use crate::qproperty_type_deduction::{deduce_type_from_getter, deduce_type_from_member, deduce_type_from_setter};
use crate::QSignalInfo;
use crate::traits::{QmlName, find_by_qml_name};

pub struct QPropertyInfo{
    name: syn::LitStr,
    span: Span,
    read_method: Option<syn::Ident>,
    write_method: Option<syn::Ident>,
    notify_signal: Option<syn::LitStr>,
    member: Option<syn::Ident>,
    constant: Option<syn::Ident>,

    /// The type of the property, deduced from getter, setter or member variable. TODO: don't use string here
    deduced_type: Option<syn::Type>,

    /// How the value is passed to the setter (by reference or by value)
    write_value_pass: Option<ValuePass>,
}

impl QPropertyInfo {
    pub fn new(item: &syn::ImplItemMacro) -> syn::Result<Option<Self>> {
        if !item.mac.path.is_ident("qproperty") {
            return Ok(None); // Not a 'qproperty!' macro
        }

        if let Some(first_attr) = item.attrs.first() {
            return Err(syn::Error::new(first_attr.span(), "Attributes for qproperty! macro are not supported"));
        }

        let mut prop = item.mac.parse_body::<QPropertyInfo>()?;
        prop.span = item.mac.span(); // Change span to include whole macro
        Ok(Some(prop))
    }

    pub fn is_read_only(&self) -> bool {
        self.write_method.is_none() && self.member.is_none()
    }

    pub fn is_const(&self) -> bool {
        self.constant.is_some()
    }

    pub fn is_type_deduced_from_member(&self) -> bool {
        self.read_method.is_none() &&
        self.write_method.is_none() &&
        self.member.is_some()
    }

    pub fn get_notify_signal(&self) -> Option<&syn::LitStr> {
        self.notify_signal.as_ref()
    }

    fn get_deduced_type(&self) -> Option<&syn::Type> {
        self.deduced_type
            .as_ref()
    }

    pub fn validate(&self, signals: &[QSignalInfo]) -> syn::Result<()> {
        self.validate_name()?;

        if self.member.is_some() && self.read_method.is_some() && self.write_method.is_some() {
            return Err(syn::Error::new(self.member.span(), "qproperty must not have 'member' flag if both Read and Write functions are set"));
        }

        if self.constant.is_some() {
            if self.write_method.is_some() {
                return Err(syn::Error::new(self.write_method.span(), "Constant qproperty must not have Write method"));
            }
            if self.notify_signal.is_some() {
                return Err(syn::Error::new(self.notify_signal.span(), "Constant qproperty must not have Notify signal"));
            }
        }

        if let Some(notify_signal) = self.notify_signal.as_ref() {
            let notify_signal_name = notify_signal.value();
            if !notify_signal_name.is_empty() {
                let signal = find_by_qml_name(&notify_signal_name, signals)
                    .ok_or_else(|| syn::Error::new(notify_signal.span(), format!("Signal '{}' not found", notify_signal_name)))?;
                match signal.get_typed_arg_count() {
                    0 => {}
                    1 => {
                        if let Some(prop_type) = self.get_deduced_type() {
                            let prop_type_str = remove_ref_to_string(prop_type)?;
                            let signal_type_str = remove_ref_to_string(signal.get_arg_type(0)?)?;
                            if prop_type_str != signal_type_str {
                                return Err(syn::Error::new(notify_signal.span(), format!("Property/signal types mismatch: '{prop_type_str}' and '{signal_type_str}'")));
                            }
                        }
                    }
                    _ => return Err(syn::Error::new(notify_signal.span(), "Notify signal of a property must have either 2 arguments (&self and value) or only 1 (&self)"))
                }
            }
        };

        Ok(())
    }

    fn validate_name(&self) -> syn::Result<()> {
        // https://doc.qt.io/qt-6/qtqml-syntax-objectattributes.html#defining-property-attributes

        let name = self.name.value();
        let first_ch = name.chars().next()
            .ok_or_else(|| syn::Error::new(name.span(), "Property name is empty"))?;

        if !first_ch.is_lowercase() {
            return Err(syn::Error::new(name.span(), format!("Property name must begin with a lower case letter. Found: '{first_ch}'")))
        }

        for ch in name.chars() {
            if !ch.is_ascii_alphanumeric() && ch != '_' {
                return Err(syn::Error::new(name.span(), format!("Illegal char in property name: '{ch}'")))
            }
        }

        Ok(())
    }

    // Sets the property type based on the type deduced from its accessors or member variable.
    pub fn set_type(&mut self, methods: &[syn::Signature], fields: Option<&syn::FieldsNamed>) -> syn::Result<()> {
        let mut deduced = Vec::new(); // Array of tuples (Type, Span, "deduced from")

        if let Some(getter) = self.read_method.as_ref() {
            let getter_type = deduce_type_from_getter(getter, methods)?;
            deduced.push((getter_type, getter.span(), "getter"));
        }

        self.write_value_pass = None;
        if let Some(setter) = self.write_method.as_ref() {
            let setter_type = deduce_type_from_setter(setter, methods)?;
            deduced.push((setter_type, setter.span(), "setter"));
            self.write_value_pass = Some(get_type_pass(setter_type));
        }

        if let Some(member) = self.member.as_ref() &&
           let Some(fields) = fields {
                let member_type = deduce_type_from_member(member, fields.named.iter())?;
                deduced.push((member_type, member.span(), "member"));
            }

        let Some((first_type, _, first_src)) = deduced.first() else {
            return Ok(())
        };

        if let Some((second_type, second_span, second_src)) = deduced.get(1) {
            let first_type_no_ref = remove_ref(first_type);
            let second_type_no_ref = remove_ref(second_type);
            if first_type_no_ref != second_type_no_ref {
                return Err(syn::Error::new(*second_span,
                    format!("Property types deduced from '{first_src}' and '{second_src}' are inconsistent: '{}' vs '{}'",
                        type_to_string_fallback(first_type_no_ref),
                        type_to_string_fallback(second_type_no_ref)
                    )));
            }
        }

        self.deduced_type = Some((*first_type).clone());
        Ok(())
    }

    pub fn get_meta_registration_code(&self, struct_ident: &syn::Ident, signal: Option<&QSignalInfo>) -> syn::Result<TokenStream> {

        let QPropertyInfo {
            name,
            span,
            notify_signal,
            member,
            deduced_type,
            ..
        } = self;

        let metatype_expr = match &deduced_type {
            // Type of property is deduced from getter, setter or member (works for #[qobject] but not for #[qobject_impl] macro)
            Some(ty) => {
                let meta_type = get_qmetatype_support_for_type(&ty)
                    .map_err(|err| syn::Error::new(*span, format!("Type '{}' of qproperty can not be mapped to qt.\nError: {err}", type_to_string_fallback(ty))))?
                    .unwrap_or_else(|| ty.clone());
                quote! {
                    #meta_type::get_qmetatype()
                }
            },
            // Get an expression that will determine metatype using runtime function
            // (but hopefully will be inlined by the compiler).
            // Will be removed if we switch to #[qobject].
            None => {
                let member_var = member.as_ref()
                    .ok_or_else(||syn::Error::new(*span, "Can't deduce type of qproperty neither from accessor nor from member"))?;
                quote!{
                    QMetaType::new(get_meta_type_id_of_fn_return_value(|this: &Self| { &this.#member_var } ) as i32)
                }
            }
        };

        // TODO: check if signal with given name was declared
        let signal_name = notify_signal.as_ref()
            .map_or(String::default(), |i| i.value());

        if signal_name.is_empty() != signal.is_none() {
            return Err(syn::Error::new(*span, "Error in signal handling logic. Signal name mismatch"));
        }

        let read_callback = self.generate_read_callback(struct_ident)?;
        let write_callback = self.generate_write_callback(struct_ident, signal)?;

        if let Some(write_callback) = write_callback {
            Ok(quote!{
                meta_obj.as_mut().register_property(#name, &(#metatype_expr), #read_callback, #write_callback, #signal_name);
            })
        }
        else {
            let is_const = self.is_const();
            Ok(quote!{
                meta_obj.as_mut().register_property_read_only(#name, &(#metatype_expr), #read_callback, #is_const, #signal_name);
            })
        }
    }

    fn generate_read_callback(&self, struct_ident: &syn::Ident) -> syn::Result<TokenStream> {
        let read_code = if let Some(getter_fn) = &self.read_method {
            quote!{ this.#getter_fn().into() }
        }
        else if let Some(member) = &self.member {
            quote! { (&this.#member).into() }
        }
        else {
            return Err(syn::Error::new(self.span, "Neither 'Read' nor 'Member' is specified for property"));
        };

        Ok(quote! {
            property_read_callback_for::<#struct_ident>(|this| {
                #read_code
            })
        })
    }

    fn generate_write_callback(&self, struct_ident: &syn::Ident, signal: Option<&QSignalInfo>) -> syn::Result<Option<TokenStream>> {
        if self.is_const() {
            return Ok(None)
        }

        let name = &self.name;

        if let Some(setter_fn) = &self.write_method {
            // Generate write callback that calls given setter
            let ty = self.get_deduced_type()
                .ok_or_else(|| syn::Error::new(self.span, "Failed to generate write property callback. Type is not deduced"))?;
            let ty_str = remove_ref_to_string(ty)?;
            let pass_arg = get_take_value_code(&format_ident!("value"), self.write_value_pass.unwrap_or(ValuePass::ByValue));

            return Ok(Some(quote! {
                property_write_callback_for::<#struct_ident>(|this, value| {
                    let Ok(value) = TryInto::try_into(value) else {
                        panic!("Failed to convert value '{}' to type '{}' in qproperty '{}'", value.to_string(), #ty_str, #name);
                    };
                    this.#setter_fn(#pass_arg);
                })
            }))
        }

        if let Some(member) = &self.member {
            // Generate write callback that assigns value to given member variable
            // and emits signal is needed
            let signal_name = self.notify_signal
                .as_ref()
                .map_or(String::new(), |signal| signal.value());
            let write_code = if signal_name.is_empty() {
                // TODO: still auto generate name for signal otherwise? E.g. On{property_name}Changed
                quote!{
                    this.#member = value;
                }
            }
            else {
                let signal = signal.as_ref().unwrap();
                if signal_name != signal.get_qml_name_span().0 {
                    return Err(syn::Error::new(self.span, "Error in signal handling logic. Inconsistent signal name"));
                }
                let signal_name_ident = signal.get_rust_name();
                let mut signal_arg = None;
                if signal.get_typed_arg_count() > 0 {
                    signal_arg = match get_type_pass(signal.get_arg_type(0)?) {
                        ValuePass::ByValue => Some(quote! { this.#member.clone() }),
                        ValuePass::ByConstReference => Some(quote! { &this.#member }),
                        ValuePass::ByMutReference => Some(quote! { &mut this.#member }),
                    };
                }

                quote! {
                    if this.#member != value {
                        this.#member = value;
                        this.#signal_name_ident(#signal_arg);
                    }
                }
            };

            Ok(Some(quote!{
                property_write_callback_for::<#struct_ident>(|this, value| {
                    let Ok(value) = value.try_into() else {
                        panic!("Failed to convert value '{}' to type '{}' in qproperty '{}'", value.to_string(), std::any::type_name_of_val(&this.#member), #name);
                    };
                    #write_code
                })
            }))
        }
        else {
            Ok(None)
        }
    }
}

mod qproperty_keywords {
    syn::custom_keyword!(Read);
    syn::custom_keyword!(Write);
    syn::custom_keyword!(Notify);
    syn::custom_keyword!(Constant);
    syn::custom_keyword!(Member);
}

impl syn::parse::Parse for QPropertyInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let span = input.span();

        // TODO: charset check?
        let name = input.parse()
            .map_err(|err| syn::Error::new(err.span(), format!("Failed to get qproperty name.\nError: {err}")))?;

        let mut read_method = None;
        let mut write_method = None;
        let mut notify_signal = None;
        let mut member = None;
        let mut constant = None;

        while !input.is_empty() {
            let token_begin = input.fork();
            input.parse::<syn::Token![,]>()
                .map_err(|_err| token_begin.error("Expected comma while parsing qproperty attributes"))?;

            if input.is_empty() {
                break; // Stop after a trailing coma
            }

            // Try to parse attribute name-value pair. Like
            //     Name = Value
            if input.peek(qproperty_keywords::Read) {
                read_attribute(input, &mut read_method, "Read")?;
            }
            else if input.peek(qproperty_keywords::Write) {
                read_attribute(input, &mut write_method, "Write")?;
            }
            else if input.peek(qproperty_keywords::Notify) {
                read_attribute(input, &mut notify_signal, "Notify")?;
            }
            else if input.peek(qproperty_keywords::Member) {
                read_attribute(input, &mut member, "Member")?;
            }
            // Try to parse bool flags
            else if input.peek(qproperty_keywords::Constant) {
                constant = Some(input.parse()?);
            }
            else {
                let attr: syn::Ident = input.parse()?;
                return Err(syn::Error::new(
                    attr.span(),
                    format!("Unsupported qproperty attribute: {}", attr),
                ));
            }
        }

        Ok(QPropertyInfo {
            name,
            span,
            read_method,
            write_method,
            notify_signal,
            member,
            constant,
            deduced_type: None,               // Property type is not clear at the moment of parsing. Will be deduced later
            write_value_pass: None,
        })
    }

}

fn read_attribute<T: Parse>(input: syn::parse::ParseStream, dst: &mut Option<T>, name: &'static str) -> syn::Result<()> {
    if dst.is_some() {
        return Err(syn::Error::new(input.span(), format!("'{name}' attribute is already defined for the property")));
    }

    *dst = Some(parse_name_value::<syn::Ident, T>(&input)?.1);

    Ok(())
}

impl QmlName for QPropertyInfo {
    fn get_qml_name_span(&self) -> (String, proc_macro2::Span) {
        (self.name.value(), self.name.span())
    }
}
