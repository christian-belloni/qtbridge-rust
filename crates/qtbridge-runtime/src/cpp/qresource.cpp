// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "qresource.h"

#include <QResource>

bool register_resource(rust::Slice<const uint8_t> data, rust::Str resourceRoot) {
    return QResource::registerResource(data.data(), RustStrToQString(resourceRoot));
}
