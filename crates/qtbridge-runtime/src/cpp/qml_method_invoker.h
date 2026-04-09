// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef QML_METHOD_INVOKER_H
#define QML_METHOD_INVOKER_H

#include <QObject>
#include <cstddef>
#include <cstdint>

void on_qobject_destroyed(std::size_t flag_ptr) noexcept; // implemented in rust side via cxx
void connect_destroyed_callback(QObject *obj, std::uintptr_t flag_ptr);

#endif // QML_METHOD_INVOKER_H
