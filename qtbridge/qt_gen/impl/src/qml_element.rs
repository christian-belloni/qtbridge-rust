use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, format_ident, quote};

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

    let uri = std::env::var("CARGO_PKG_NAME")
        .map_err(|err| syn::Error::new(Span::call_site(), format!("Failed to get CARGO_PKG_NAME: {err}")))?
        .trim_start_matches(char::is_numeric)
        .chars()
        .map(|ch| if ch.is_alphanumeric() { ch } else { '_' })
        .collect::<String>();
    let minor_version: u8 = std::env::var("CARGO_PKG_VERSION_MINOR")
        .map_err(|err| syn::Error::new(Span::call_site(), format!("Failed to get CARGO_PKG_VERSION_MINOR: {err}")))?
        .parse()
        .expect("Failed to parse CARGO_PKG_VERSION_MINOR");
    let major_version: u8 = std::env::var("CARGO_PKG_VERSION_MAJOR")
        .map_err(|err| syn::Error::new(Span::call_site(), format!("Failed to get CARGO_PKG_VERSION_MAJOR: {err}")))?
        .parse()
        .expect("Failed to parse CARGO_PKG_VERSION_MAJOR");

    let qmlregister_code = quote! {
        // TODO: make auto registration via 'linkme' dependency an optional cargo feature?
        #[linkme::distributed_slice(qtbridge::qt_type_lib::QML_REGISTER_CALLBACKS)]
        #[allow(non_camel_case_types)]
        fn #qml_register_fn_indent() {
            <#struct_ident as qtbridge::bridge::QmlRegister>::qml_register();
        }

        impl qtbridge::bridge::QmlRegister for #struct_ident {
            const URI: &str = #uri;
            const ELEMENT_NAME: &str = #struct_name;
            const MINOR_VERSION: u8 = #minor_version;
            const MAJOR_VERSION: u8 = #major_version;
            const IS_SINGLETON: bool = #is_singleton;
        }
    };
    let output = quote! {
        #item_ts
        #qmlregister_code
    };
    Ok(output)
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
