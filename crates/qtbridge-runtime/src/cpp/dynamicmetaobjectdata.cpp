// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "dynamicmetaobjectdata.h"
#include "dispatchmetacallcpp.h"
#include "rustconv.h"
#include <QtLogging>

void DynamicMetaObjectData::addProperty(uint32_t userId, const QMetaType& metaType)
{
    m_properties.push_back(PropertyInfo{ userId, metaType });
}

void DynamicMetaObjectData::addSignal(const QByteArray& name)
{
    m_signals.push_back(SignalInfo{ name });
}

void DynamicMetaObjectData::addSlot(uint32_t userId, Mutability mutability)
{
    m_slots.push_back(SlotInfo{ userId, mutability });
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
    for (qsizetype index = 0; index < m_signals.size(); ++index)
    {
        if (m_signals[index].m_name == name)
            return static_cast<int>(index);
    }

    return std::nullopt;
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
    // Handle signals and slots. Currently we don't support any other type of meta methods.
    const int methodId = id - m_metaObject->methodOffset();
    if (methodId < 0 || methodId >= m_metaObject->methodCount())
        return false; // Must be handled by a base or a derived class.

    if (methodId < m_signals.size())
    {
        // This is a signal. Signals are added first to QMetaObject.
        QMetaObject::activate(o, id, argv);
        return true;
    }

    // This is a slot. Slots are added after signals.
    const int slotId = methodId - m_signals.size();
    if (slotId >= m_slots.size())
        return false;
    const SlotInfo& slotInfo = m_slots[slotId];

    QMetaMethod method = m_metaObject->method(id);
    const int paramCount = method.parameterCount();
    const QMetaType returnType = method.returnMetaType();
    if ((paramCount > 0 || returnType.isValid()) && !argv)
        qFatal() << __func__ << "(): input meta params are null";

    // Prepare spans with the callback's input and output data.
    uint8_t* const* u8Argv = reinterpret_cast<uint8_t* const*>(argv);
    const uint8_t* const* inputsBegin = u8Argv + 1;
    rust::Slice inputsSlice(inputsBegin, static_cast<size_t>(paramCount));
    auto outputSlice = returnType.isValid() ?
        rust::Slice<uint8_t* const>(u8Argv, 1) :
        rust::Slice<uint8_t* const>();

    // Invoke the slot callback.
    const uint32_t userId = slotInfo.m_userId;
    slotInfo.m_mutability == Mutability::Mutable ?
        dispatch.invokeSlotMut(userId, inputsSlice, outputSlice) :
        dispatch.invokeSlot(userId, inputsSlice, outputSlice);
    return true;
}

bool DynamicMetaObjectData::handleMetaCallReadProperty(const DispatchMetaCallCpp& dispatch, int id, void** argv)
{
    const int propId = id - m_metaObject->propertyOffset();
    if (propId < 0 || propId >= m_metaObject->propertyCount())
        return false; // Must be handled by a base or a derived class.
    if (propId >= m_properties.size())
        return false;
    const PropertyInfo& propInfo = m_properties[propId];

    void* dstArg = argv[0];
    if (!dstArg)
        return false;

    const QMetaProperty property = m_metaObject->property(id);
    const QVariant result = dispatch.readProperty(propInfo.m_userId);
    if (!QMetaType::convert(result.metaType(), result.data(), property.metaType(), dstArg))
        qFatal() << "Property type mismatch";

    return true;
}

bool DynamicMetaObjectData::handleMetaCallWriteProperty(DispatchMetaCallCpp& dispatch, int id, void** argv)
{
    const int propId = id - m_metaObject->propertyOffset();
    if (propId < 0 || propId >= m_metaObject->propertyCount())
        return false; // Must be handled by a base or a derived class.
    if (propId >= m_properties.size())
        return false;
    const PropertyInfo& propInfo = m_properties[propId];

    void* arg = argv[0];
    if (!arg)
        return false;

    const QMetaProperty property = m_metaObject->property(id);
    const QVariant value = QVariant::fromMetaType(property.metaType(), arg);
    dispatch.writeProperty(propInfo.m_userId, value);

    return true;
}
