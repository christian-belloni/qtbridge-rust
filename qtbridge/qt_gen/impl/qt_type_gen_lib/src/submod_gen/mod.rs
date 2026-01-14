// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

pub mod common;
mod generic;
mod monomorphed;
mod non_generic_base;
mod non_generic;
mod submodule_generator;

pub use generic::GenericSubmoduleGenerator;
pub use monomorphed::MonomorphedSubmoduleGenerator;
pub use non_generic::NonGenericSubmoduleGenerator;
pub use submodule_generator::SubmoduleGenerator;

