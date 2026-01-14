// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::hash::Hash;

use proc_macro2::Span;

use crate::type_registry::type_traits::{FindType, MetaTypeId, TypeInfo, TypeName, TypesEnum, find_type_by_partial_path};
use crate::type_utils::get_angle_bracketed_generic_arguments_of_last_path_segment;
use crate::type_registry::qt;
use qt::non_generic::QtNonGenericType;
use qt::generic::{QtGenericTypeWithoutArgs, QtGenericTypeWithArgs};
use qt::monomorphed::QtMonomorphedType;
use qt::monomorphed_alias::QtAliasToMonomorphedType;

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum QtType {
    NonGeneric(QtNonGenericType),
    GenericWithoutArgs(QtGenericTypeWithoutArgs),
    GenericWithArgs(QtGenericTypeWithArgs),
    GenericMonomorphed(QtMonomorphedType),
    AliasToMonomorphed(QtAliasToMonomorphedType),
}


type QtTypeMap = BTreeMap<String, QtType>;
thread_local!(static QT_TYPE_MAP: RefCell<QtTypeMap> = RefCell::new(QtType::get_all()));
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/qt_types.rs"));

impl QtType {
    /// Iterate over all the values in the map with Qt types
    /// and invoke the provided functor for each value.
    pub fn visit_all<F>(mut f: F) -> Result<(), String>
    where F: FnMut(&QtType) -> Result<(), String>
    {
        QT_TYPE_MAP.with_borrow(|map| -> Result<(), String> {
            for ty in map.values() {
                f(ty)?;
            }
            Ok(())
        })
    }

    pub fn path_in_gen(&self) -> &str {
        match self {
            QtType::NonGeneric(concrete) => concrete.path_in_gen(),
            QtType::GenericWithoutArgs(generic_wo_args) => generic_wo_args.path_in_gen(),
            QtType::GenericWithArgs(generic_w_args) => generic_w_args.path_in_gen(),
            QtType::GenericMonomorphed(monomorphed) => monomorphed.path_in_gen(),
            QtType::AliasToMonomorphed(alias_to_monomorphed) => alias_to_monomorphed.path_in_gen(),
        }
    }

    /// Returns all supported Qt types as a map,
    /// where the key is the type name and the value is the corresponding `QtType`.
    fn get_all() -> BTreeMap<String, QtType> {
        let non_generics = get_non_generic_types();
        let generics = get_generic_types();
        let monomorphed = get_monomorphed_types(&generics, &non_generics);
        let aliases = get_alias_to_monomorphed_types();

        non_generics.into_iter()
                .map(QtType::from)
            .chain(generics.into_iter()
               .map(QtType::from))
            .chain(monomorphed.into_iter()
                .map(QtType::from))
            .chain(aliases.into_iter()
                .map(QtType::from))
            .map(|qt_type| (qt_type.name().to_owned(), qt_type))
            .collect()
    }
}

impl FindType for QtType {
    fn find_by_name(name: &str) -> Option<Self> {
        QT_TYPE_MAP.with_borrow(|map| map.get(name).cloned())
    }

    fn find_by_partial_path(path: &syn::Path) -> Option<Self> {
        let qt_type = find_type_by_partial_path::<Self>(path)?;

        // If type is generic with args specified - try to find monomorphed form
        if let Self::GenericWithoutArgs(qt_generic) = &qt_type {
            if let Some(ab) = get_angle_bracketed_generic_arguments_of_last_path_segment(path) {
                let qt_generic_w_types = qt_generic.set_args_from_syn_generic_args(ab)
                    .ok()?;
                return Some(qt_generic_w_types.into())
            }
        };

        Some(qt_type)
    }
}

impl TypesEnum for QtType {
    fn dyn_type_info(&self) -> &dyn TypeInfo {
        match self {
            QtType::NonGeneric(concrete) => concrete.dyn_type_info(),
            QtType::GenericWithoutArgs(generic_wo_args) => generic_wo_args.dyn_type_info(),
            QtType::GenericWithArgs(generic_w_args) => generic_w_args.dyn_type_info(),
            QtType::GenericMonomorphed(monomorphed) => monomorphed.dyn_type_info(),
            QtType::AliasToMonomorphed(alias_to_monomorphed) => alias_to_monomorphed.dyn_type_info(),
        }
    }
}

impl From<QtNonGenericType> for QtType {
    fn from(value: QtNonGenericType) -> Self {
        Self::NonGeneric(value)
    }
}

impl From<QtGenericTypeWithoutArgs> for QtType {
    fn from(value: QtGenericTypeWithoutArgs) -> Self {
        Self::GenericWithoutArgs(value)
    }
}

impl From<QtGenericTypeWithArgs> for QtType {
    fn from(value: QtGenericTypeWithArgs) -> Self {
        Self::GenericWithArgs(value)
    }
}

impl From<QtMonomorphedType> for QtType {
    fn from(value: QtMonomorphedType) -> Self {
        Self::GenericMonomorphed(value)
    }
}

impl From<QtAliasToMonomorphedType> for QtType {
    fn from(value: QtAliasToMonomorphedType) -> Self {
        Self::AliasToMonomorphed(value)
    }
}


#[derive(Clone)]
pub struct QtTypeSpanned {
    ty: QtType,
    span: Span,
}

impl QtTypeSpanned {
    pub fn new(ty: QtType, span: Span) -> Self {
        Self { ty, span }
    }

    pub fn get_type(&self) -> &QtType {
        &self.ty
    }

    pub fn set_type(&mut self, ty: QtType) {
        self.ty = ty;
    }

    pub fn dyn_type_info(&self) -> &dyn TypeInfo {
        self.ty.dyn_type_info()
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

impl PartialEq for QtTypeSpanned {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty
    }
}

impl Eq for QtTypeSpanned {}

impl Hash for QtTypeSpanned {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ty.hash(state);
    }
}
