use qtbridge_gen_common::type_tokens::TypeTokens;
use qtbridge_gen_common::type_utils::ident_str_to_path;

use crate::function::Function;
use crate::generic_types_instantiations::GenericTypesInstantiations;
use crate::trait_impl::TraitImpl;

/// Types mentioned in Rust code of submodule
#[derive(Default)]
pub struct SubmoduleTypeTokens {
    all: TypeTokens,
}

impl SubmoduleTypeTokens {
    pub fn new_for_generic(generic_idents: &[syn::Ident]) -> Self {
        Self {
            all: TypeTokens::new_for_generic(generic_idents),
        }
    }

    pub fn all(&self) -> &TypeTokens {
        &self.all
    }

    pub fn all_mut(&mut self) -> &mut TypeTokens {
        &mut self.all
    }

    pub fn check_unclassified(&mut self) -> syn::Result<()> {
        self.all.check_unclassified()
    }

    pub fn remove_self(&mut self) {
        let self_path = ident_str_to_path("Self");

        self.remove_unclassified(&self_path);
    }

    pub fn remove_unclassified(&mut self, value: &syn::Path) {
        self.all.remove_unclassified(value);
    }

    fn remove_qt(&mut self, value: &syn::Path) {
        self.all.remove_qt(value);
    }

    pub fn remove_qt_and_unclassified(&mut self, value: &syn::Path) {
        self.remove_qt(value);
        self.remove_unclassified(value);
    }

    pub fn collect_from_functions(&mut self, src: &[Function]) -> syn::Result<()> {
        src.iter()
            .try_for_each(|func| self.collect_from_function(func))
    }

    pub fn collect_from_traits(&mut self, src: &[TraitImpl]) -> syn::Result<()> {
        src.iter()
            .try_for_each(|t| self.collect_from_trait(t))
    }

    fn collect_from_function(&mut self, src: &Function) -> syn::Result<()> {
        self.all.collect_from_signature(src.signature())?;

        src.cpp_functions().iter()
            .try_for_each(|cpp_func|
                self.all.collect_from_signature(cpp_func.signature())
            )
    }

    fn collect_from_trait(&mut self, src: &TraitImpl) -> syn::Result<()> {

        // Ignore generic const idents of the trait impl
        src.generics().consts()
            .try_for_each(|const_|
                self.all.add_generic_ident(&const_.ident)
            )?;


        self.all.collect_from_path(src.self_type())?;
        self.collect_from_functions(src.functions())?;

        // Substitute trait-level generics
        if let Some(inst_decl) = src.get_instantiations() {
            let all_insts = GenericTypesInstantiations::new(src.generics().idents(), inst_decl)?;
            all_insts.iter_type_insts()
                .try_for_each(|(ident, inst_types)|
                    self.all.substitute_generic_insts(ident, inst_types.iter())
                )?;
        }

        // Remove generic const idents added at the beginning of the function
        src.generics().consts()
            .try_for_each(|const_|
                self.all.remove_generic_ident(&const_.ident)
            )?;

        Ok(())
    }
}
