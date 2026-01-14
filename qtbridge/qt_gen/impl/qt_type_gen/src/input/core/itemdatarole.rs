// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#[cxx::bridge(namespace = "Qt")]
pub mod ffi {
    #[repr(i32)]
    enum ItemDataRole {
        DisplayRole = 0,
        DecorationRole = 1,
        EditRole = 2,
        ToolTipRole = 3,
        StatusTipRole = 4,
        WhatsThisRole = 5,
        SizeHintRole = 13,
        UserRole = 0x0100,
    }

    unsafe extern "C++" {
        include!(<qnamespace.h>); // for ItemDataRole enum
        type ItemDataRole;
    }
}
