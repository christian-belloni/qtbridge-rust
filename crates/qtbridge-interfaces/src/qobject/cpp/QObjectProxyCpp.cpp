// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "QObjectProxyCpp.h"

namespace rust::bridge {

QObjectProxyCpp::QObjectProxyCpp(QObjectProxyRust* rustProxy)
    : m_rustProxy(rustProxy)
{}
QObjectProxyCpp::~QObjectProxyCpp()
{
    QObjectProxyRust::dropSelf(m_rustProxy);
}


// DispatchMetaCallCpp implementation
void QObjectProxyCpp::invokeSlot(uint32_t slotId, rust::Slice<const uint8_t *const> inputs, rust::Slice<uint8_t* const> outputs) const
{
    m_rustProxy->invokeSlot(slotId, inputs, outputs);
}

void QObjectProxyCpp::invokeSlotMut(uint32_t slotId, rust::Slice<const uint8_t *const> inputs, rust::Slice<uint8_t* const> outputs) const
{
    m_rustProxy->invokeSlotMut(slotId, inputs, outputs);
}

QVariant QObjectProxyCpp::readProperty(uint32_t propId) const
{
    return m_rustProxy->readProperty(propId);
}

void QObjectProxyCpp::writeProperty(uint32_t propId, const QVariant& value) const
{
    m_rustProxy->writeProperty(propId, value);
}


// RustObjectGetter implementation
const void* QObjectProxyCpp::getRustObjectRcPtr() const
{
    return static_cast<const void*>(m_rustProxy->getRustObjectRcPtr());
}


// Functions for object construction

QObjectProxyCpp* create_QObjectProxyCpp(QObjectProxyRust* rustProxy, const DynamicMetaObjectData* metaObject)
{
    auto proxy = new QObjectProxyCpp(rustProxy);
    QObjectPrivate::get(proxy)->metaObject = const_cast<DynamicMetaObjectData*>(metaObject);
    return proxy;
}

QObjectProxyCpp* create_QObjectProxyCpp_At(QObjectProxyRust* rustProxy, const DynamicMetaObjectData* metaObject, uint8_t* addr)
{
    auto proxy = new (addr) QObjectProxyCpp(rustProxy);
    QObjectPrivate::get(proxy)->metaObject = const_cast<DynamicMetaObjectData*>(metaObject);
    return proxy;
}

const QMetaObject& staticQMetaObjectOf_QObjectProxyCpp()
{
    return QObject::staticMetaObject;
}

size_t sizeOf_QObjectProxyCpp()
{
    return sizeof(QObjectProxyCpp);
}

size_t alignOf_QObjectProxyCpp()
{
    return alignof(QObjectProxyCpp);
}

QMetaType qmetaTypeListOf_QObjectProxyCpp()
{
    return QMetaType::fromType<QQmlListProperty<QObjectProxyCpp>>();
}

} // namespace rust::bridge
