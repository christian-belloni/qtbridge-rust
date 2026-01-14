// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

mod containers;
pub mod cxx_types;
mod holders;
mod pointers;
mod primitives;
pub mod qt;
pub mod standards;
mod strings;
pub mod type_traits;
pub mod types;

pub use cxx_types::CxxType;
pub use type_traits::{TypeCategory, TypesEnum};
pub use primitives::PrimitiveType;
pub use standards::StandardType;
pub use qt::{QtType, QtTypeSpanned};
pub use types::Type;
