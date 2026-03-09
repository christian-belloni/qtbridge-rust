use qt_gen_common::parse_utils::parse_name_value;

pub struct QObjectModuleParams {
    pub base: Option<syn::Ident>,
    pub no_drop: bool,
}

mod keywords {
    syn::custom_keyword!(Base);
    syn::custom_keyword!(NoDrop);
}

impl Default for QObjectModuleParams {
    fn default() -> Self {
        Self {
            base: None,
            no_drop: false,
        }
    }
}

impl syn::parse::Parse for QObjectModuleParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {

        let mut result = QObjectModuleParams::default();

        while !input.is_empty() {
            if input.peek(keywords::Base) {
                result.base = Some(parse_name_value::<syn::Ident, syn::Ident>(input)?.1);
            } else if input.peek(keywords::NoDrop) {
                input.parse::<keywords::NoDrop>()?;
                result.no_drop = true;
            } else {
                return Err(input.error("Unsupported attribute of qobject_impl macro"));
            }
            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(result)
    }
}
