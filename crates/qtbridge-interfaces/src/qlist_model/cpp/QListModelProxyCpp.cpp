// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "QListModelProxyCpp.h"

namespace rust::bridge {

QListModelProxyCpp::QListModelProxyCpp(QListModelProxyRust* rustProxy)
    : m_rustProxy(rustProxy)
{}
QListModelProxyCpp::~QListModelProxyCpp()
{
    QListModelProxyRust::dropSelf(m_rustProxy);
}

void QListModelProxyCpp::emitSignal(rust::Str signalName, rust::Slice<const uint8_t* const> argv) const {
    auto* meta = dynamic_cast<DynamicMetaObjectData*>(QObjectPrivate::get(this)->metaObject);
    if (meta)
        meta->emitSignal(*const_cast<QListModelProxyCpp*>(this), signalName, argv);
    else
        qFatal() << "Error while emiting singal from Rust: The QObject does not contain a Rust dynamic meta object";
}

// Virtual methods
QModelIndex QListModelProxyCpp::index(int32_t row, int32_t column, const QModelIndex& parent) const
{
    return m_rustProxy->index(row, column, parent);
}
int32_t QListModelProxyCpp::rowCount(const QModelIndex& parent) const
{
    return m_rustProxy->rowCount(parent);
}
QVariant QListModelProxyCpp::data(const QModelIndex& index, int32_t role) const
{
    return m_rustProxy->data(index, role);
}
QHash<int32_t,QByteArray> QListModelProxyCpp::roleNames() const
{
    return m_rustProxy->roleNames();
}
bool QListModelProxyCpp::setData(const QModelIndex& index, const QVariant& value, int32_t role)
{
    return m_rustProxy->setData(index, value, role);
}
bool QListModelProxyCpp::removeRows(int32_t first, int32_t count, const QModelIndex& parent)
{
    return m_rustProxy->removeRows(first, count, parent);
}
QModelIndex QListModelProxyCpp::sibling(int32_t row, int32_t column, const QModelIndex& idx) const
{
    return m_rustProxy->sibling(row, column, idx);
}

// DispatchMetaCallCpp implementation
void QListModelProxyCpp::invokeSlot(uint32_t slotId, rust::Slice<const uint8_t *const> inputs, rust::Slice<uint8_t* const> outputs) const
{
    m_rustProxy->invokeSlot(slotId, inputs, outputs);
}

void QListModelProxyCpp::invokeSlotMut(uint32_t slotId, rust::Slice<const uint8_t *const> inputs, rust::Slice<uint8_t* const> outputs) const
{
    m_rustProxy->invokeSlotMut(slotId, inputs, outputs);
}

QVariant QListModelProxyCpp::readProperty(uint32_t propId) const
{
    return m_rustProxy->readProperty(propId);
}

void QListModelProxyCpp::writeProperty(uint32_t propId, const QVariant& value) const
{
    m_rustProxy->writeProperty(propId, value);
}


// RustObjectGetter implementation
const void* QListModelProxyCpp::getRustObjectRcPtr() const
{
    return static_cast<const void*>(m_rustProxy->getRustObjectRcPtr());
}

// Access to base implementation of virtual functions
QModelIndex QListModelProxyCpp::base_index(int32_t row, int32_t column, const QModelIndex& parent) const
{
    return Base::index(row, column, parent);
}
QHash<int32_t,QByteArray> QListModelProxyCpp::base_roleNames() const
{
    return Base::roleNames();
}
bool QListModelProxyCpp::base_setData(const QModelIndex& index, const QVariant& value, int32_t role)
{
    return Base::setData(index, value, role);
}
bool QListModelProxyCpp::base_removeRows(int32_t first, int32_t count, const QModelIndex& parent)
{
    return Base::removeRows(first, count, parent);
}
QModelIndex QListModelProxyCpp::base_sibling(int32_t row, int32_t column, const QModelIndex& idx) const
{
    return Base::sibling(row, column, idx);
}

// Access to base implementation of non virtual functions
void QListModelProxyCpp::dataChanged(const QModelIndex& topLeft, const QModelIndex& bottomRight)
{
    Base::dataChanged(topLeft, bottomRight);
}
void QListModelProxyCpp::beginInsertRows(const QModelIndex& parent, int32_t first, int32_t last)
{
    Base::beginInsertRows(parent, first, last);
}
void QListModelProxyCpp::endInsertRows()
{
    Base::endInsertRows();
}
void QListModelProxyCpp::beginMoveRows(const QModelIndex& sourceParent, int32_t sourceFirst, int32_t sourceLast, const QModelIndex& destinationParent, int32_t destinationChild)
{
    Base::beginMoveRows(sourceParent, sourceFirst, sourceLast, destinationParent, destinationChild);
}
void QListModelProxyCpp::endMoveRows()
{
    Base::endMoveRows();
}
void QListModelProxyCpp::beginRemoveRows(const QModelIndex& parent, int32_t first, int32_t last)
{
    Base::beginRemoveRows(parent, first, last);
}
void QListModelProxyCpp::endRemoveRows()
{
    Base::endRemoveRows();
}
void QListModelProxyCpp::beginResetModel()
{
    Base::beginResetModel();
}
void QListModelProxyCpp::endResetModel()
{
    Base::endResetModel();
}

// Functions for object construction

QListModelProxyCpp* create_QListModelProxyCpp(QListModelProxyRust* rustProxy, const DynamicMetaObjectData* metaObject)
{
    auto proxy = new QListModelProxyCpp(rustProxy);
    QObjectPrivate::get(proxy)->metaObject = const_cast<DynamicMetaObjectData*>(metaObject);
    return proxy;
}

QListModelProxyCpp* create_QListModelProxyCpp_At(QListModelProxyRust* rustProxy, const DynamicMetaObjectData* metaObject, uint8_t* addr)
{
    auto proxy = new (addr) QListModelProxyCpp(rustProxy);
    QObjectPrivate::get(proxy)->metaObject = const_cast<DynamicMetaObjectData*>(metaObject);
    return proxy;
}

const QMetaObject& staticQMetaObjectOf_QListModelProxyCpp()
{
    return QListModelProxyCpp::staticMetaObject;
}

size_t sizeOf_QListModelProxyCpp()
{
    return sizeof(QListModelProxyCpp);
}

size_t alignOf_QListModelProxyCpp()
{
    return alignof(QListModelProxyCpp);
}

QMetaType qmetaTypeListOf_QListModelProxyCpp()
{
    return QMetaType::fromType<QQmlListProperty<QListModelProxyCpp>>();
}

} // namespace rust::bridge
