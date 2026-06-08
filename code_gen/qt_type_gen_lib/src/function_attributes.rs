use qtbridge_gen_common::type_to_string::path_to_string_fallback;
use syn::spanned::Spanned;

use crate::generic_instantiation_decl::GenericInstantiationTypesList;

#[derive(Clone)]
pub struct FunctionAttributes {
    /// List of struct instantiation types for which this `Function` must be included in the generated code.
    instantiation_inclusions: Option<GenericInstantiationTypesList>,

    /// List of struct instantiation types for which this `Function` must not be included in the generated code.
    instantiation_exclusions: Option<GenericInstantiationTypesList>,

    /// List of doc attributes
    docs: Vec<syn::Attribute>,
}

impl FunctionAttributes {
    pub fn new(attrs: &[syn::Attribute]) -> syn::Result<Self> {
        let mut instantiation_inclusions = None;
        let mut instantiation_exclusions = None;
        let mut docs = Vec::new();

        for attr in attrs {
            if let syn::AttrStyle::Inner(tok) = attr.style {
               return Err(syn::Error::new(tok.span(), "Inner attributes are not supported"));
            }

            let path = attr.path();
            let Some(ident) = path.get_ident() else {
                return Err(syn::Error::new(attr.span(), format!("Unsupported path in function attribute: '{}'", path_to_string_fallback(path))))
            };

            match ident.to_string().as_str() {
                "include_if_struct_instantiation" => {
                    instantiation_inclusions = Some(attr.parse_args()?);
                },
                "exclude_if_struct_instantiation" => {
                    instantiation_exclusions = Some(attr.parse_args()?);
                },
                "doc" => docs.push(attr.clone()), // A doc attribute
                _ => return Err(syn::Error::new(ident.span(), "Unsupported attribute")),
            }
            if instantiation_inclusions.is_some() && instantiation_exclusions.is_some() {
                return Err(syn::Error::new(attr.meta.span(), "Forbidden to have #[include_if_struct_instantiation] and #[exclude_if_struct_instantiation] attributes for the same function"))
            }
        }

        Ok(Self {
            instantiation_inclusions,
            instantiation_exclusions,
            docs,
        })
    }

    pub fn instantiation_inclusions(&self) -> Option<&GenericInstantiationTypesList> {
        self.instantiation_inclusions.as_ref()
    }

    pub fn instantiation_exclusions(&self) -> Option<&GenericInstantiationTypesList> {
        self.instantiation_exclusions.as_ref()
    }

    pub fn docs(&self) -> &[syn::Attribute] {
        &self.docs
    }
}
