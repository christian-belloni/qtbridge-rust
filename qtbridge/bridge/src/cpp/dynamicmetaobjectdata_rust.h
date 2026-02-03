// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef DYNAMICMETAOBJECTDATA_RUST_H
#define DYNAMICMETAOBJECTDATA_RUST_H
#include <QMetaType>
#include <QVariant>
#include <memory>
#include "rust/cxx.h"

class DynamicMetaObjectData_Cpp;
class MetaMethodIncomingParams;
class MetaMethodOutgoingParams;
class QObject;

namespace rust::bridge
{

// Wrapper around DynamicMetaObjectData_Cpp that accepts Rust types, coverts them to Cpp types and forwards to DynamicMetaObjectData_Cpp
// The class is exposed to the Rust side
 class DynamicMetaObjectData_Rust
{
public:
// Rust callbacks to be passed across the bridge
    using PropertyGetterFn = rust::Fn<QVariant(uint8_t* receiver)>;
    using PropertySetterFn = rust::Fn<void(uint8_t* receiver, const QVariant& value)>;
    using SlotCallbackFn   = rust::Fn<void(uint8_t* receiver, const MetaMethodIncomingParams& params)>;

    DynamicMetaObjectData_Rust(const QMetaObject* staticMetaObj, rust::Str className);
    ~DynamicMetaObjectData_Rust();

    void setToQObject(QObject& dst) const;
    const QMetaObject* getDynamicQMetaObject() const;

    void addClassInfo(rust::Str name, rust::Str value);
    void registerProperty  (rust::Str name, const QMetaType& metaType, PropertyGetterFn getter, PropertySetterFn setter, rust::Str notifySignal);
    void registerPropertyReadOnly(rust::Str name, const QMetaType& metaType, PropertyGetterFn getter, bool isConstant, rust::Str notifySignal);
    void registerSignal  (rust::Str name, rust::Slice<const QMetaType> argMetaTypes);
    void registerSlot(rust::Str name, rust::Slice<const QMetaType> argMetaTypes, SlotCallbackFn callback);
    void endMetaRegistration();

    void emitSignal(QObject& obj, rust::Str name, const MetaMethodOutgoingParams& params) const;

private:
    std::unique_ptr<DynamicMetaObjectData_Cpp> m_impl;
};

DynamicMetaObjectData_Rust* createDynamicMetaObjectData(rust::Str rustStructName, const QMetaObject& staticMeta);

} // namespace rust::bridge

#endif // DYNAMICMETAOBJECTDATA_RUST_H
