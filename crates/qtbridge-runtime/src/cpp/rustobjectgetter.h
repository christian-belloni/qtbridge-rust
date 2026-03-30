// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _RUST_OBJECT_GETTER_
#define _RUST_OBJECT_GETTER_
#include <cstdint>

class RustObjectGetter
{
public:
    RustObjectGetter(uint8_t* rustObj)
        : m_rustObj(rustObj)
    {}

    virtual ~RustObjectGetter() = default;

    uint8_t* getRustObject() const
    {
        return m_rustObj;
    }

protected:
    uint8_t* m_rustObj;
};

#endif // _RUST_OBJECT_GETTER_
