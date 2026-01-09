use qt_gen_common::parse_utils::parse_name_value;

pub struct QObjectModuleParams {
    base: Option<syn::Ident>,
}

impl QObjectModuleParams {
    pub fn base(&self) -> Option<&syn::Ident> {
        self.base.as_ref()
    }
}

mod keywords {
    syn::custom_keyword!(Base);
}

impl syn::parse::Parse for QObjectModuleParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut base = None;

        while !input.is_empty() {
            if input.peek(keywords::Base) {
                base = Some(parse_name_value::<syn::Ident, syn::Ident>(input)?.1);
            } else {
                return Err(input.error("Unsupported attribute of qobject_impl macro"));
            }
        }
        Ok(Self {
            base,
        })
    }
}
