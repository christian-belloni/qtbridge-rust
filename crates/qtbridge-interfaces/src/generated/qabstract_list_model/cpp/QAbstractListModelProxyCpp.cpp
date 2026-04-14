// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "QAbstractListModelProxyCpp.h"

namespace rust::bridge {

QAbstractListModelProxyCpp::QAbstractListModelProxyCpp(QAbstractListModelProxyRust* rustProxy)
    : m_rustProxy(rustProxy)
{}
QAbstractListModelProxyCpp::~QAbstractListModelProxyCpp()
{
    QAbstractListModelProxyRust::dropSelf(m_rustProxy);
}

// Virtual methods
QModelIndex QAbstractListModelProxyCpp::index(int32_t row, int32_t column, const QModelIndex& parent) const
{
    return m_rustProxy->index(row, column, parent);
}
int32_t QAbstractListModelProxyCpp::rowCount(const QModelIndex& parent) const
{
    return m_rustProxy->rowCount(parent);
}
QVariant QAbstractListModelProxyCpp::data(const QModelIndex& index, int32_t role) const
{
    return m_rustProxy->data(index, role);
}
QHash<int32_t,QByteArray> QAbstractListModelProxyCpp::roleNames() const
{
    return m_rustProxy->roleNames();
}
bool QAbstractListModelProxyCpp::setData(const QModelIndex& index, const QVariant& value, int32_t role)
{
    return m_rustProxy->setData(index, value, role);
}
bool QAbstractListModelProxyCpp::removeRows(int32_t first, int32_t count, const QModelIndex& parent)
{
    return m_rustProxy->removeRows(first, count, parent);
}
QModelIndex QAbstractListModelProxyCpp::sibling(int32_t row, int32_t column, const QModelIndex& idx) const
{
    return m_rustProxy->sibling(row, column, idx);
}


// DispatchMetaCallCpp implementation
void QAbstractListModelProxyCpp::invokeSlot(uint32_t slotId, rust::Slice<const uint8_t *const> inputs, rust::Slice<uint8_t* const> outputs) const
{
    m_rustProxy->invokeSlot(slotId, inputs, outputs);
}

void QAbstractListModelProxyCpp::invokeSlotMut(uint32_t slotId, rust::Slice<const uint8_t *const> inputs, rust::Slice<uint8_t* const> outputs) const
{
    m_rustProxy->invokeSlotMut(slotId, inputs, outputs);
}

QVariant QAbstractListModelProxyCpp::readProperty(uint32_t propId) const
{
    return m_rustProxy->readProperty(propId);
}

void QAbstractListModelProxyCpp::writeProperty(uint32_t propId, const QVariant& value) const
{
    m_rustProxy->writeProperty(propId, value);
}


// Access to base implementation of virtual functions
QModelIndex QAbstractListModelProxyCpp::base_index(int32_t row, int32_t column, const QModelIndex& parent) const
{
    return Base::index(row, column, parent);
}
QHash<int32_t,QByteArray> QAbstractListModelProxyCpp::base_roleNames() const
{
    return Base::roleNames();
}
bool QAbstractListModelProxyCpp::base_setData(const QModelIndex& index, const QVariant& value, int32_t role)
{
    return Base::setData(index, value, role);
}
bool QAbstractListModelProxyCpp::base_removeRows(int32_t first, int32_t count, const QModelIndex& parent)
{
    return Base::removeRows(first, count, parent);
}
QModelIndex QAbstractListModelProxyCpp::base_sibling(int32_t row, int32_t column, const QModelIndex& idx) const
{
    return Base::sibling(row, column, idx);
}


// Access to base implementation of non virtual functions
void QAbstractListModelProxyCpp::dataChanged(const QModelIndex& topLeft, const QModelIndex& bottomRight)
{
    Base::dataChanged(topLeft, bottomRight);
}
void QAbstractListModelProxyCpp::beginInsertRows(const QModelIndex& parent, int32_t first, int32_t last)
{
    Base::beginInsertRows(parent, first, last);
}
void QAbstractListModelProxyCpp::endInsertRows()
{
    Base::endInsertRows();
}
void QAbstractListModelProxyCpp::beginMoveRows(const QModelIndex& sourceParent, int32_t sourceFirst, int32_t sourceLast, const QModelIndex& destinationParent, int32_t destinationChild)
{
    Base::beginMoveRows(sourceParent, sourceFirst, sourceLast, destinationParent, destinationChild);
}
void QAbstractListModelProxyCpp::endMoveRows()
{
    Base::endMoveRows();
}
void QAbstractListModelProxyCpp::beginRemoveRows(const QModelIndex& parent, int32_t first, int32_t last)
{
    Base::beginRemoveRows(parent, first, last);
}
void QAbstractListModelProxyCpp::endRemoveRows()
{
    Base::endRemoveRows();
}
void QAbstractListModelProxyCpp::beginResetModel()
{
    Base::beginResetModel();
}
void QAbstractListModelProxyCpp::endResetModel()
{
    Base::endResetModel();
}



// Functions for object construction

QAbstractListModelProxyCpp* create_QAbstractListModelProxyCpp(QAbstractListModelProxyRust* rustProxy)
{
    return new QAbstractListModelProxyCpp(rustProxy);
}

QAbstractListModelProxyCpp* create_QAbstractListModelProxyCpp_At(uint8_t* addr, QAbstractListModelProxyRust* rustProxy)
{
    return new (addr) QAbstractListModelProxyCpp(rustProxy);
}

const QMetaObject& staticQMetaObjectOf_QAbstractListModelProxyCpp()
{
    return QAbstractListModel::staticMetaObject;
}

size_t sizeOf_QAbstractListModelProxyCpp()
{
    return sizeof(QAbstractListModelProxyCpp);
}

size_t alignOf_QAbstractListModelProxyCpp()
{
    return alignof(QAbstractListModelProxyCpp);
}

QMetaType qmetaTypeListOf_QAbstractListModelProxyCpp()
{
    return QMetaType::fromType<QQmlListProperty<QAbstractListModelProxyCpp>>();
}

} // namespace rust::bridge
