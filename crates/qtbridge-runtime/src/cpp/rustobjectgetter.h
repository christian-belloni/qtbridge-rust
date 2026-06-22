// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#ifndef _RUST_OBJECT_GETTER_
#define _RUST_OBJECT_GETTER_
#include <cstdint>

class QObject;

class RustObjectGetter
{
public:
    virtual ~RustObjectGetter() = default;

    virtual const void* getRustProxy() const = 0;
};

const uint8_t* getRustProxy(const QObject& qobj);

#endif // _RUST_OBJECT_GETTER_
