// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "QAIMProxyCpp.h"

namespace rust::bridge {

QAIMProxyCpp::QAIMProxyCpp(uint8_t* rustObj, QAIMProxyRust* rustProxy)
    : m_rustObj(rustObj)
    , m_rustProxy(rustProxy)
{}
QAIMProxyCpp::~QAIMProxyCpp()
{
    QAIMProxyRust::dropSelf(m_rustProxy, m_rustObj);
}

// DispatchMetaCallCpp implementation
void QAIMProxyCpp::invokeSlot(uint32_t slotId, rust::Slice<const uint8_t *const> inputs, rust::Slice<uint8_t* const> outputs) const
{
    m_rustProxy->invokeSlot(slotId, inputs, outputs);
}

void QAIMProxyCpp::invokeSlotMut(uint32_t slotId, rust::Slice<const uint8_t *const> inputs, rust::Slice<uint8_t* const> outputs) const
{
    m_rustProxy->invokeSlotMut(slotId, inputs, outputs);
}

QVariant QAIMProxyCpp::readProperty(uint32_t propId) const
{
    return m_rustProxy->readProperty(propId);
}

void QAIMProxyCpp::writeProperty(uint32_t propId, const QVariant& value) const
{
    m_rustProxy->writeProperty(propId, value);
}

// Virtual methods
QModelIndex QAIMProxyCpp::index(int32_t row, int32_t column, const QModelIndex& parent) const
{
    return m_rustProxy->index(row, column, parent);
}
QModelIndex QAIMProxyCpp::parent(const QModelIndex& child) const
{
    return m_rustProxy->parent(child);
}
int32_t QAIMProxyCpp::rowCount(const QModelIndex& parent) const
{
    return m_rustProxy->rowCount(parent);
}
int32_t QAIMProxyCpp::columnCount(const QModelIndex& parent) const
{
    return m_rustProxy->columnCount(parent);
}
QVariant QAIMProxyCpp::data(const QModelIndex& index, int32_t role) const
{
    return m_rustProxy->data(index, role);
}
QHash<int32_t,QByteArray> QAIMProxyCpp::roleNames() const
{
    return m_rustProxy->roleNames();
}
bool QAIMProxyCpp::setData(const QModelIndex& index, const QVariant& value, int32_t role)
{
    return m_rustProxy->setData(index, value, role);
}
bool QAIMProxyCpp::removeRows(int32_t first, int32_t count, const QModelIndex& parent)
{
    return m_rustProxy->removeRows(first, count, parent);
}
QModelIndex QAIMProxyCpp::sibling(int32_t row, int32_t column, const QModelIndex& idx) const
{
    return m_rustProxy->sibling(row, column, idx);
}


// Access to base implementation of virtual functions
QHash<int32_t,QByteArray> QAIMProxyCpp::base_roleNames() const
{
    return Base::roleNames();
}
bool QAIMProxyCpp::base_setData(const QModelIndex& index, const QVariant& value, int32_t role)
{
    return Base::setData(index, value, role);
}
bool QAIMProxyCpp::base_removeRows(int32_t first, int32_t count, const QModelIndex& parent)
{
    return Base::removeRows(first, count, parent);
}
bool QAIMProxyCpp::base_removeColumns(int32_t first, int32_t count, const QModelIndex& parent)
{
    return Base::removeColumns(first, count, parent);
}
QModelIndex QAIMProxyCpp::base_sibling(int32_t row, int32_t column, const QModelIndex& idx) const
{
    return Base::sibling(row, column, idx);
}


// Access to base implementation of non virtual functions
void QAIMProxyCpp::dataChanged(const QModelIndex& topLeft, const QModelIndex& bottomRight)
{
    Base::dataChanged(topLeft, bottomRight);
}
void QAIMProxyCpp::beginInsertColumns(const QModelIndex& parent, int32_t first, int32_t last)
{
    Base::beginInsertColumns(parent, first, last);
}
void QAIMProxyCpp::endInsertColumns()
{
    Base::endInsertColumns();
}
void QAIMProxyCpp::beginInsertRows(const QModelIndex& parent, int32_t first, int32_t last)
{
    Base::beginInsertRows(parent, first, last);
}
void QAIMProxyCpp::endInsertRows()
{
    Base::endInsertRows();
}
void QAIMProxyCpp::beginMoveColumns(const QModelIndex& sourceParent, int32_t sourceFirst, int32_t sourceLast, const QModelIndex& destinationParent, int32_t destinationChild)
{
    Base::beginMoveColumns(sourceParent, sourceFirst, sourceLast, destinationParent, destinationChild);
}
void QAIMProxyCpp::endMoveColumns()
{
    Base::endMoveColumns();
}
void QAIMProxyCpp::beginMoveRows(const QModelIndex& sourceParent, int32_t sourceFirst, int32_t sourceLast, const QModelIndex& destinationParent, int32_t destinationChild)
{
    Base::beginMoveRows(sourceParent, sourceFirst, sourceLast, destinationParent, destinationChild);
}
void QAIMProxyCpp::endMoveRows()
{
    Base::endMoveRows();
}
void QAIMProxyCpp::beginRemoveColumns(const QModelIndex& parent, int32_t first, int32_t last)
{
    Base::beginRemoveColumns(parent, first, last);
}
void QAIMProxyCpp::endRemoveColumns()
{
    Base::endRemoveColumns();
}
void QAIMProxyCpp::beginRemoveRows(const QModelIndex& parent, int32_t first, int32_t last)
{
    Base::beginRemoveRows(parent, first, last);
}
void QAIMProxyCpp::endRemoveRows()
{
    Base::endRemoveRows();
}
void QAIMProxyCpp::beginResetModel()
{
    Base::beginResetModel();
}
void QAIMProxyCpp::endResetModel()
{
    Base::endResetModel();
}
QModelIndex QAIMProxyCpp::createIndex(int32_t row, int32_t column, size_t ptr) const
{
    return Base::createIndex(row, column, ptr);
}



// Functions for object construction

QAIMProxyCpp* create_QAIMProxyCpp(uint8_t* rustObj, QAIMProxyRust* rustProxy)
{
    return new QAIMProxyCpp(rustObj, rustProxy);
}

QAIMProxyCpp* create_QAIMProxyCpp_At(uint8_t* addr, uint8_t* rustObj, QAIMProxyRust* rustProxy)
{
    return new (addr) QAIMProxyCpp(rustObj, rustProxy);
}

const QMetaObject& staticQMetaObjectOf_QAIMProxyCpp()
{
    return QAbstractItemModel::staticMetaObject;
}

size_t sizeOf_QAIMProxyCpp()
{
    return sizeof(QAIMProxyCpp);
}

size_t alignOf_QAIMProxyCpp()
{
    return alignof(QAIMProxyCpp);
}

QMetaType qmetaTypeListOf_QAIMProxyCpp()
{
    return QMetaType::fromType<QQmlListProperty<QAIMProxyCpp>>();
}

} // namespace rust::bridge
