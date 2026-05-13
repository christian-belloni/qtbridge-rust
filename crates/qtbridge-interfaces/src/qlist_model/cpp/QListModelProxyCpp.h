// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QLISTMODELPROXYCPP_RUST_BRIDGE__H_
#define _QLISTMODELPROXYCPP_RUST_BRIDGE__H_
#include <QAbstractListModel>
#include <QMetaObject>
#include <QQmlListProperty>
#include <cstdint>
#include "qtbridge-runtime/src/cpp/dispatchmetacallcpp.h"
#include "qtbridge-runtime/src/cpp/dynamicmetaobjectdata.h"
#include "qtbridge-runtime/src/cpp/rustobjectgetter.h"
#include "qtbridge-interfaces/src/qlist_model/proxy_rust_bridge.rs.h"
#include "qtbridge-type-lib/src/generated/core/qbytearray/cpp/qbytearray.h"
#include "qtbridge-type-lib/src/generated/core/qhash/cpp/qhash_i32_qbytearray.h"
#include "qtbridge-type-lib/src/generated/core/qmodelindex/cpp/qmodelindex.h"
#include "qtbridge-type-lib/src/generated/core/qvariant/cpp/qvariant.h"


namespace rust::bridge {

class QListModelProxyCpp : public QAbstractListModel, public DispatchMetaCallCpp, public RustObjectGetter
{
    using Base = QAbstractListModel;
public:
    QListModelProxyCpp(QListModelProxyRust* rustProxy);
    ~QListModelProxyCpp();

    void emitSignal(rust::Str signalName, rust::Slice<const uint8_t* const> argv) const;
    void emitSignalMut(rust::Str signalName, rust::Slice<const uint8_t* const> argv);

    // Virtual methods
    QModelIndex index(int32_t row, int32_t column, const QModelIndex& parent) const override;
    int32_t rowCount(const QModelIndex& parent) const override;
    QVariant data(const QModelIndex& index, int32_t role) const override;
    QHash<int32_t,QByteArray> roleNames() const override;
    bool setData(const QModelIndex& index, const QVariant& value, int32_t role) override;
    bool removeRows(int32_t first, int32_t count, const QModelIndex& parent) override;
    QModelIndex sibling(int32_t row, int32_t column, const QModelIndex& idx) const override;

    // DispatchMetaCallCpp implementation
    void invokeSlot(uint32_t slotId, rust::Slice<const uint8_t *const> inputs, rust::Slice<uint8_t* const> outputs) const override;
    void invokeSlotMut(uint32_t slotId, rust::Slice<const uint8_t *const> inputs, rust::Slice<uint8_t* const> outputs) const override;
    QVariant readProperty(uint32_t propId) const override;
    void writeProperty(uint32_t propId, const QVariant& value) const override;

    // RustObjectGetter implementation
    const void* getRustObjectRcPtr() const override;

    // Access to base implementation of virtual functions
    QModelIndex base_index(int32_t row, int32_t column, const QModelIndex& parent) const;
    QHash<int32_t,QByteArray> base_roleNames() const;
    bool base_setData(const QModelIndex& index, const QVariant& value, int32_t role);
    bool base_removeRows(int32_t first, int32_t count, const QModelIndex& parent);
    QModelIndex base_sibling(int32_t row, int32_t column, const QModelIndex& idx) const;

    // Access to base implementation of non virtual functions
    void dataChanged(const QModelIndex& topLeft, const QModelIndex& bottomRight);
    void beginInsertRows(const QModelIndex& parent, int32_t first, int32_t last);
    void endInsertRows();
    void beginMoveRows(const QModelIndex& sourceParent, int32_t sourceFirst, int32_t sourceLast, const QModelIndex& destinationParent, int32_t destinationChild);
    void endMoveRows();
    void beginRemoveRows(const QModelIndex& parent, int32_t first, int32_t last);
    void endRemoveRows();
    void beginResetModel();
    void endResetModel();

private:
    QListModelProxyRust* m_rustProxy;
};

// Functions for object construction
QListModelProxyCpp* create_QListModelProxyCpp(QListModelProxyRust* rustProxy, const DynamicMetaObjectData* metaObject);
QListModelProxyCpp* create_QListModelProxyCpp_At(QListModelProxyRust* rustProxy, const DynamicMetaObjectData* metaObject, uint8_t* addr);
const QMetaObject& staticQMetaObjectOf_QListModelProxyCpp();
size_t sizeOf_QListModelProxyCpp();
size_t alignOf_QListModelProxyCpp();
QMetaType qmetaTypeListOf_QListModelProxyCpp();

} // namespace rust::bridge

#endif // _QLISTMODELPROXYCPP_RUST_BRIDGE__H_
