// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "dynamicmetaobjectdata_rust.h"
#include "dynamicmetaobjectdata_cpp.h"
#include "metamethodparams.h"
#include "rustconv.h"

namespace rust::bridge
{

DynamicMetaObjectData_Rust::DynamicMetaObjectData_Rust(const QMetaObject* staticMetaObj, rust::Str className)
    : m_impl(std::make_unique<DynamicMetaObjectData_Cpp>(staticMetaObj, RustStrToQByteArray(className)))
{}

DynamicMetaObjectData_Rust::~DynamicMetaObjectData_Rust()
{}

void DynamicMetaObjectData_Rust::setToQObject(QObject& dst) const
{
    m_impl->setToQObject(dst);
}

const QMetaObject* DynamicMetaObjectData_Rust::getDynamicQMetaObject() const
{
    return m_impl->getDynamicQMetaObject();
}

void DynamicMetaObjectData_Rust::addClassInfo(rust::Str name, rust::Str value)
{
    m_impl->addClassInfo(RustStrToQByteArray(name), RustStrToQByteArray(value));
}

void DynamicMetaObjectData_Rust::registerPropertyId(rust::Str name, const QMetaType& metaType, PropertyGetterFn getter, bool isConstant, int32_t notifySignal)
{
    m_impl->registerPropertyId(RustStrToQByteArray(name), metaType, std::move(getter), nullptr, isConstant, notifySignal);
}

void DynamicMetaObjectData_Rust::registerProperty(rust::Str name, const QMetaType& metaType, PropertyGetterFn getter, PropertySetterFn setter, rust::Str notifySignal)
{
    m_impl->registerProperty(RustStrToQByteArray(name), metaType, std::move(getter), std::move(setter), false, RustStrToQByteArray(notifySignal));
}

void DynamicMetaObjectData_Rust::registerPropertyReadOnly(rust::Str name, const QMetaType& metaType, PropertyGetterFn getter, bool isConstant, rust::Str notifySignal)
{
    m_impl->registerProperty(RustStrToQByteArray(name), metaType, std::move(getter), nullptr, isConstant, RustStrToQByteArray(notifySignal));
}

void DynamicMetaObjectData_Rust::registerSignalId(rust::Str name, rust::Slice<const QMetaType> argMetaTypes, int32_t signalId)
{
    m_impl->registerSignal(RustStrToQByteArray(name), RustContainerToCppVector(argMetaTypes), signalId);
}

void DynamicMetaObjectData_Rust::registerSignal(rust::Str name, rust::Slice<const QMetaType> argMetaTypes)
{
    m_impl->registerSignal(RustStrToQByteArray(name), RustContainerToCppVector(argMetaTypes));
}

void DynamicMetaObjectData_Rust::registerSlot(rust::Str name, rust::Slice<const QMetaType> argMetaTypes, SlotCallbackFn callback)
{
    m_impl->registerSlot(RustStrToQByteArray(name), RustContainerToCppVector(argMetaTypes), std::move(callback));
}

void DynamicMetaObjectData_Rust::endMetaRegistration()
{
    m_impl->endMetaRegistration();
}

void DynamicMetaObjectData_Rust::emitSignal(QObject& obj, rust::Str name, const MetaMethodOutgoingParams& params) const
{
    m_impl->emitSignal(&obj, RustStrToQByteArray(name), params);
}

void DynamicMetaObjectData_Rust::emitSignal(QObject& obj, ClientSignalId clientSignalId, const MetaMethodOutgoingParams& params) const
{
    m_impl->emitSignal(&obj, clientSignalId, params);
}

DynamicMetaObjectData_Rust* createDynamicMetaObjectData(rust::Str rustStructName, const QMetaObject& staticMeta)
{
    const QByteArray structName = RustStrToQByteArray(rustStructName);
    return new DynamicMetaObjectData_Rust(&staticMeta, rustStructName);
}

} // namespace rust::bridge
