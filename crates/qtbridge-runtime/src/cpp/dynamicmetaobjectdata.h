// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef DYNAMICMETAOBJECTDATA_H
#define DYNAMICMETAOBJECTDATA_H
#include <private/qobject_p.h>
#include <QByteArray>
#include <QMetaObject>
#include <QMetaType>
#include <QScopedPointer>
#include <cstdint>
#include <map>
#include <memory>
#include <optional>
#include "rust/cxx.h"

class DispatchMetaCallCpp;

enum class Mutability: int8_t {
    Immutable = 0,
    Mutable = 1,
};

/**
 * Class that stores dynamic `QMetaObject` built by `DynamicMetaObjectBuilder`.
 *
 * Handles meta calls using custom handlers, forwards calls to Rust via the `DispatchMetaCallCpp` interface.
 * Forward signal emission to Qt internals.
*/
class DynamicMetaObjectData: public QDynamicMetaObjectData
{
public:
    DynamicMetaObjectData() = default;

    void addProperty(int index, const QByteArray& name, uint32_t userId, const QMetaType& metaType);
    void addSignal(int index, const QByteArray& name);
    void addSlot(int index, const QByteArray& name, uint32_t userId, Mutability mutability);

    void emitSignal(QObject& obj, rust::Str name, rust::Slice<const uint8_t* const> argvSlice) const;

    void setToQObject(QObject& dst) const;
    QMetaObject* getMetaObject() const;

    std::optional<int> getSignalIndex(const QByteArray& name) const;

    void setMetaObject(std::unique_ptr<QMetaObject, QScopedPointerPodDeleter> metaObject);
    bool isMetaObjectSet() const;

private:
    void objectDestroyed(QObject *) override;
    QMetaObject* toDynamicMetaObject(QObject* o) override;
    int metaCall(QObject* o, QMetaObject::Call call, int id, void** argv) override;

    bool handleMetaCallInvoke(QObject* o, DispatchMetaCallCpp& dispatch, int id, void** argv);
    bool handleMetaCallReadProperty(const DispatchMetaCallCpp& dispatch, int id, void** argv);
    bool handleMetaCallWriteProperty(DispatchMetaCallCpp& dispatch, int id, void** argv);

private:
    struct PropertyInfo
    {
        uint32_t m_userId;
        QMetaType m_type;
    };

    struct SignalInfo
    {
        QByteArray m_name;
    };

    struct SlotInfo
    {
        // Id of the slot handler on the Rust side (used corresponding match operator arm).
        uint32_t m_userId;

        // Indicates whether the Rust-side slot method takes `&self` or `&mut self`.
        Mutability m_mutability;
    };

private:
    std::map<int, PropertyInfo> m_properties;
    std::map<int, SignalInfo> m_signals;
    std::map<int, SlotInfo> m_slots;
    std::unique_ptr<QMetaObject, QScopedPointerPodDeleter> m_metaObject;
};

#endif // DYNAMICMETAOBJECTDATA_H
