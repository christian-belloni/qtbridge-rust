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
        type MetaMethodIncomingParams;

        fn get_bool(&self, num: usize) -> bool;

        #[rust_name = "get_i64"]
        fn get_int64_t(&self, num: usize) -> i64;

        #[rust_name = "get_u64"]
        fn get_uint64_t(&self, num: usize) -> u64;

        #[rust_name = "get_i32"]
        fn get_int32_t(&self, num: usize) -> i32;

        #[rust_name = "get_u32"]
        fn get_uint32_t(&self, num: usize) -> u32;

        #[rust_name = "get_i16"]
        fn get_int16_t(&self, num: usize) -> i16;

        #[rust_name = "get_u16"]
        fn get_uint16_t(&self, num: usize) -> u16;

        #[rust_name = "get_i8"]
        fn get_int8_t(&self, num: usize) -> i8;

        #[rust_name = "get_u8"]
        fn get_uint8_t(&self, num: usize) -> u8;

        #[rust_name = "get_f32"]
        fn get_float(&self, num: usize) -> f32;

        #[rust_name = "get_f64"]
        fn get_double(&self, num: usize) -> f64;

        #[rust_name = "get_string"]
        fn getString(&self, num: usize) -> String;

        #[rust_name = "get_string_list"]
        fn getStringList(&self, num: usize) -> Vec<String>;
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

pub use ffi::MetaMethodIncomingParams;

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
