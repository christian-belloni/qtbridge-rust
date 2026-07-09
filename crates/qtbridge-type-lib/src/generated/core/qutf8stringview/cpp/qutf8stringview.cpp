// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "qutf8stringview.h"

namespace rust::bridge::qutf8stringview {

QUtf8StringView QUtf8StringView_Default()
{
    return QUtf8StringView();
}

void inlineCppFn_as_bytes(QUtf8StringView const &self, uint8_t const *&ptr, ptrdiff_t &size)
{
    ptr = reinterpret_cast<const uint8_t *>(self.data());
    size = self.size();
}

} // namespace rust::bridge::qutf8stringview
