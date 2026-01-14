use syn::parse::Parse;

/// The type of a generic argument in a trait `impl`
/// specified in angle brackets after 'impl' keyword.
/// Supported options so far:
/// ident specifying a type (e.g. T, K, V, etc.)
/// const param (e.g., 'const N: usize')
#[derive(Clone)]
pub enum TraitImplGeneric {
    Ident(syn::Ident),
    Const(syn::ConstParam),
}

#[derive(Clone, Default)]
pub struct TraitImplGenericList {
    list: Vec<TraitImplGeneric>,
}

impl TraitImplGenericList {
    pub fn idents(&self) -> impl Iterator<Item = &syn::Ident> {
        self.list.iter()
            .filter_map(|item| match item {
                TraitImplGeneric::Ident(ident) => Some(ident),
                _ => None,
            })
    }

    pub fn consts(&self) -> impl Iterator<Item = &syn::ConstParam> {
        self.list.iter()
            .filter_map(|item| match item {
                TraitImplGeneric::Const(const_param) => Some(const_param),
                _ => None,
            })
    }

    pub fn clone_generics(&self) -> Self {
        let list = self.consts()
                .map(|const_param| TraitImplGeneric::Const(const_param.clone()))
                .collect();

        Self {
            list
        }
    }
}

impl Parse for TraitImplGenericList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut list = Vec::new();

        let _left_angle: syn::Token![<] = input.parse()?;
        {
            if input.peek(syn::Token![const]) {
                list.push(TraitImplGeneric::Const(input.parse()?));
            } else {
                list.push(TraitImplGeneric::Ident(input.parse()?));
            }
        }
        let _right_angle: syn::Token![>] = input.parse()?;

        Ok(Self {
            list
        })
    }
}
