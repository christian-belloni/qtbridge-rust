// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::{QMetaTypeIdConst, QMetaTypeId};

#[doc(hidden)]
pub fn get_meta_type_id_of_val<T: QMetaTypeIdConst>(_t: &T) -> QMetaTypeId {
    T::ID
}

#[doc(hidden)]
pub fn get_meta_type_id_of_fn_return_value<F, This, R>(_f: F) -> QMetaTypeId
where F: FnOnce(&This) -> &R,
      R: QMetaTypeIdConst
{
    R::ID
}
