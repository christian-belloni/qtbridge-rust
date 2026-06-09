use proc_macro2::TokenStream;
use quote::quote;

use qtbridge_gen_common::type_registry;
use type_registry::QtType;
use type_registry::type_traits::{TypeInfo, TypeName};

/// Generate code of functions returning collections of Qt types (per type category)
pub fn generate_qt_types_getters_code() -> Result<TokenStream, String> {
    let mut non_generic_vec = Vec::new();
    let mut generic_vec = Vec::new();
    let mut monomorphed_vec = Vec::new();
    let mut alias_vec = Vec::new();

    QtType::visit_all(|ty| {
        let name = ty.name();
        let path_in_gen = ty.path_in_gen();
        let metatype = ty.metatype_id();
        let namespace = ty.cpp_namespace()
            .unwrap_or_default();
        match ty {
            QtType::NonGeneric(_) => non_generic_vec.push(quote! {
                QtNonGenericType::new_str(#name, #path_in_gen, #metatype, #namespace)
            }),
            QtType::GenericWithoutArgs(gen_wo_args) => {
                let args = gen_wo_args.args();
                generic_vec.push(quote!{
                    QtGenericTypeWithoutArgs::new_str(#name, #path_in_gen, &[#(#args),*])
                })
            }
            QtType::GenericMonomorphed(mono) => {
                let gen_name = mono.source().gen_name().to_owned();
                let args = mono.source().args().iter()
                    .map(|arg| {
                        let ty_str = arg.to_string();
                        quote! {
                            #ty_str.try_into().unwrap()
                        }
                    });

                monomorphed_vec.push(quote! {
                    QtMonomorphedType::new_str(
                        #name,
                        #path_in_gen,
                        QtGenericTypeWithoutArgs::find_by_name(#gen_name)
                            .unwrap()
                            .set_args(vec![#(#args),*])
                            .unwrap(),
                        #metatype)
                });
            },
            QtType::AliasToMonomorphed(alias) => {
                let alias_to = alias.to();
                alias_vec.push(quote! {
                    QtAliasToMonomorphedType::new_str(#name, #alias_to, #path_in_gen, #metatype)
                });
            },
            _ => {},
        }
        Ok(())
    })?;

    let non_generic_count = non_generic_vec.len();
    let generic_count = generic_vec.len();
    let monomorphed_count = monomorphed_vec.len();
    let alias_count = alias_vec.len();

    let code = quote! {
        fn get_non_generic_types() -> [QtNonGenericType; #non_generic_count] {
            [
                #(#non_generic_vec),*
            ]
        }
        fn get_generic_types() -> [QtGenericTypeWithoutArgs; #generic_count] {
            [
                #(#generic_vec),*
            ]
        }
        fn get_monomorphed_types() -> [QtMonomorphedType; #monomorphed_count] {
            [
                #(#monomorphed_vec),*
            ]
        }
        fn get_alias_to_monomorphed_types() -> [QtAliasToMonomorphedType; #alias_count] {
            [
                #(#alias_vec),*
            ]
        }
    };
    Ok(code)
}
