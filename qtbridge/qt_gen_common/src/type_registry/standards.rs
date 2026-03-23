// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::sync::LazyLock;

use super::containers::StandardContainer;
use super::holders::ValueHolder;
use super::pointers::PointerType;
use super::primitives::PrimitiveType;
use super::strings::StringType;
use super::type_traits::{StaticTypeGroup, TypesEnum, TypeInfo, TypeName};

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum StandardType {
    Primitive(&'static PrimitiveType),
    String(&'static StringType),
    Container(&'static StandardContainer),
    Holder(&'static ValueHolder),
    Pointer(&'static PointerType),
}


impl TypesEnum for StandardType {
    fn dyn_type_info(&self) -> &dyn TypeInfo {
        match self {
            Self::Primitive(primitive) => primitive.dyn_type_info(),
            Self::String(string) => string.dyn_type_info(),
            Self::Container(container) => container.dyn_type_info(),
            Self::Holder(holder) => holder.dyn_type_info(),
            Self::Pointer(pointer) => pointer.dyn_type_info(),
        }
    }
}

impl StaticTypeGroup for StandardType {
    fn get_static_sorted_list() -> &'static [Self] {
        static LIST: LazyLock<Vec<StandardType>> = LazyLock::new(|| {
            let mut result: Vec<_> = PrimitiveType::get_static_sorted_list().iter()
                    .map(StandardType::from)
                .chain(StringType::get_static_sorted_list().iter()
                    .map(StandardType::from))
                .chain(StandardContainer::get_static_sorted_list().iter()
                    .map(StandardType::from))
                .chain(ValueHolder::get_static_sorted_list().iter()
                    .map(StandardType::from))
                .chain(PointerType::get_static_sorted_list().iter()
                    .map(StandardType::from))
                .collect();
            result.sort_unstable_by(|l, r| l.name().cmp(r.name()));
            result
        });

        LIST.as_slice()
    }
}

impl From<&'static PrimitiveType> for StandardType {
    fn from(value: &'static PrimitiveType) -> Self {
        Self::Primitive(value)
    }
}

impl From<&'static StringType> for StandardType {
    fn from(value: &'static StringType) -> Self {
        Self::String(value)
    }
}

impl From<&'static StandardContainer> for StandardType {
    fn from(value: &'static StandardContainer) -> Self {
        Self::Container(value)
    }
}

impl From<&'static ValueHolder> for StandardType {
    fn from(value: &'static ValueHolder) -> Self {
        Self::Holder(value)
    }
}

impl From<&'static PointerType> for StandardType {
    fn from(value: &'static PointerType) -> Self {
        Self::Pointer(value)
    }
}
