// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;
use qt_type_lib::QVariant;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("qt_type_lib/src/generated/core/qvariant/cpp/qvariant.h");
        type QVariant = super::QVariant;
    }

    unsafe extern "C++" {
        include!("cpp/metamethodparams.h");
        type MetaMethodOutgoingParams = super::MetaMethodOutgoingParams;

        #[rust_name = "push"]
        fn push(&mut self, value: QVariant);
    }

    #[namespace = "rust::bridge"]
    unsafe extern "C++" {
        #[rust_name = "meta_method_outgoing_params_new"]
        fn MetaMethodOutgoingParams_New() -> MetaMethodOutgoingParams;
    }
}

// Allow instantiation of the class on the stack
// to avoid too wordy syntax using std::pin<> in the client code
#[repr(C)]
pub struct MetaMethodOutgoingParams {
    // class contains only std::vector<QObject>
    _content: MaybeUninit<[usize; 4]>,
}

unsafe impl ExternType for MetaMethodOutgoingParams {
    type Id = type_id!("MetaMethodOutgoingParams");
    type Kind = cxx::kind::Trivial;
}

impl MetaMethodOutgoingParams {
    pub fn new() -> Self {
        ffi::meta_method_outgoing_params_new()
    }
}
