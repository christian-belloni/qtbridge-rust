// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _DISPATCH_META_CALL_CPP_
#define _DISPATCH_META_CALL_CPP_
#include <QVariant>
#include <cstdint>
#include "rust/cxx.h"

class DispatchMetaCallCpp
{
public:
    virtual ~DispatchMetaCallCpp() = default;

    virtual void invokeSlot(uint32_t slotId, rust::Slice<const uint8_t *const> inputs, rust::Slice<uint8_t* const> outputs) const = 0;
    virtual void invokeSlotMut(uint32_t slotId, rust::Slice<const uint8_t *const> inputs, rust::Slice<uint8_t* const> outputs) const = 0;
    virtual QVariant readProperty(uint32_t propId) const = 0;
    virtual void writeProperty(uint32_t propId, const QVariant& value) const = 0;
};

#endif // _DISPATCH_META_CALL_CPP_
