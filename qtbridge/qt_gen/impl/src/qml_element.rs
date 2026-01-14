use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

 use qt_gen_common::naming;

pub fn qml_element(args: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    let (item_ts, struct_ident) = get_struct_or_impl_info(input)?;
    let mut is_singleton = false;
    if !args.is_empty() {
        let args_ident: syn::Ident = syn::parse2(args)?;
        if args_ident != "singleton" {
            return Err(syn::Error::new_spanned(
                args_ident,
                "Unexpected syntax of #[qml_element] macro attributes",
            ));
        }
        is_singleton = true;
    }
    let qml_register_fn_indent = format_ident!("qml_register_{struct_ident}");
    let struct_name = struct_ident.to_string();

    let constructor_body = build_constructor_body(&struct_ident, is_singleton);
    let qml_register_call = build_qml_register_call(&struct_name, is_singleton);

    let qmlregister_code = quote! {
        // TODO: make auto registration via 'linkme' dependency an optional cargo feature?
        #[linkme::distributed_slice(qtbridge::qt_type_lib::QML_REGISTER_CALLBACKS)]
        #[allow(non_camel_case_types)]
        fn #qml_register_fn_indent() {
            <#struct_ident as qtbridge::bridge::QmlRegister>::qml_register();
        }

        impl qtbridge::bridge::QmlRegister for #struct_ident {
             fn qml_register() {
                #constructor_body

                let meta_obj_data = <Self as qtbridge::bridge::QMetaInfo>::get_shared_dynamic_meta_object_data();
                let meta_obj = unsafe {
                    meta_obj_data.get_dynamic_qmetaobject()
                        .as_ref()
                        .expect("Failed to get QMetaObject")
                };

                // TODO: find a better way to specify URI. Possible options visible so far:
                // * Set it in build.rs. That will force user to write build.rs. Previously build script wasn't mandatory.
                // * Custom attributes in Cargo.toml. Probably requires us to parse Cargo.toml manually at build/proc macro time.
                // * Add another custom file containing settings for Qt/QML at package root
                let uri = env!("CARGO_PKG_NAME")
                    .trim_start_matches(char::is_numeric)
                    .chars()
                    .map(|ch| if ch.is_alphanumeric() { ch } else { '_' })
                    .collect::<String>();

                let version_major = env!("CARGO_PKG_VERSION_MAJOR").parse()
                    .expect("Failed to parse package major version");
                let version_minor = env!("CARGO_PKG_VERSION_MINOR").parse()
                    .expect("Failed to parse package minor version");

                #qml_register_call
            }
        }
    };
    let output = quote! {
        #item_ts
        #qmlregister_code
    };
    Ok(output)
}

fn build_constructor_body(struct_ident: &syn::Ident, is_singleton: bool) -> TokenStream {
    let impl_details_mod = naming::rust::module::impl_details(&struct_ident.to_string());

    if is_singleton {
        quote! {
            pub extern "C"
            fn default_ctor() -> *mut qtbridge::QObject {
                let instance = std::rc::Rc::new(std::cell::RefCell::new(<#struct_ident as Default>::default()));
                #impl_details_mod::register_instance_in_map(instance.clone(), true);
                #impl_details_mod::set_dynamic_meta(&instance);
                std::ptr::from_mut(#impl_details_mod::get_qobject(&instance.borrow()))
            };
        }
    } else {
        quote! {
            pub extern "C"
            fn default_ctor(addr: *mut u8, _userdata: *mut u8) {
                let instance = std::rc::Rc::new(std::cell::RefCell::new(<#struct_ident as Default>::default()));
                #impl_details_mod::register_instance_in_map_with_cpp_proxy_at(addr, instance.clone());
                #impl_details_mod::set_dynamic_meta(&instance);
            };
        }
    }
}

fn build_qml_register_call(struct_name: &str, is_singleton: bool) -> TokenStream {
    if is_singleton {
        quote! {
            qtbridge::qml_register_singleton(
                <Self as qtbridge::qt_type_lib::QMetaTypeGet>::get_qmetatype(),
                default_ctor as usize,
                uri.as_bytes(),
                version_major,
                version_minor,
                #struct_name.as_bytes(),
                meta_obj,
            )
        }
    } else {
        let impl_details_mod = naming::rust::module::impl_details(struct_name);
        quote! {
            qtbridge::qt_type_lib::qml_register_element(
                <Self as qtbridge::qt_type_lib::QMetaTypeGet>::get_qmetatype(),
                #impl_details_mod::ProxyRust::get_qmetatype_list_of_cpp_proxy(),
                #impl_details_mod::ProxyRust::get_size_of_cpp_proxy() as u32,
                default_ctor as usize,
                uri.as_bytes(),
                version_major,
                version_minor,
                #struct_name.as_bytes(),
                meta_obj,
            )
        }
    }
}

// TODO: support potential generics here?
// TODO: check syntax to make sure it is not a trait impl?
fn get_struct_or_impl_info(input: TokenStream) -> syn::Result<(TokenStream, syn::Ident)> {
    if let Ok(item_struct) = syn::parse2::<syn::ItemStruct>(input.clone()) {
        let ident = item_struct.ident.clone();
        let ts = item_struct.to_token_stream();
        return Ok((ts, ident));
    }
    if let Ok(item_impl) = syn::parse2::<syn::ItemImpl>(input.clone()) {
        let ident = match &*item_impl.self_ty {
            syn::Type::Path(tp) => tp.path.segments.last().unwrap().ident.clone(),
            _ => {
                return Err(syn::Error::new_spanned(
                    &item_impl.self_ty,
                    "Unexpected type in impl",
                ));
            }
        };
        let ts = item_impl.to_token_stream();
        return Ok((ts, ident));
    }
    Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "Expected struct or impl",
    ))
}
