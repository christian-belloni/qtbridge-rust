use qt_gen_common::parse_utils::is_doc_attribute;
use syn::spanned::Spanned;

#[derive(Clone)]
pub struct FunctionAttributes {
    docs: Vec<syn::Attribute>,
}

impl FunctionAttributes {
    pub fn new(attrs: &[syn::Attribute]) -> syn::Result<Self> {
        let mut docs = Vec::new();

        for attr in attrs {
            if is_doc_attribute(attr) {
                docs.push(attr.clone());
                continue
            }

            return Err(syn::Error::new(attr.span(), "Unsupported type of function attribute"))
        }

        Ok(Self {
            docs,
        })
    }

    pub fn docs(&self) -> &[syn::Attribute] {
        &self.docs
    }
}
