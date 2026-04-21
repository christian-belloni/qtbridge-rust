// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge_type_lib::QVariant;

pub trait DispatchMetaCall {
    fn invoke_slot(&self, slot_id: u32, inputs: &[*const u8], outputs: &[*mut u8]);
    fn invoke_slot_mut(&mut self, slot_id: u32, inputs: &[*const u8], outputs: &[*mut u8]);
    fn read_property(&self, prop_id: u32) -> QVariant;
    fn write_property(&mut self, prop_id: u32, value: &QVariant);
}
