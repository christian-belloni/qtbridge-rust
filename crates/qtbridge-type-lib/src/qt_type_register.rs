// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge_gen_common::type_registry::type_traits::{FindType, MetaTypeId};
use qtbridge_gen_common::type_registry::qt;
use qt::non_generic::QtNonGenericType;
use qt::generic::{QtGenericTypeWithoutArgs};
use qt::monomorphed::QtMonomorphedType;
use qt::monomorphed_alias::QtAliasToMonomorphedType;
use qtbridge_gen_common::type_registry::qt::QtType;
use std::sync::Once;

include!("qt_types.rs");

thread_local! {
    static INIT: Once = Once::new();
}

pub fn init() {
    INIT.with(|init| {
        init.call_once(|| {
            for ty in get_non_generic_types() {
                QtType::add_concrete(ty);
            }

            for ty in get_generic_types() {
                QtType::add_generic(ty);
            }

            for ty in get_monomorphed_types() {
                QtType::add_monomorphed(ty);
            }

            for ty in get_alias_to_monomorphed_types() {
                QtType::add_alias_to_monomoprhed(ty);
            }
        });
    });
}
