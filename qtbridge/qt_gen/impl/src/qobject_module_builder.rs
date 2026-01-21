use proc_macro2::TokenStream;
use qt_gen_common::type_qualified_mapping::CallOrigin;
use quote::{ToTokens, format_ident};
use syn::Generics;
use syn::spanned::Spanned;

use qt_gen_common::function_with_attributes::FunctionWithAttributes;
use qt_iface_gen_lib::{InterfaceDesc, InterfaceImpl, MethodOverride};
use qt_meta_gen::generate_meta::{QMetaInfoContext, generate_qmetainfo_trait_impl};
use qt_meta_gen::generate_qmetatype_interface_get::generate_qmeta_type_interface_get;
use qt_meta_gen::traits::{QmlName, find_duplicate_by_qml_name};
use qt_meta_gen::{ExpandTokens, QClassInfo, QPropertyInfo, QSignalInfo, QSlotInfo};

use crate::qobject_impl::{check_if_generated_functions_conflict_wth_client_code};
use crate::qobject_module_params::QObjectModuleParams;


pub struct QObjectModuleBuilder {
    origin: CallOrigin,
    struct_ident: syn::Ident,
    struct_generics: Generics,
    signals: Vec<QSignalInfo>,
    slots: Vec<QSlotInfo>,
    properties: Vec<QPropertyInfo>,
    overrides: Vec<MethodOverride>,
    class_infos: Vec<QClassInfo>,
    other_methods: Vec<syn::Signature>,
}


impl QObjectModuleBuilder {
    pub fn new(origin: CallOrigin) -> Self {
        Self {
            origin,
            struct_ident: format_ident!("dummy"),
            struct_generics: Generics::default(),
            signals: Vec::new(),
            slots: Vec::new(),
            properties: Vec::new(),
            overrides: Vec::new(),
            class_infos: Vec::new(),
            other_methods: Vec::new()
        }
    }

    pub fn build_token_stream(&mut self, input: TokenStream, params: TokenStream) -> syn::Result<TokenStream> {
        self.build(input, params)
            .map(|item_mod| item_mod.to_token_stream())
    }

    pub fn build(&mut self, input: TokenStream, params: TokenStream) -> syn::Result<syn::ItemMod> {
        // Parse input parameters of this macro,
        let params = syn::parse2::<QObjectModuleParams>(params)
            .map_err(|err| syn::Error::new(err.span(), format!("Failed to parse input params.\nError: {err}")))?;

        // Parse input token stream as 'mod' item,
        let module = syn::parse2::<syn::ItemMod>(input)
            .map_err(|err| syn::Error::new(err.span(), format!("Failed to parse input as mod block.\nError: {err}")))?;

        // Process input module. Generate output module (after expansion of the macro).
        // Extract signals, slots, properties, class infos, and other relevant parts.
        let mut output_module_items = self.handle_item_mod(&module)?;

        self.check_duplicates()?;

        // Try to deduce properties type here when we have list of potential getters/setters
        for prop in &mut self.properties {
            prop.set_type(&self.other_methods)?;
            prop.validate(&self.signals)
                .map_err(|err| syn::Error::new(err.span(), format!("Wrong property declaration: {err}")))?;
        }

        // Code generation for QObject interface implementation
        // Load interface from description file
        let iface_desc = match params.base() {
            Some(base) => InterfaceDesc::new_from_ident(base),
            None => InterfaceDesc::new_from_name_str("QObject"),
        }?;
        let iface_name = iface_desc.get_ident().to_string();

        // Generate the implementation of the interface.
        let iface_impl = InterfaceImpl::new(self.struct_ident.clone(), self.struct_generics.clone(), self.overrides.clone(), iface_desc, self.origin.clone())?;
        let unimpl_methods = iface_impl.get_unimplemented_pure_methods()?;
        if !unimpl_methods.is_empty() {
            // TODO: Check if suitable methods are in 'other_methods' but not marked as #[overridden]?
            return Err(syn::Error::new(iface_name.span(),
                format!("Some of pure virtual methods of interface '{iface_name}' are not overridden in '{}':\n{}",
                    self.struct_ident, unimpl_methods.join(", "))));
        }

        // Generate blocks of code that will be added to expanded code.
        let impl_details = iface_impl.generate_impl_details()
            .map_err(|err| syn::Error::new(err.span(), format!("Failed to generate implementation details block.\nError:{err}")))?;
        let qobject_funcs = iface_impl.generate_qobject_funcs()
            .map_err(|err| syn::Error::new(err.span(), format!("Failed to generate code block with auxiliary functions.\nError:{err}")))?;
        let iface_base_impl = iface_impl.generate_iface_base_impl()
            .map_err(|err| syn::Error::new(err.span(), format!("Failed to generate code block with base functions of given interface.\nError:{err}")))?;
        let iface_trait = iface_impl.generate_iface_trait_impl()
            .map_err(|err| syn::Error::new(err.span(), format!("Failed to generate code block with interface functions implementation.\nError:{err}")))?;

        let iface_base_func_names = iface_base_impl.items.iter()
            .filter_map(|item| match item {
                syn::ImplItem::Fn(item_fn) => Some(item_fn.sig.ident.to_string()),
                _ => None,
            })
            .collect::<Vec<_>>();

        if let Some(conflict) = check_if_generated_functions_conflict_wth_client_code(iface_base_func_names.iter(), &self.other_methods) {
            // TODO: check signal & slot functions in addition to other_methods
            return Err(syn::Error::new(conflict.span(), format!("Function generated for interface implementation conflicts with user function '{conflict}'")));
        }

        // TODO: pass QObjectModule to generate_qmetainfo_trait_impl() instead
        let ctx = QMetaInfoContext {
            struct_ident: &self.struct_ident,
            generics: &self.struct_generics,
            cpp_iface_name: &iface_name,
            signals: &self.signals,
            slots: &self.slots,
            properties: &self.properties,
            class_infos: &self.class_infos
        };

        // Generate traits code.
        let qmeta_info_impl_tokens = generate_qmetainfo_trait_impl(&ctx, &self.origin)
            .map_err(|err| syn::Error::new(err.span(), format!("Failed to generate implementation of QMetaInfo trait.\nError: {}", err)))?;
        let qmetatype_iface_get_impl_tokens = generate_qmeta_type_interface_get(&self.struct_ident, &self.struct_generics, &self.origin)
            .map_err(|err| syn::Error::new(err.span(), format!("Failed to generate implementation of QMetaTypeInterfaceGet trait.\nError: {}", err)))?;

        // Concat additional items to the source items processed
        output_module_items.push(iface_base_impl.into());                  // impl block with the base functions
        output_module_items.push(iface_trait.into());                      // Rust implementation of C++ interface methods
        output_module_items.push(qobject_funcs.into());                    // Impl block with functions needed to attach, detach and reference QObject
        // TODO: return items below as high level AST but not TokenStreams
        output_module_items.push(syn::parse2(qmeta_info_impl_tokens)?);             // impl qtbridge::bridge::QMetaInfo
        output_module_items.push(syn::parse2(qmetatype_iface_get_impl_tokens)?);    // impl qtbridge::qt_type_lib::QMetaTypeInterfaceGet

        // TODO: this is not very elegant. We should probably return Vec<Item> from the function generating this code
        let file: syn::File = syn::parse2(impl_details)?;
        output_module_items.extend(file.items);                                // Functionality called from implementation internals

        Ok(syn::ItemMod {
            content: Some((syn::token::Brace::default(), output_module_items)),
            ..module.clone()
        })
    }

    fn handle_item_mod(&mut self, input: &syn::ItemMod) -> syn::Result<Vec<syn::Item>> {
        // Get items contained in the given mod block.
        let (_brace, mod_items) = input.content.as_ref()
            .ok_or_else(|| syn::Error::new(input.span(), "mod expected to contain a code block in curly brackets"))?;

        // Filter items with struct declaration.
        let struct_items: Vec<&syn::ItemStruct> = mod_items.iter()
            .filter_map(|item| match item {
                syn::Item::Struct(struct_) => Some(struct_),
                _ => None,
            })
            .collect();
        let struct_ = struct_items.first()
            .ok_or_else(|| syn::Error::new(input.span(), "mod annotation with #[qobject] must contain a struct"))?;
        if let Some(second_struct) = struct_items.get(1) {
            return Err(syn::Error::new(second_struct.span(), "Only single struct is allowed in module annotated with #[qobject]"))
        }
        self.struct_ident = struct_.ident.clone();
        self.struct_generics = struct_.generics.clone();

        // Process input items of module. Expand item to output module
        let mut output_items = Vec::new();
        for item in mod_items.iter() {
            output_items.push(self.handle_item(item)?);
        }

        Ok(output_items)
    }

    fn handle_item(&mut self, input: &syn::Item) -> syn::Result<syn::Item> {
        match input {
            syn::Item::Impl(item_impl) => {
                let result = match &item_impl.trait_ {
                    Some(_) => self.handle_item_impl_trait(item_impl),
                    None => self.handle_item_impl_struct(item_impl),
                }?;
                Ok(result.into())
            }
            _ => Ok(input.clone()),
        }
    }

    /// Handle block
    /// impl SomeTrait for SomeStruct
    fn handle_item_impl_trait(&mut self, input: &syn::ItemImpl) -> syn::Result<syn::ItemImpl> {
        // Will be handled later
        Ok(input.clone())
    }

    /// Handle block
    /// impl SomeStruct
    fn handle_item_impl_struct(&mut self, input: &syn::ItemImpl) -> syn::Result<syn::ItemImpl> {
        let mut output_items = Vec::new();

        for input_item in &input.items {
            let opt_output_item = self.handle_impl_item(input_item)
                .map_err(|err |syn::Error::new(err.span(), format!("Failed to process ImplItem.\nError: {err}")))?;
            let Some(output_item) = opt_output_item else {
                continue
            };
            output_items.push(output_item);
        }

        Ok(syn::ItemImpl {
            items: output_items,
            ..input.clone()
        })
    }

    fn handle_impl_item(&mut self, input: &syn::ImplItem) -> syn::Result<Option<syn::ImplItem>> {
        match input {
            syn::ImplItem::Fn(item_fn) =>
                self.handle_impl_item_fn(item_fn)
                    .map(|opt_fn| opt_fn.map(syn::ImplItem::from)),
            syn::ImplItem::Macro(item_macro) =>
                self.handle_impl_item_macro(item_macro)
                    .map(|opt_macro| opt_macro.map(syn::ImplItem::from)),
            syn::ImplItem::Verbatim(tokens) =>
                self.handle_impl_item_verbatim(tokens)
                    .map(|opt_tokens| opt_tokens.map(syn::ImplItem::Verbatim)),
            _ => Ok(Some(input.clone())),
        }
    }

    fn handle_impl_item_fn(&mut self, input: &syn::ImplItemFn) -> syn::Result<Option<syn::ImplItemFn>> {
        if input.attrs.is_empty() {
            // No attributes - then it is not a slot, signal or method
            self.other_methods.push(input.sig.clone());
            return Ok(Some(input.clone()))
        }

        let Some(meta_attr) = Self::get_meta_attribute(&input.attrs)? else {
            return Ok(Some(input.clone()))
        };
        let func = syn::parse2::<FunctionWithAttributes>(input.to_token_stream())?;

        let output;
        if QSlotInfo::is_for_me(meta_attr) {
            let slot = QSlotInfo::new(func)?;
            output = slot.expand_tokens()?;
            self.slots.push(slot);
        }
        else if QSignalInfo::is_for_me(meta_attr) {
            let signal = QSignalInfo::new(func, &self.origin)?;
            output = signal.expand_tokens()?;
            self.signals.push(signal);
        }
        else if MethodOverride::is_for_me(meta_attr) {
            let method = MethodOverride::new(func)?;
            output = method.expand_tokens()?;
            self.overrides.push(method);
        } else {
            unreachable!()
        }

        Ok(Some(syn::parse2(output)?))
    }

    fn handle_impl_item_macro(&mut self, input: &syn::ImplItemMacro) -> syn::Result<Option<syn::ImplItemMacro>> {
        let name = input.mac.path
            .get_ident()
            .map(syn::Ident::to_string)
            .unwrap_or_default();
        match name.as_str() {
            "qproperty" => {
                let property = QPropertyInfo::new(input)?
                    .ok_or_else(|| syn::Error::new(input.span(), "Not a qproperty"))?;
                self.properties.push(property);
                Ok(None)
            }
            "qclass_info" => {
                let class_info = QClassInfo::new(input)?
                    .ok_or_else(|| syn::Error::new(input.span(), "Not a ClassInfo"))?;
                self.class_infos.push(class_info);
                Ok(None)
            }
            _ => Ok(Some(input.clone()))
        }
    }

    fn handle_impl_item_verbatim(&mut self, input: &TokenStream) -> syn::Result<Option<TokenStream>> {
        let func = syn::parse2::<FunctionWithAttributes>(input.clone())?;
        let Some(meta_attr) = Self::get_meta_attribute(&func.attrs)? else {
            return Ok(Some(input.clone()))
        };

        if QSignalInfo::is_for_me(meta_attr) {
            let signal = QSignalInfo::new(func, &self.origin)?;
            let output = signal.expand_tokens()?;
            self.signals.push(signal);
            return Ok(Some(output))
        }

        Ok(Some(input.clone()))
    }

    fn get_meta_attribute(input: &[syn::Attribute]) -> syn::Result<Option<&syn::Attribute>> {
        let meta_attrs: Vec<_> = input.iter()
            .filter(|attr|
                QSlotInfo::is_for_me(attr) ||
                QSignalInfo::is_for_me(attr) ||
                MethodOverride::is_for_me(attr)
            )
            .take(2)
            .collect();
        match meta_attrs.len() {
            0 => Ok(None),
            1 => Ok(Some(meta_attrs[0])),
            2.. => {
                let second = meta_attrs[1];
                Err(syn::Error::new(second.span(), "The meta attribute conflicts with an attribute above"))
            }
        }
    }

    fn check_duplicates(&self) -> syn::Result<()> {
        if let Some(dup) = find_duplicate_by_qml_name(&self.signals) {
            let (name, span) = dup.get_qml_name_span();
            return Err(syn::Error::new(span, format!("Signal '{name}' was already declared")));
        }

        if let Some(dup) = find_duplicate_by_qml_name(&self.slots) {
            let (name, span) = dup.get_qml_name_span();
            return Err(syn::Error::new(span, format!("Slot '{name}' was already declared")));
        }

        if let Some(dup) = find_duplicate_by_qml_name(&self.properties) {
            let (name, span) = dup.get_qml_name_span();
            return Err(syn::Error::new(span, format!("Property '{name}' was already declared")));
        }

        Ok(())
    }
}
