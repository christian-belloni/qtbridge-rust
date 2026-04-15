// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "dynamicmetaobjectbuilder.h"
#include "dispatchmetacallcpp.h"
#include "rustconv.h"
#include <QMetaType>
#include <QObject>
#include <QScopedPointer>
#include <QSpan>
#include <QVariant>
#include <private/qobject_p.h>
#include <private/qmetaobjectbuilder_p.h>
#include <cassert>
#include <map>
#include <optional>
#include <stdexcept>

using namespace std::string_literals;

class DynamicMetaObjectBuilder::Impl : public QDynamicMetaObjectData
{
public:
    using PropertyId = int;
    using SignalId = int;
    using SlotId = int;

    Impl(const QMetaObject* staticMetaObj, const QByteArray& className)
        : m_mob(std::make_unique<QMetaObjectBuilder>())
    {
        m_mob->setSuperClass(staticMetaObj); // TODO: check without this
        m_mob->setClassName(className.isEmpty() ? QByteArray(staticMetaObj->className()) : className);
    }

    void addClassInfo(const QByteArray& name, const QByteArray& value) {
        m_mob->addClassInfo(name, value);
    }

    // TODO: assume that
    //      notifySignal = name + "Changed"; ?
    void registerProperty(const QByteArray& name, uint32_t propId, const QMetaType& metaType, PropertyGetterFn getter, std::optional<PropertySetterFn> setter, bool isConstant, const QByteArray& notifySignal)
    {
        std::optional<int> signal;
        if (!notifySignal.isEmpty())
        {
            signal = getSignalIndexByName(notifySignal);
            if (!signal)
                throw std::runtime_error("Failed to find signal by name");
        }

        doRegisterProperty(name, propId, metaType, std::move(getter), std::move(setter), isConstant, signal);
    }

    void registerSignal(const QByteArray& name, QSpan<const QMetaType> argMetaTypes)
    {
        if (!m_mob)
            throw std::runtime_error("Signal registration must be done before endMetaRegistration() call");

        for (const QMetaType& type: argMetaTypes)
            type.registerType();

        QByteArray signature = generateFuncSignature(name, argMetaTypes);
        QMetaMethodBuilder builder = m_mob->addSignal(signature);
        const int localId = builder.index();
        auto [_, added] = m_signals.emplace(localId, SignalInfo{ name });
        if (!added)
            throw std::runtime_error("Failed to register signal");
    }

    void registerSlot(const QByteArray& name, uint32_t slotId, QSpan<const QMetaType> argMetaTypes, const QMetaType& returnMetaType, SlotCallbackFn&& func)
    {
        if (!m_mob)
            throw std::runtime_error("Slot registration must be done before endMetaRegistration() call");

        for (const QMetaType& type: argMetaTypes)
            type.registerType();

        QByteArray signature = generateFuncSignature(name, argMetaTypes);
        QMetaMethodBuilder builder = m_mob->addSlot(signature);
        if (returnMetaType.isValid())
            builder.setReturnType(returnMetaType.name());
        const int localId = builder.index();

        m_slots.emplace(localId, SlotInfo{ slotId, std::move(func) });
    }

    void endMetaRegistration()
    {
        if (m_mob)
        {
            m_mo.reset(m_mob->toMetaObject());
            m_mob.reset();
        }
        else
        {
            assert(false && "The function is called more than once");
        }
    }

    void emitSignal(QObject& obj, const QByteArray& name, rust::Slice<const uint8_t* const> argv)
    {
        if (auto idx = getSignalIndexByName(name))
            doEmitSignal(obj, *idx, argv);
        else
            throw std::runtime_error("Failed to find signal by name");
    }

    const QMetaObject* getDynamicQMetaObject()
    {
        if (!m_mo)
            endMetaRegistration();

        return m_mo.get();
    }

private:
    void doRegisterProperty(const QByteArray& name, uint32_t propId, const QMetaType& metaType, PropertyGetterFn getter, std::optional<PropertySetterFn> setter, bool isConstant, std::optional<int> signalIndex)
    {
        if (!m_mob)
            throw std::runtime_error("Property registration must be done before endMetaRegistration() call");

        if (!metaType.isValid())
            throw std::runtime_error("Invalid property type");

        const bool writable = setter.has_value();
        metaType.registerType();

        //TODO: pass notifierId argument to addProperty?
        QMetaPropertyBuilder builder = m_mob->addProperty(name, metaType.name());
        builder.setReadable(true);
        builder.setWritable(writable);
        builder.setConstant(!writable && isConstant);

        if (signalIndex)
        {
            const int idx = *signalIndex;
            if (!m_signals.count(idx))
                throw std::runtime_error("Unknown property change signal");
            builder.setNotifySignal(m_mob->method(idx));
        }

        const auto localId = builder.index();
        auto [_, added] = m_properties.emplace(localId, PropertyInfo{ propId, metaType, std::move(getter), std::move(*setter) });
        if (!added)
            throw std::runtime_error("Failed to register property");
    }

    void doEmitSignal(QObject& obj, SignalId id, void** params)
    {
        if (!m_mo)
            throw std::logic_error(__func__ + " called before endMetaRegistration()"s);
        QMetaObject::activate(&obj, m_mo.get(), id, params);
    }

     void doEmitSignal(QObject& obj, SignalId id, rust::Slice<const uint8_t* const> argv)
     {
        doEmitSignal(obj, id, reinterpret_cast<void**>(const_cast<uint8_t**>(argv.data())));
    }

    void objectDestroyed(QObject *) override
    {
        // Do nothing here unlike QDynamicMetaObjectData
        // to avoid double deletion
    }

    QMetaObject* toDynamicMetaObject(QObject* /*o*/) override
    {
        if (!m_mo)
            endMetaRegistration();

        return m_mo.get();
    }

    int metaCall(QObject* o, QMetaObject::Call call, int id, void** argv) override
    {
        if (!m_mo)
            throw std::logic_error(__func__ + " called before endMetaRegistration()"s);

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

    bool handleMetaCallInvoke(QObject* o, DispatchMetaCallCpp& dispatch, int id, void** argv)
    {
        const int methodId = id - m_mo->methodOffset();
        if (methodId < 0 || methodId >= m_mo->methodCount())
            return false;

        QMetaMethod method = m_mo->method(id);
        switch (method.methodType())
        {
            case QMetaMethod::Signal:
            {
                if (!m_signals.count(methodId))
                    return false;
                doEmitSignal(*o, methodId, argv);
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
                    throw std::runtime_error("Input meta params are null");

                uint8_t* const* u8Argv = reinterpret_cast<uint8_t* const*>(argv);
                const uint8_t* const* inputsBegin = u8Argv + 1;
                rust::Slice inputsSlice(inputsBegin, static_cast<size_t>(paramCount));
                auto outputSlice = returnType.isValid() ?
                    rust::Slice<uint8_t* const>(u8Argv, 1) :
                    rust::Slice<uint8_t* const>();
                dispatch.invokeSlot(slotIt->second.m_userId, inputsSlice, outputSlice);
                return true;
            }
            break;
            default:
            break;
        }

        return false;
    }

    bool handleMetaCallReadProperty(const DispatchMetaCallCpp& dispatch, int id, void** argv)
    {
        const int propId = id - m_mo->propertyOffset();
        if (propId < 0 || propId >= m_mo->propertyCount())
            return false;

        void* dstArg = argv[0];
        if (!dstArg)
            return false;

        auto propIt = m_properties.find(propId);
        if (propIt == m_properties.end())
            return false;

        const QMetaProperty property = m_mo->property(id);
        const QVariant result = dispatch.readProperty(propIt->second.m_userId);
        if (!QMetaType::convert(result.metaType(), result.data(), property.metaType(), dstArg))
            throw std::logic_error("Property type mismatch");

        return true;
    }

    bool handleMetaCallWriteProperty(DispatchMetaCallCpp& dispatch, int id, void** argv)
    {
        const int propId = id - m_mo->propertyOffset();
        if (propId < 0 || propId >= m_mo->propertyCount())
            return false;

        void* arg = argv[0];
        if (!arg)
            return false;

        auto propIt = m_properties.find(propId);
        if (propIt == m_properties.end())
            return false;

        const QMetaProperty property = m_mo->property(id);
        const QVariant value = QVariant::fromMetaType(property.metaType(), arg);
        dispatch.writeProperty(propIt->second.m_userId, value);

        return true;
    }

    static QByteArray generateFuncSignature(const QByteArray& name, const QSpan<const QMetaType>& argMetaTypes)
    {
        QString paramStr;
        for (const auto& type : argMetaTypes)
        {
            if (!type.isValid())
                throw std::runtime_error("Unspecified argument type");

            if (!paramStr.isEmpty())
                paramStr.append(',');
            paramStr += type.name();
        }

        QString sign = name + '(' + paramStr + ')';
        return QMetaObject::normalizedSignature(sign.toStdString().c_str());
    }

    QMetaMethod getMetaMethod(int id) const
    {
        if (!m_mo)
            throw std::logic_error(__func__ + " called before endMetaRegistration()"s);

        const int methodOffset = m_mo->methodOffset();
        const QMetaMethod method = m_mo->method(id + methodOffset);
        return method;
    }

    std::optional<int> getSignalIndexByName(const QByteArray& name) const
    {
        for (const auto& [idx, signalInfo] : m_signals)
        {
            if (signalInfo.m_name == name)
                return idx;
        }
        return std::nullopt;
    }

private:
    struct PropertyInfo
    {
        uint32_t m_userId;
        QMetaType m_type;
        PropertyGetterFn m_getter;
        PropertySetterFn m_setter;
    };

    struct SignalInfo
    {
        QByteArray m_name;
    };

    struct SlotInfo
    {
        uint32_t m_userId;
        SlotCallbackFn m_callback;
    };

private:
    std::unique_ptr<QMetaObjectBuilder> m_mob;
    std::unique_ptr<QMetaObject, QScopedPointerPodDeleter> m_mo;
    std::map<PropertyId, PropertyInfo> m_properties;
    std::map<SignalId, SignalInfo> m_signals;
    std::map<SlotId, SlotInfo> m_slots;
};


DynamicMetaObjectBuilder::DynamicMetaObjectBuilder(const QMetaObject* staticMetaObj, rust::Str className)
    : m_impl(std::make_unique<Impl>(staticMetaObj, RustStrToQByteArray(className)))
{}

void DynamicMetaObjectBuilder::setToQObject(QObject& dst) const
{
    QObjectPrivate::get(&dst)->metaObject = m_impl.get();
}

const QMetaObject* DynamicMetaObjectBuilder::getDynamicQMetaObject() const
{
    return m_impl->getDynamicQMetaObject();
}

void DynamicMetaObjectBuilder::addClassInfo(rust::Str name, rust::Str value)
{
    m_impl->addClassInfo(RustStrToQByteArray(name), RustStrToQByteArray(value));
}

void DynamicMetaObjectBuilder::registerProperty(rust::Str name, uint32_t propId, const QMetaType& metaType, PropertyGetterFn getter, PropertySetterFn setter, rust::Str notifySignal)
{
    m_impl->registerProperty(RustStrToQByteArray(name), propId, metaType, std::move(getter), std::move(setter), false, RustStrToQByteArray(notifySignal));
}

void DynamicMetaObjectBuilder::registerPropertyReadOnly(rust::Str name, uint32_t propId, const QMetaType& metaType, PropertyGetterFn getter, bool isConstant, rust::Str notifySignal)
{
    m_impl->registerProperty(RustStrToQByteArray(name), propId, metaType, std::move(getter), std::nullopt, isConstant, RustStrToQByteArray(notifySignal));
}

void DynamicMetaObjectBuilder::registerSignal(rust::Str name, rust::Slice<const QMetaType> argMetaTypes)
{
    m_impl->registerSignal(RustStrToQByteArray(name), RustSliceToQSpan(argMetaTypes));
}

void DynamicMetaObjectBuilder::registerSlot(rust::Str name, uint32_t slotId, rust::Slice<const QMetaType> argMetaTypes, const QMetaType& returnMetaType, SlotCallbackFn callback)
{
    m_impl->registerSlot(RustStrToQByteArray(name), slotId, RustSliceToQSpan(argMetaTypes), returnMetaType, std::move(callback));
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
