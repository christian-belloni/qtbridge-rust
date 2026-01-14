// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef QRESOURCE_RUST_H
#define QRESOURCE_RUST_H

#include "rustconv.h"

bool register_resource(rust::Slice<const uint8_t> data, rust::Str resourceRoot);

#endif //QRESOURCE_RUST_H
