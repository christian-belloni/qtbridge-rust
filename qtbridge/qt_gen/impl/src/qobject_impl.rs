// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::collections::BTreeSet;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Ident};

use qt_gen_common::parse_utils::parse_name_value;
use qt_gen_common::function_with_attributes::FunctionWithAttributes;
use qt_gen_common::type_qualified_mapping::CallOrigin;
use qt_iface_gen_lib::{InterfaceDesc, InterfaceImpl, MethodOverride};
use qt_meta_gen::generate_meta::{generate_qmetainfo_trait_impl, QMetaInfoContext};
use qt_meta_gen::generate_qmetatype_interface_get::generate_qmeta_type_interface_get;
use qt_meta_gen::traits::{ExpandTokens, QmlName, find_duplicate_by_qml_name};
use qt_meta_gen::{QClassInfo, QPropertyInfo, QSignalInfo, QSlotInfo};

pub struct QObjectImplOutput {
    /// Content of 'impl' block after the macro expansion (Qt-specific annotations removed, signals expanded, etc)
    pub new_impl: TokenStream,

    /// 'impl' block containing base functions from C++ interface (both virtual and not)
    pub iface_base_impl: syn::ItemImpl,

    // implementation of trait consisting of virtual methods of C++ interface
    pub iface_trait: syn::ItemImpl,

    /// 'impl' block containing functions to attach/detach/access proxies
    pub qobject_funcs: syn::ItemImpl,

    /// Implementation of QMetaInfo trait
    pub qmeta_info_impl: TokenStream,

    /// Implementation of QMetaTypeInterfaceGet trait
    pub qmetatype_iface_get_impl: TokenStream,

    /// Implementation details
    pub impl_details: TokenStream,
}

impl QObjectImplOutput {
    // Implement as regular function but not as ToTokens trait
    // not to add a 'quote' dependency to qt_gen project
    pub fn to_token_stream(&self) -> TokenStream {
        let Self{ new_impl, iface_base_impl, iface_trait, qobject_funcs, qmeta_info_impl, qmetatype_iface_get_impl, impl_details } = &self;

        let iface_base_impl = (!iface_base_impl.items.is_empty())
            .then(|| iface_base_impl.to_token_stream());

        quote!{
            #new_impl
            #iface_base_impl
            #iface_trait
            #qobject_funcs
            #qmeta_info_impl
            #qmetatype_iface_get_impl
            #impl_details
        }
    }
}

/// Expand #[qobject_impl] macro applied to 'impl' block of a struct.
/// Handle annotations of Qt signals, slots, properties.
/// Make struct behave as if it was 'inherited' from given QObject interface (optionally).
/// Generate code needed to make it work.
pub fn qobject_impl(input: TokenStream, params: TokenStream, origin: &CallOrigin) -> syn::Result<QObjectImplOutput> {
    // Parsing input parameters of this macro
    let params = syn::parse2::<QObjectImplParams>(params)
        .map_err(|err| syn::Error::new(err.span(), format!("Failed to parse input params.\nError: {err}")))?;

    // Parsing code the macro is applied to
    let orig_impl = syn::parse2::<syn::ItemImpl>(input.clone())
        .map_err(|err| syn::Error::new(err.span(), format!("Failed to parse input as ItemImpl.\nError: {err}")))?;

    let syn::Type::Path(type_path) = orig_impl.self_ty.as_ref() else {
        return Err(syn::Error::new(orig_impl.span(), "Unexpected type of impl struct"));
    };
    let struct_ident = type_path.path.segments.last()
        .ok_or_else(|| syn::Error::new(type_path.path.span(), "Failed to get last segment from path"))?
        .ident.clone();
    let generics = &orig_impl.generics;

    let mut signals    = Vec::<QSignalInfo>::new();
    let mut slots      = Vec::<QSlotInfo>::new();
    let mut properties = Vec::<QPropertyInfo>::new();
    let mut overrides  = Vec::<MethodOverride>::new();
    let mut items_out  = Vec::<syn::ImplItem>::new();
    let mut class_infos = Vec::<QClassInfo>::new();

    let mut other_methods = Vec::<syn::Signature>::new(); // Methods that are not signal or slot, but potentially can be property setter/getter

    for item in &orig_impl.items {
        let qmeta_item = match extract_qobject_item(&item, origin) {
            Ok(s) => s,
            Err(err) => return Err(syn::Error::new(err.span(), format!("Failed to process item of 'impl' block. Error: {}", err))),
        };

        let mut item_out_tokens = TokenStream::new();
        match qmeta_item {
            Some(QObjectImplItem::Signal(signal)) => {
                item_out_tokens = signal.expand_tokens()?;
                signals.push(signal);
            },
            Some(QObjectImplItem::Slot(slot)) => {
                item_out_tokens = slot.expand_tokens()?;
                slots.push(slot);
            },
            Some(QObjectImplItem::Property(property)) => {
                properties.push(property);
            },
            Some(QObjectImplItem::Override(method)) => {
                item_out_tokens = method.expand_tokens()?;
                overrides.push(method);
            },
            Some(QObjectImplItem::ClassInfo(class_info)) => {
                class_infos.push(class_info);
            },
            None => {
                items_out.push(item.clone());
                if let syn::ImplItem::Fn(item_fn) = item {
                    other_methods.push(item_fn.sig.clone());
                }
            },
        }

        if !item_out_tokens.is_empty() {
            match syn::parse2::<syn::ImplItem>(item_out_tokens) {
                Ok(item) => items_out.push(item),
                Err(err) => return Err(syn::Error::new(err.span(), format!("Failed to get updated ItemImpl. Error: {}", err))),
            }
        }
    }

    check_duplicates(&signals, &slots, &properties)?;

    // Try to deduce properties type here when we have list of potential getters/setters
    for prop in &mut properties {
        prop.set_type(&other_methods)?;
        if let Err(err) = prop.validate(&signals) {
            return Err(syn::Error::new(err.span(), format!("Wrong property declaration: {}", err)));
        }
    }

    // Code generation for QObject interface implementation
    // Load interface from description file
    let (iface_name, iface_desc) = if let Some(base) = params.base.as_ref() {
        (base.to_string(), InterfaceDesc::new_from_ident(base)?)
    }
    else {
        ("QObject".to_owned(), InterfaceDesc::new_from_name_str("QObject")?)
    };

    let iface_impl = InterfaceImpl::new(struct_ident.clone(), generics.clone(), overrides, iface_desc, origin.clone())?;
    let unimpl_methods = iface_impl.get_unimplemented_pure_methods()?;
    if !unimpl_methods.is_empty() {
        // TODO: Check if suitable methods are in 'other_methods' but not marked as #[overridden]?
        return Err(syn::Error::new(iface_name.span(), format!("Some of pure virtual methods of interface '{}' are not overridden in '{}':\n{}", iface_name, struct_ident, unimpl_methods.join(", "))));
    }

    let impl_details = iface_impl.generate_impl_details()
        .map_err(|err| syn::Error::new(err.span(), format!("Failed to generate implementation details block.\nError:{err}")))?;
    let qobject_funcs = iface_impl.generate_qobject_funcs()
        .map_err(|err| syn::Error::new(err.span(), format!("Failed to generate code block with auxiliary functions.\nError:{err}")))?;
    let iface_base_impl = iface_impl.generate_iface_base_impl()?;
    let iface_trait = iface_impl.generate_iface_trait_impl()?;

    let iface_base_func_names = iface_base_impl.items.iter()
        .filter_map(|item| match item {
            syn::ImplItem::Fn(item_fn) => Some(item_fn.sig.ident.to_string()),
            _ => None,
        })
        .collect::<Vec<_>>();

    if let Some(conflict) = check_if_generated_functions_conflict_wth_client_code(iface_base_func_names.iter(), &other_methods) {
        // TODO: check signal & slot functions in addition to other_methods
        return Err(syn::Error::new(conflict.span(), format!("Function generated for interface implementation conflicts with user function '{conflict}'")));
    }
    let ctx = QMetaInfoContext {
        struct_ident: &struct_ident,
        generics: &generics,
        cpp_iface_name: &iface_name,
        signals: &signals,
        slots: &slots,
        properties: &properties,
        class_infos: &class_infos
    };

    // Generate traits code
    let qmeta_info_impl = generate_qmetainfo_trait_impl(&ctx, &origin)
        .map_err(|err| syn::Error::new(err.span(), format!("Failed to generate implementation of QMetaInfo trait.\nError: {}", err)))?;
    let qmetatype_iface_get_impl = generate_qmeta_type_interface_get(&struct_ident, &generics, &origin)
        .map_err(|err| syn::Error::new(err.span(), format!("Failed to generate implementation of QMetaTypeInterfaceGet trait.\nError: {}", err)))?;

    // Prepare altered input token stream
    let new_impl = syn::ItemImpl{ items: items_out, ..orig_impl }
        .to_token_stream();

    Ok(QObjectImplOutput {
        new_impl,
        iface_base_impl,
        iface_trait,
        qobject_funcs,
        qmeta_info_impl,
        qmetatype_iface_get_impl,
        impl_details,
    })
}

struct QObjectImplParams {
    base: Option<syn::Ident>,
}

mod qobject_impl_params_keywords {
    syn::custom_keyword!(Base);
}

impl syn::parse::Parse for QObjectImplParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut base = None;

        while !input.is_empty() {
            if input.peek(qobject_impl_params_keywords::Base) {
                base = Some(parse_name_value::<Ident, Ident>(input)?.1);
            } else {
                return Err(input.error("Unsupported attribute of qobject_impl macro"));
            }
        }
        Ok(QObjectImplParams {
            base,
        })
    }
}


pub(crate) enum QObjectImplItem {
    Signal(QSignalInfo),
    Slot(QSlotInfo),
    Property(QPropertyInfo),
    Override(MethodOverride),
    ClassInfo(QClassInfo)
}

/// Returns Result with parsed QSignalInfo/QSlotInfo/QProperty/Overridden method (if found)
fn extract_qobject_item(item_in: &syn::ImplItem, origin: &CallOrigin) -> syn::Result<Option<QObjectImplItem>> {
    // TODO: more code validating signal/slot function signature?

    match &item_in {
        syn::ImplItem::Fn(item_fn) => {
            let mut result = None;

            if !item_fn.attrs.is_empty() {
                let func = syn::parse2::<FunctionWithAttributes>(item_fn.to_token_stream())?;
                let attrs = func.attrs.clone();

                for (idx, attr) in attrs.iter().enumerate() {
                    if QSlotInfo::is_for_me(attr) {
                        result = Some(QObjectImplItem::Slot(QSlotInfo::new(func)?));
                    }
                    else if QSignalInfo::is_for_me(attr) {
                        result = Some(QObjectImplItem::Signal(QSignalInfo::new(func, origin)?))
                    }
                    else if MethodOverride::is_for_me(attr) {
                        result = Some(QObjectImplItem::Override(MethodOverride::new(func)?));
                    } else {
                        continue;
                    }

                    for attr in attrs.iter().skip(idx+1) {
                        if QSignalInfo::is_for_me(attr) || QSlotInfo::is_for_me(attr) || MethodOverride::is_for_me(attr) {
                            return Err(syn::Error::new(attr.span(), "The attribute conflicts with function attribute above"));
                        }
                    }
                    break;
                }
            }

            Ok(result)
        },

        syn::ImplItem::Macro(item_macro) => {
            let ident_option: Option<String> = item_macro.mac.path.get_ident().map(|i| i.to_string());
                match ident_option.as_deref() {
                    Some("qproperty") => {
                        let prop = QPropertyInfo::new(item_macro)?;
                        Ok(prop.map(QObjectImplItem::Property))
                    },
                    Some("qclass_info") => {
                        let class_info = QClassInfo::new(item_macro)?;
                        Ok(class_info.map(QObjectImplItem::ClassInfo))
                    },
                    _ => Ok(None),
                }
        },

        syn::ImplItem::Verbatim(verb_tokens) => {
            let func = syn::parse2::<FunctionWithAttributes>(verb_tokens.clone())?;
            if func.attrs.iter().any(QSignalInfo::is_for_me) {
                return Ok(Some(QObjectImplItem::Signal(QSignalInfo::new(func, origin)?)))
            } else {
                Ok(None)
            }
        },

        _ => Ok(None),
    }
}

fn check_duplicates(signals: &[QSignalInfo], slots: &[QSlotInfo], properties: &[QPropertyInfo]) -> syn::Result<()> {
    if let Some(dup) = find_duplicate_by_qml_name(signals) {
        let (name, span) = dup.get_qml_name_span();
        return Err(syn::Error::new(span, format!("Signal '{name}' was already declared")));
    }

    if let Some(dup) = find_duplicate_by_qml_name(slots) {
        let (name, span) = dup.get_qml_name_span();
        return Err(syn::Error::new(span, format!("Slot '{name}' was already declared")));
    }

    if let Some(dup) = find_duplicate_by_qml_name(properties) {
        let (name, span) = dup.get_qml_name_span();
        return Err(syn::Error::new(span, format!("Property '{name}' was already declared")));
    }

    Ok(())
}

pub(crate) fn check_if_generated_functions_conflict_wth_client_code<'a>(
    generated_names: impl Iterator<Item = &'a String>,
    client_funcs: &Vec<syn::Signature>
) -> Option<&syn::Ident> {

    let generated_names = BTreeSet::from_iter(generated_names);
    for sig in client_funcs {
        let sig_ident = &sig.ident;
        if generated_names.contains(&sig_ident.to_string()) {
            return Some(sig_ident);
        }
    }

    None
}
