// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qt::generic::QtGenericArg;

// Functionality of QtType that is used for 'no_types' case only (type generation).
impl QtType {

    pub fn add_concrete(name: String, path_in_gen: String, metatypeid: MetaTypeId, namespace: String) {
        QT_TYPE_MAP.with_borrow_mut(|map| {
            map.insert(name.clone(),
                QtNonGenericType::new(name, path_in_gen, metatypeid, namespace)
                    .into());
        })
    }

    pub fn add_generic(name: String, path_in_gen: String, args: Vec<syn::Ident>) {
        QT_TYPE_MAP.with_borrow_mut(|map| {
            map.insert(name.clone(),
                QtGenericTypeWithoutArgs::new(name, path_in_gen, args)
                    .into());
        })
    }

    pub fn add_monomorphed(monomorphed_name: String, generic_ident: &syn::Ident, generic_args: Vec<QtGenericArg>, path_in_gen: String, metatypeid: MetaTypeId) -> syn::Result<()> {
        let generic_wo_args = QtGenericTypeWithoutArgs::find_by_name(&generic_ident.to_string())
            .ok_or_else(|| syn::Error::new(generic_ident.span(), format!("Failed to find generic type '{generic_ident}' for monomorphed '{monomorphed_name}'")))?;
        let generic_w_args = generic_wo_args.set_args(generic_args)
            .map_err(|err| syn::Error::new(generic_ident.span(), format!("Failed to set arguments to generic struct '{generic_ident}': {err}")))?;

        QT_TYPE_MAP.with_borrow_mut(|map| {
            map.insert(monomorphed_name.clone(),
                QtMonomorphedType::new(monomorphed_name, path_in_gen, generic_w_args, metatypeid)
                    .into());
        });

        Ok(())
    }

    pub fn add_alias_to_monomoprhed(name: String, monomorped_name: String, path_in_gen: String, metatypeid: MetaTypeId) {
        QT_TYPE_MAP.with_borrow_mut(|map| {
            map.insert(name.clone(),
                QtAliasToMonomorphedType::new(name, monomorped_name, path_in_gen, metatypeid)
                    .into());
        });
    }
}

// Stubs that just return empty arrays.
pub fn get_non_generic_types() -> [QtNonGenericType; 0] {
    []
}

pub fn get_generic_types() -> [QtGenericTypeWithoutArgs; 0] {
    []
}

pub fn get_monomorphed_types(_generics: &[QtGenericTypeWithoutArgs], _non_generics: &[QtNonGenericType]) -> [QtMonomorphedType; 0] {
    []
}

pub fn get_alias_to_monomorphed_types() -> [QtAliasToMonomorphedType; 0] {
    []
}
