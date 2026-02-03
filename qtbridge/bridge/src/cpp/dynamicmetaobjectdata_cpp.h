// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef DYNAMICMETAOBJECTDATA_CPP_H
#define DYNAMICMETAOBJECTDATA_CPP_H
#include <QByteArray>
#include <QMetaType>
#include <QVariant>
#include <memory>
#include <functional>

class MetaMethodIncomingParams;
class MetaMethodOutgoingParams;
class QObject;

class DynamicMetaObjectData_Cpp
{
public:
    // C++ callbacks
    using PropertyGetterFunc = std::function<QVariant(uint8_t* receiver)>;
    using PropertySetterFunc = std::function<void(uint8_t* receiver, const QVariant& value)>;
    using SlotFunc = std::function<void(uint8_t* receiver, const MetaMethodIncomingParams&)>;

    DynamicMetaObjectData_Cpp(const QMetaObject* staticMetaObj, const QByteArray& className);
    ~DynamicMetaObjectData_Cpp();

    void setToQObject(QObject& dst) const;
    const QMetaObject* getDynamicQMetaObject() const;

    void addClassInfo(const QByteArray& name, const QByteArray& value);
    // Registration of properties/signals/events
    void registerProperty  (const QByteArray& name, const QMetaType& metaType, PropertyGetterFunc&& getter, PropertySetterFunc&& setter, bool isConstant, const QByteArray& notifySignal);
    void registerSignal(const QByteArray& name, const std::vector<QMetaType>& argMetaTypes);
    void registerSlot(const QByteArray& name, const std::vector<QMetaType>& argMetaTypes, SlotFunc&& callback);
    void endMetaRegistration();

    void emitSignal(QObject* obj, const QByteArray& name, const MetaMethodOutgoingParams& params) const;

private:
    // Use pimpl idiom not to expose private APIs
    // and hide implementation details
    class Impl;
    std::unique_ptr<Impl> m_impl;
};

#endif // #ifndef DYNAMICMETAOBJECTDATA_CPP_H
