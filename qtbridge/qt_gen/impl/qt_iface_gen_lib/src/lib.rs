// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

pub mod method_override;
pub mod iface_desc;
pub mod iface_desc_method;
pub mod proxy_gen;

pub use iface_desc::InterfaceDesc;
pub use iface_desc_method::IfaceMethodDesc;
pub use method_override::MethodOverride;
