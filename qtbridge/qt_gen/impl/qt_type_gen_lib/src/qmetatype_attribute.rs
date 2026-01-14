use proc_macro2::Span;
use qt_gen_common_no_types::parse_utils::parse_name_value;

#[derive(Clone)]
pub struct QMetaTypeAttribute {
    /// When 'id' is Some it holds constant built-in QMetaType Id for given type
    /// None - type can be treated with QMetaType but has no constant Id. Id is assigned to that type at runtime
    id: Option<syn::LitInt>,
}

impl QMetaTypeAttribute {
    pub fn id(&self) -> Option<i32> {
        self.id.as_ref()
            .map(|lit| lit.base10_parse().ok())
            .flatten()
    }

    pub fn id_span(&self) -> Option<Span> {
        self.id.as_ref()
            .map(|lit| lit.span())
    }

    pub fn new() -> Self {
        Self {
            id: None,
        }
    }

    pub fn parse_from_meta_list_args(input: &syn::parse::ParseBuffer) -> syn::Result<Self> {
        let (name, value) = parse_name_value::<syn::Ident, syn::LitInt>(input)?;
        if name != "id" {
            return Err(syn::Error::new(name.span(), "'id' expected here"))
        }

        let id: u16 = value.base10_parse()?;
        if id == 0 {
            return Err(syn::Error::new(value.span(), "id must be > 0"))
        }

        Ok(QMetaTypeAttribute {
            id: Some(value),
        })
    }
}

