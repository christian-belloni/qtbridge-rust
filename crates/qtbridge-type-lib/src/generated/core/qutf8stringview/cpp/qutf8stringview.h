// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QUTF8STRINGVIEW_RUST_BRIDGE_H_
#define _QUTF8STRINGVIEW_RUST_BRIDGE_H_

#include <QUtf8StringView>
#include <cstdint>
#include "rust/cxx.h"

namespace rust::bridge::qutf8stringview {

QUtf8StringView QUtf8StringView_Default();

void inlineCppFn_as_bytes(QUtf8StringView const &self, uint8_t const *&ptr, ptrdiff_t &size);

} // namespace rust::bridge::qutf8stringview

namespace rust {

template <>
struct IsRelocatable<::QUtf8StringView> : ::std::true_type
{
};

} // namespace rust

#endif // _QUTF8STRINGVIEW_RUST_BRIDGE_H_
