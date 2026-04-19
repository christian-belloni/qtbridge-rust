// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "dynamicmetaobjectbuilder.h"
#include "dispatchmetacallcpp.h"
#include "dynamicmetaobjectdata.h"
#include "rustconv.h"
#include <QMetaType>
#include <QObject>
#include <QScopedPointer>
#include <QSpan>
#include <QtLogging>
#include <QVariant>
#include <private/qobject_p.h>
#include <private/qmetaobjectbuilder_p.h>
#include <map>
#include <optional>

class DynamicMetaObjectBuilder::Impl
{
public:
    Impl(const QMetaObject* staticMetaObj, const QByteArray& className)
        : m_mob(std::make_unique<QMetaObjectBuilder>())
        , m_data(std::make_unique<DynamicMetaObjectData>())
    {
        m_mob->setSuperClass(staticMetaObj); // TODO: check without this
        m_mob->setClassName(className.isEmpty() ? QByteArray(staticMetaObj->className()) : className);
    }

    void addClassInfo(const QByteArray& name, const QByteArray& value) {
        m_mob->addClassInfo(name, value);
    }

    // TODO: assume that
    //      notifySignal = name + "Changed"; ?
    void registerProperty(const QByteArray& name, uint32_t propId, const QMetaType& metaType, bool isConstant, const QByteArray& notifySignal)
    {
        std::optional<int> signal;
        if (!notifySignal.isEmpty())
        {
            signal = m_data->getSignalIndex(notifySignal);
            if (!signal)
                qFatal() << "Failed to find a signal by name: " << notifySignal;
        }

        doRegisterProperty(name, propId, metaType, isConstant, signal);
    }

    void registerSignal(const QByteArray& name, QSpan<const QMetaType> argMetaTypes)
    {
        if (!m_mob)
            qFatal() << "Signal registration must be done before endMetaRegistration() call";

        for (const QMetaType& type: argMetaTypes)
            type.registerType();

        QByteArray signature = generateFuncSignature(name, argMetaTypes);
        QMetaMethodBuilder builder = m_mob->addSignal(signature);
        const int index = builder.index();
        m_data->addSignal(index, name);
    }

    void registerSlot(const QByteArray& name, uint32_t slotId, QSpan<const QMetaType> argMetaTypes, const QMetaType& returnMetaType)
    {
        if (!m_mob)
            qFatal() << "Failed to register slot " << name << ". Slot registration must be done before endMetaRegistration() call";

        for (const QMetaType& type: argMetaTypes)
            type.registerType();

        QByteArray signature = generateFuncSignature(name, argMetaTypes);
        QMetaMethodBuilder builder = m_mob->addSlot(signature);
        if (returnMetaType.isValid())
            builder.setReturnType(returnMetaType.name());
        const int index = builder.index();
        m_data->addSlot(index, name, slotId);
    }

    void endMetaRegistration()
    {
        if (m_mob)
        {
            std::unique_ptr<QMetaObject, QScopedPointerPodDeleter> metaObject(m_mob->toMetaObject());
            m_data->setMetaObject(std::move(metaObject));
            m_mob.reset();
        }
        else
        {
            qFatal() << __func__ << "() is called more than once";
        }
    }

    void emitSignal(QObject& obj, const QByteArray& name, rust::Slice<const uint8_t* const> argvSlice)
    {
        if (auto index = m_data->getSignalIndex(name))
        {
            auto argv = reinterpret_cast<void**>(const_cast<uint8_t**>(argvSlice.data()));
            QMetaObject::activate(&obj, getDynamicQMetaObject(), *index, argv);
        }
        else
            qFatal() << "Failed to find signal " << name << " by name";
    }

    const QMetaObject* getDynamicQMetaObject()
    {
        return m_data->getMetaObject();
    }

    void setToQObject(QObject& dst) const
    {
        QObjectPrivate::get(&dst)->metaObject = m_data.get();
    }

private:
    void doRegisterProperty(const QByteArray& name, uint32_t propId, const QMetaType& metaType, bool isConstant, std::optional<int> signalIndex)
    {
        if (!m_mob)
            qFatal() << "Property registration must be done before endMetaRegistration() call";

        if (!metaType.isValid())
            qFatal() << "Invalid type of property " << name;

        const bool writable = !isConstant;
        metaType.registerType();

        QMetaPropertyBuilder builder = m_mob->addProperty(name, metaType.name());
        builder.setReadable(true);
        builder.setWritable(writable);
        builder.setConstant(isConstant);

        if (signalIndex)
            builder.setNotifySignal(m_mob->method(*signalIndex));

        const auto index = builder.index();
        m_data->addProperty(index, name, propId, metaType);
    }

    static QByteArray generateFuncSignature(const QByteArray& name, const QSpan<const QMetaType>& argMetaTypes)
    {
        QByteArray paramStr;
        for (const auto& type : argMetaTypes)
        {
            if (!type.isValid())
                qFatal() << "Unspecified argument type";

            if (!paramStr.isEmpty())
                paramStr.append(',');
            paramStr += type.name();
        }

        QByteArray sign = name + '(' + paramStr + ')';
        return QMetaObject::normalizedSignature(sign.constData());
    }

private:
    std::unique_ptr<QMetaObjectBuilder> m_mob;
    std::unique_ptr<DynamicMetaObjectData> m_data;
};


DynamicMetaObjectBuilder::DynamicMetaObjectBuilder(const QMetaObject* staticMetaObj, rust::Str className)
    : m_impl(std::make_unique<Impl>(staticMetaObj, RustStrToQByteArray(className)))
{}

void DynamicMetaObjectBuilder::setToQObject(QObject& dst) const
{
    m_impl->setToQObject(dst);
}

const QMetaObject* DynamicMetaObjectBuilder::getDynamicQMetaObject() const
{
    return m_impl->getDynamicQMetaObject();
}

void DynamicMetaObjectBuilder::addClassInfo(rust::Str name, rust::Str value)
{
    m_impl->addClassInfo(RustStrToQByteArray(name), RustStrToQByteArray(value));
}

void DynamicMetaObjectBuilder::registerProperty(rust::Str name, uint32_t propId, const QMetaType& metaType, bool isConstant, rust::Str notifySignal)
{
    m_impl->registerProperty(RustStrToQByteArray(name), propId, metaType, isConstant, RustStrToQByteArray(notifySignal));
}

void DynamicMetaObjectBuilder::registerSignal(rust::Str name, rust::Slice<const QMetaType> argMetaTypes)
{
    m_impl->registerSignal(RustStrToQByteArray(name), RustSliceToQSpan(argMetaTypes));
}

void DynamicMetaObjectBuilder::registerSlot(rust::Str name, uint32_t slotId, rust::Slice<const QMetaType> argMetaTypes, const QMetaType& returnMetaType)
{
    m_impl->registerSlot(RustStrToQByteArray(name), slotId, RustSliceToQSpan(argMetaTypes), returnMetaType);
}

void DynamicMetaObjectBuilder::endMetaRegistration()
{
    m_impl->endMetaRegistration();
}

void DynamicMetaObjectBuilder::emitSignal(QObject& obj, rust::Str name, rust::Slice<const uint8_t* const> argv) const
{
    m_impl->emitSignal(obj, RustStrToQByteArray(name), argv);
}

DynamicMetaObjectBuilder *createDynamicMetaObjectBuilder(rust::Str rustStructName, const QMetaObject& staticMeta)
{
    return new DynamicMetaObjectBuilder(&staticMeta, rustStructName);
}
