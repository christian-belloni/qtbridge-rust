// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef DYNAMICMETAOBJECTDATA_H
#define DYNAMICMETAOBJECTDATA_H
#include <QMetaObject>
#include <memory>
#include "rust/cxx.h"

class MetaMethodIncomingParams;
class MetaMethodOutgoingParams;
class QMetaType;
class QObject;
class QVariant;

class DynamicMetaObjectData
{
public:
    // Rust callbacks to be passed across the bridge
    using PropertyGetterFn = rust::Fn<QVariant(uint8_t* receiver)>;
    using PropertySetterFn = rust::Fn<void(uint8_t* receiver, const QVariant& value)>;
    using SlotCallbackFn   = rust::Fn<void(uint8_t* receiver, const MetaMethodIncomingParams& params)>;

    DynamicMetaObjectData(const QMetaObject* staticMetaObj, rust::Str className);

    void setToQObject(QObject& dst) const;
    const QMetaObject* getDynamicQMetaObject() const;

    void addClassInfo(rust::Str name, rust::Str value);
    void registerProperty(rust::Str name, const QMetaType& metaType, PropertyGetterFn getter, PropertySetterFn setter, rust::Str notifySignal);
    void registerPropertyReadOnly(rust::Str name, const QMetaType& metaType, PropertyGetterFn getter, bool isConstant, rust::Str notifySignal);
    void registerSignal(rust::Str name, rust::Slice<const QMetaType> argMetaTypes);
    void registerSlot(rust::Str name, rust::Slice<const QMetaType> argMetaTypes, SlotCallbackFn callback);
    void endMetaRegistration();

    void emitSignal(QObject& obj, rust::Str name, const MetaMethodOutgoingParams& params) const;

private:
    // Use pimpl idiom not to expose private APIs
    // and hide implementation details
    class Impl;
    std::unique_ptr<Impl> m_impl;
};

DynamicMetaObjectData *createDynamicMetaObjectData(rust::Str rustStructName, const QMetaObject& staticMeta);

#endif // #ifndef DYNAMICMETAOBJECTDATA_H
