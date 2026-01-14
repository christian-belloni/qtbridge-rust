// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::mem::MaybeUninit;
use crate::QObject;

#[qt_gen::bridge]
mod qsignalblocker {
    include_in_cpp!(<QSignalBlocker>);

    #[doc(hidden)]
    #[derive_cpp(Drop)]
    struct QSignalBlocker {
        _m_o: MaybeUninit<usize>,
        _m_blocked: MaybeUninit<bool>,
        _m_inhibited: MaybeUninit<bool>,
    }

    pub fn new(qobj: &mut QObject) -> Self {
        let cpp = cpp_fn!(|qobj: &mut QObject| -> Self {
            return QSignalBlocker(qobj);
        });
        cpp(qobj)
    }
}
