use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::DeriveInput;
use std::collections::HashMap;

/// Derive macro that generates a `QModelItem` implementation.
///
/// Applying this macro to a struct allows it to be used inside `QVec<T>`
/// and exposed to QML, where it can be visualized with various views. The
/// delegates within the view are able to read and write to the struct
/// through the roles.
///
/// ## Roles
/// - **Named Field Struct:** The generated roles will match the names of the
///   fileds (e.g., `name`, `age`, …). Fields named `display`, `decoration`,
///   `edit`, `toolTip`, `statusTip`, or `whatsThis` are recognized as default
///   roles as used in Qt's default delegates.
/// - **Tuple Structs:** The generated roles are `"_0"`, `"_1"`, `"_2"`, ...
///
/// ## Type requirements
/// All fields must be convertible to and from `QVariant`.
///
/// ## Example
/// ```rust,ignore
/// #[derive(QModelItem)]
/// struct Person {
///     name: String,   // role "name"
///     age: u32,       // role "age"
/// }
///
/// #[derive(QModelItem)]
/// struct Pair(i32, String); // roles "_0", "_1"
/// ```
#[proc_macro_derive(QModelItem)]
pub fn derive_qmodelitem(input: TokenStream) -> TokenStream {
        match try_derive_qmodelitem(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error().into(),
    }
}

fn try_derive_qmodelitem(input: TokenStream) -> syn::Result<TokenStream> {
    let input: DeriveInput = syn::parse(input)?;
    let name = &input.ident;

    let s = match &input.data {
        syn::Data::Struct(st) => st,
        _ => return Err(syn::Error::new_spanned(&input, "QModelItem only supports structs")),
    };

    let fields = s.fields.iter().collect::<Vec<_>>();

    let (members, field_names): (Vec<_>, Vec<_>) = fields.iter().enumerate()
    .map(|(i, f)|
        match f.ident.as_ref() {
            Some(ident) => (quote! { self.#ident }, ident.to_string()),
            None => {
                let index = syn::Index::from(i);
                (quote! { self.#index }, format!("_{}", i))
            },
        })
    .unzip();

    let len = fields.len();
    let max_elements = 15;

    if len > max_elements {
        return Err(syn::Error::new_spanned(&s.fields, "QModelItem supports only up to 15 fields"));
    }

    let field_types = fields.iter().map(|f| &f.ty).collect::<Vec<_>>();

    // see  Qt::ItemDataRole
    let default_roles: HashMap<&str, i32> = HashMap::from([
        ("display", 0x0),
        ("decoration", 0x1),
        ("edit", 0x2),
        ("toolTip", 0x3),
        ("statusTip", 0x4),
        ("whatsThis", 0x5),
    ]);

    let mut prop_mapping = Vec::new();

    // Do some mapping so the default types go to the respective index
    // if they are present. This is important due to the current implementation
    // of some default delegates.
    prop_mapping.resize(field_names.len(), max_elements);
    for (i, f) in field_names.iter().enumerate() {
        if let Some(&predef) = default_roles.get(f.as_str()) {
            prop_mapping[predef as usize] = i;
        }
    }
    let mut j = 0;
    for (i, f) in field_names.iter().enumerate() {
        if !default_roles.contains_key(f.as_str()){
            while prop_mapping[j] < max_elements {
                j = j + 1;
            }
            prop_mapping[j] = i;
            j = j + 1;
        }
    }

    let (getters, setters): (Vec<_>, Vec<_>) = members.iter()
        .enumerate()
        .map(|(i, member)| {
            let getter_fn_ident = format_ident!("elem{}", prop_mapping[i]);
            let setter_fn_ident = format_ident!("set_elem{}", prop_mapping[i]);
            let ty = field_types.get(i).unwrap();
            (
                quote! {
                    fn #getter_fn_ident(&self) -> QVariant {  QVariant::from(&#member) }
                },
                quote! {
                    fn #setter_fn_ident(&mut self, value: &QVariant) -> bool {
                        <#ty>::try_from(value).map(|val| #member = val).is_ok()
                    }
                }
            )
        })
        .unzip();

    let mut role_tokens = Vec::new();

    for (i,f) in field_names.iter().enumerate() {
        let idx = prop_mapping[i] as i32;

        role_tokens.push(quote! {
            (#idx, #f.into()),
        });
    }

    Ok(quote! {
        impl QModelItem for #name {

            const LEN: usize = #len;

            #(#getters)*

            #(#setters)*

            fn role_names() -> HashMap<i32, String> {
                let roles = [
                #(#role_tokens)*
                ];
                roles.iter().cloned().collect()
            }
        }
    }.into())
}
