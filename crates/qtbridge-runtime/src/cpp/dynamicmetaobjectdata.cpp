// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "dynamicmetaobjectdata.h"
#include "dispatchmetacallcpp.h"
#include "rustconv.h"
#include <QtLogging>

void DynamicMetaObjectData::addProperty(int index, const QByteArray& name, uint32_t userId, const QMetaType& metaType)
{
    auto [_, added] = m_properties.emplace(index, PropertyInfo{ userId, metaType, });
    if (!added)
        qFatal() << "Failed to register property " << name << ". Given index is already in use.";
}

void DynamicMetaObjectData::addSignal(int index, const QByteArray& name)
{
    auto [_, added] = m_signals.emplace(index, SignalInfo{ name });
    if (!added)
        qFatal() << "Failed to register signal " << name << ". Given index is already in use.";
}

void DynamicMetaObjectData::addSlot(int index, const QByteArray& name, uint32_t userId, Mutability mutability)
{
    auto [_, added] = m_slots.emplace(index, SlotInfo{ userId, mutability });
    if (!added)
        qFatal() << "Failed to register slot " << name << ". Given index is already in use.";
}

void DynamicMetaObjectData::emitSignal(QObject& obj, rust::Str name, rust::Slice<const uint8_t* const> argvSlice) const
{
    const QByteArray signalName = RustStrToQByteArray(name);
    if (auto index = getSignalIndex(signalName))
    {
        auto argv = reinterpret_cast<void**>(const_cast<uint8_t**>(argvSlice.data()));
        QMetaObject::activate(&obj, m_metaObject.get(), *index, argv);
    }
    else
        qFatal() << "Failed to find signal " << signalName << " by name";
}

std::optional<int> DynamicMetaObjectData::getSignalIndex(const QByteArray& name) const
{
    for (const auto& [index, signalInfo] : m_signals)
    {
        if (signalInfo.m_name == name)
            return index;
    }

    return std::nullopt;
}

void DynamicMetaObjectData::setToQObject(QObject& dst) const
{
    QObjectPrivate::get(&dst)->metaObject = const_cast<DynamicMetaObjectData*>(this);
}

void DynamicMetaObjectData::setMetaObject(std::unique_ptr<QMetaObject, QScopedPointerPodDeleter> metaObject)
{
    m_metaObject = std::move(metaObject);
}

bool DynamicMetaObjectData::isMetaObjectSet() const
{
    return static_cast<bool>(m_metaObject);
}

QMetaObject* DynamicMetaObjectData::getMetaObject() const
{
    return m_metaObject.get();
}

void DynamicMetaObjectData::objectDestroyed(QObject *)
{
    // Do nothing here unlike QDynamicMetaObjectData
    // to avoid double deletion
}

QMetaObject* DynamicMetaObjectData::toDynamicMetaObject(QObject* /*o*/)
{
    return getMetaObject();
}

int DynamicMetaObjectData::metaCall(QObject* o, QMetaObject::Call call, int id, void** argv)
{
    if (!m_metaObject)
        qFatal() << __func__ << "() called before setMetaObject()";

    auto dispatch = dynamic_cast<DispatchMetaCallCpp*>(o);
    if (!dispatch)
        qFatal("Failed to get pointer to QObject handling meta call");

    switch (call)
    {
        case QMetaObject::InvokeMetaMethod:
            if (handleMetaCallInvoke(o, *dispatch, id, argv))
                return -1;
        break;
        case QMetaObject::ReadProperty:
            if (handleMetaCallReadProperty(*dispatch, id, argv))
                return -1;
        break;
        case QMetaObject::WriteProperty:
            if (handleMetaCallWriteProperty(*dispatch, id, argv))
                return -1;
        break;
        default:
        break;
    }

    return o->qt_metacall(call, id, argv);
}

bool DynamicMetaObjectData::handleMetaCallInvoke(QObject* o, DispatchMetaCallCpp& dispatch, int id, void** argv)
{
    const int methodId = id - m_metaObject->methodOffset();
    if (methodId < 0 || methodId >= m_metaObject->methodCount())
        return false;

    QMetaMethod method = m_metaObject->method(id);
    switch (method.methodType())
    {
        case QMetaMethod::Signal:
        {
            if (!m_signals.count(methodId))
                return false;

            QMetaObject::activate(o, id, argv);
            return true;
        }
        break;
        case QMetaMethod::Slot:
        {
            auto slotIt = m_slots.find(methodId);
            if (slotIt == m_slots.end())
                return false;

            const int paramCount = method.parameterCount();
            const QMetaType returnType = method.returnMetaType();
            if ((paramCount > 0 || returnType.isValid()) && !argv)
                qFatal() << __func__ << "(): input meta params are null";

            uint8_t* const* u8Argv = reinterpret_cast<uint8_t* const*>(argv);
            const uint8_t* const* inputsBegin = u8Argv + 1;
            rust::Slice inputsSlice(inputsBegin, static_cast<size_t>(paramCount));
            auto outputSlice = returnType.isValid() ?
                rust::Slice<uint8_t* const>(u8Argv, 1) :
                rust::Slice<uint8_t* const>();
            const uint32_t userId = slotIt->second.m_userId;
            slotIt->second.m_mutability == Mutability::Mutable ?
                dispatch.invokeSlotMut(userId, inputsSlice, outputSlice) :
                dispatch.invokeSlot(userId, inputsSlice, outputSlice);
            return true;
        }
        break;
        default:
        break;
    }

    return false;
}

bool DynamicMetaObjectData::handleMetaCallReadProperty(const DispatchMetaCallCpp& dispatch, int id, void** argv)
{
    const int propId = id - m_metaObject->propertyOffset();
    if (propId < 0 || propId >= m_metaObject->propertyCount())
        return false;

    void* dstArg = argv[0];
    if (!dstArg)
        return false;

    auto propIt = m_properties.find(propId);
    if (propIt == m_properties.end())
        return false;

    const QMetaProperty property = m_metaObject->property(id);
    const QVariant result = dispatch.readProperty(propIt->second.m_userId);
    if (!QMetaType::convert(result.metaType(), result.data(), property.metaType(), dstArg))
        qFatal() << "Property type mismatch";

    return true;
}

bool DynamicMetaObjectData::handleMetaCallWriteProperty(DispatchMetaCallCpp& dispatch, int id, void** argv)
{
    const int propId = id - m_metaObject->propertyOffset();
    if (propId < 0 || propId >= m_metaObject->propertyCount())
        return false;

    void* arg = argv[0];
    if (!arg)
        return false;

    auto propIt = m_properties.find(propId);
    if (propIt == m_properties.end())
        return false;

    const QMetaProperty property = m_metaObject->property(id);
    const QVariant value = QVariant::fromMetaType(property.metaType(), arg);
    dispatch.writeProperty(propIt->second.m_userId, value);

    return true;
}
