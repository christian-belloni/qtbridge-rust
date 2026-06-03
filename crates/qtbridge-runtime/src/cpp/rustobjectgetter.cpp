// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#include "rustobjectgetter.h"
#include <QObject>

const uint8_t* getRustObjectRcPtr(const QObject& qobj)
{
    if (auto* getter = dynamic_cast<const RustObjectGetter*>(&qobj))
        return static_cast<const uint8_t*>(getter->getRustObjectRcPtr());
    return nullptr;
}

const uint8_t* getRustProxy(const QObject& qobj)
{
    if (auto* getter = dynamic_cast<const RustObjectGetter*>(&qobj))
        return static_cast<const uint8_t*>(getter->getRustProxy());
    return nullptr;
}
