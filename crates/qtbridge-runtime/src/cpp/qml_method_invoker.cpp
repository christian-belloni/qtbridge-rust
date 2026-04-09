// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "qml_method_invoker.h"

void connect_destroyed_callback(QObject *obj, std::uintptr_t flag_ptr)
{
    QObject::connect(obj, &QObject::destroyed, [flag_ptr](QObject *) {
        on_qobject_destroyed(flag_ptr);
    });
}
