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
    void addSlot(int index, const QByteArray& name, uint32_t userId, bool is_mutable);

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
        uint32_t m_userId;
        bool m_isMutable;
    };

private:
    std::map<int, PropertyInfo> m_properties;
    std::map<int, SignalInfo> m_signals;
    std::map<int, SlotInfo> m_slots;
    std::unique_ptr<QMetaObject, QScopedPointerPodDeleter> m_metaObject;
};

#endif // DYNAMICMETAOBJECTDATA_H
