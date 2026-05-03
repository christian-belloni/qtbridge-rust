// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "QAbstractItemModelProxyCpp.h"

namespace rust::bridge {

QAbstractItemModelProxyCpp::QAbstractItemModelProxyCpp(QAbstractItemModelProxyRust* rustProxy)
    : m_rustProxy(rustProxy)
{}
QAbstractItemModelProxyCpp::~QAbstractItemModelProxyCpp()
{
    QAbstractItemModelProxyRust::dropSelf(m_rustProxy);
}

// Virtual methods
QModelIndex QAbstractItemModelProxyCpp::index(int32_t row, int32_t column, const QModelIndex& parent) const
{
    return m_rustProxy->index(row, column, parent);
}
QModelIndex QAbstractItemModelProxyCpp::parent(const QModelIndex& child) const
{
    return m_rustProxy->parent(child);
}
int32_t QAbstractItemModelProxyCpp::rowCount(const QModelIndex& parent) const
{
    return m_rustProxy->rowCount(parent);
}
int32_t QAbstractItemModelProxyCpp::columnCount(const QModelIndex& parent) const
{
    return m_rustProxy->columnCount(parent);
}
QVariant QAbstractItemModelProxyCpp::data(const QModelIndex& index, int32_t role) const
{
    return m_rustProxy->data(index, role);
}
QHash<int32_t,QByteArray> QAbstractItemModelProxyCpp::roleNames() const
{
    return m_rustProxy->roleNames();
}
bool QAbstractItemModelProxyCpp::setData(const QModelIndex& index, const QVariant& value, int32_t role)
{
    return m_rustProxy->setData(index, value, role);
}
bool QAbstractItemModelProxyCpp::removeRows(int32_t first, int32_t count, const QModelIndex& parent)
{
    return m_rustProxy->removeRows(first, count, parent);
}
QModelIndex QAbstractItemModelProxyCpp::sibling(int32_t row, int32_t column, const QModelIndex& idx) const
{
    return m_rustProxy->sibling(row, column, idx);
}

// DispatchMetaCallCpp implementation
void QAbstractItemModelProxyCpp::invokeSlot(uint32_t slotId, rust::Slice<const uint8_t *const> inputs, rust::Slice<uint8_t* const> outputs) const
{
    m_rustProxy->invokeSlot(slotId, inputs, outputs);
}

void QAbstractItemModelProxyCpp::invokeSlotMut(uint32_t slotId, rust::Slice<const uint8_t *const> inputs, rust::Slice<uint8_t* const> outputs) const
{
    m_rustProxy->invokeSlotMut(slotId, inputs, outputs);
}

QVariant QAbstractItemModelProxyCpp::readProperty(uint32_t propId) const
{
    return m_rustProxy->readProperty(propId);
}

void QAbstractItemModelProxyCpp::writeProperty(uint32_t propId, const QVariant& value) const
{
    m_rustProxy->writeProperty(propId, value);
}


// RustObjectGetter implementation
const void* QAbstractItemModelProxyCpp::getRustObjectRcPtr() const
{
    return static_cast<const void*>(m_rustProxy->getRustObjectRcPtr());
}


// Access to base implementation of virtual functions
QHash<int32_t,QByteArray> QAbstractItemModelProxyCpp::base_roleNames() const
{
    return Base::roleNames();
}
bool QAbstractItemModelProxyCpp::base_setData(const QModelIndex& index, const QVariant& value, int32_t role)
{
    return Base::setData(index, value, role);
}
bool QAbstractItemModelProxyCpp::base_removeRows(int32_t first, int32_t count, const QModelIndex& parent)
{
    return Base::removeRows(first, count, parent);
}
QModelIndex QAbstractItemModelProxyCpp::base_sibling(int32_t row, int32_t column, const QModelIndex& idx) const
{
    return Base::sibling(row, column, idx);
}


// Access to base implementation of non virtual functions
void QAbstractItemModelProxyCpp::dataChanged(const QModelIndex& topLeft, const QModelIndex& bottomRight)
{
    Base::dataChanged(topLeft, bottomRight);
}
void QAbstractItemModelProxyCpp::beginInsertColumns(const QModelIndex& parent, int32_t first, int32_t last)
{
    Base::beginInsertColumns(parent, first, last);
}
void QAbstractItemModelProxyCpp::endInsertColumns()
{
    Base::endInsertColumns();
}
void QAbstractItemModelProxyCpp::beginInsertRows(const QModelIndex& parent, int32_t first, int32_t last)
{
    Base::beginInsertRows(parent, first, last);
}
void QAbstractItemModelProxyCpp::endInsertRows()
{
    Base::endInsertRows();
}
void QAbstractItemModelProxyCpp::beginMoveColumns(const QModelIndex& sourceParent, int32_t sourceFirst, int32_t sourceLast, const QModelIndex& destinationParent, int32_t destinationChild)
{
    Base::beginMoveColumns(sourceParent, sourceFirst, sourceLast, destinationParent, destinationChild);
}
void QAbstractItemModelProxyCpp::endMoveColumns()
{
    Base::endMoveColumns();
}
void QAbstractItemModelProxyCpp::beginMoveRows(const QModelIndex& sourceParent, int32_t sourceFirst, int32_t sourceLast, const QModelIndex& destinationParent, int32_t destinationChild)
{
    Base::beginMoveRows(sourceParent, sourceFirst, sourceLast, destinationParent, destinationChild);
}
void QAbstractItemModelProxyCpp::endMoveRows()
{
    Base::endMoveRows();
}
void QAbstractItemModelProxyCpp::beginRemoveColumns(const QModelIndex& parent, int32_t first, int32_t last)
{
    Base::beginRemoveColumns(parent, first, last);
}
void QAbstractItemModelProxyCpp::endRemoveColumns()
{
    Base::endRemoveColumns();
}
void QAbstractItemModelProxyCpp::beginRemoveRows(const QModelIndex& parent, int32_t first, int32_t last)
{
    Base::beginRemoveRows(parent, first, last);
}
void QAbstractItemModelProxyCpp::endRemoveRows()
{
    Base::endRemoveRows();
}
void QAbstractItemModelProxyCpp::beginResetModel()
{
    Base::beginResetModel();
}
void QAbstractItemModelProxyCpp::endResetModel()
{
    Base::endResetModel();
}
QModelIndex QAbstractItemModelProxyCpp::createIndex(int32_t row, int32_t column, size_t ptr) const
{
    return Base::createIndex(row, column, ptr);
}



// Functions for object construction

QAbstractItemModelProxyCpp* create_QAbstractItemModelProxyCpp(QAbstractItemModelProxyRust* rustProxy)
{
    return new QAbstractItemModelProxyCpp(rustProxy);
}

QAbstractItemModelProxyCpp* create_QAbstractItemModelProxyCpp_At(uint8_t* addr, QAbstractItemModelProxyRust* rustProxy)
{
    return new (addr) QAbstractItemModelProxyCpp(rustProxy);
}

const QMetaObject& staticQMetaObjectOf_QAbstractItemModelProxyCpp()
{
    return QAbstractItemModel::staticMetaObject;
}

size_t sizeOf_QAbstractItemModelProxyCpp()
{
    return sizeof(QAbstractItemModelProxyCpp);
}

size_t alignOf_QAbstractItemModelProxyCpp()
{
    return alignof(QAbstractItemModelProxyCpp);
}

QMetaType qmetaTypeListOf_QAbstractItemModelProxyCpp()
{
    return QMetaType::fromType<QQmlListProperty<QAbstractItemModelProxyCpp>>();
}

} // namespace rust::bridge
