// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::{QMetaType, QMetaTypeGet};

#[doc(hidden)]
pub fn get_meta_type_of_fn_return_value<F, This, R>(_f: F) -> QMetaType
where F: FnOnce(&This) -> &R,
      R: QMetaTypeGet
{
    R::get_qmetatype()
}
