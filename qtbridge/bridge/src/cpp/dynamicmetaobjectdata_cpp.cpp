// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "dynamicmetaobjectdata_cpp.h"
#include "metamethodparams.h"
#include "rustobjectgetter.h"
#include <QMetaType>
#include <QObject>
#include <QScopedPointer>
#include <private/qobject_p.h>
#include <private/qmetaobjectbuilder_p.h>
#include <map>
#include <optional>
#include <stdexcept>

using namespace std::string_literals;

class DynamicMetaObjectData_Cpp::Impl : public QDynamicMetaObjectData
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

     void registerPropertyId(const QByteArray& name, const QMetaType& metaType, PropertyGetterFunc&& getter, PropertySetterFunc&& setter, bool isConstant, int notifySignalId)
    {
        std::optional<int> signal;
        if (notifySignalId >= 0)
        {
            signal = getSignalIndexByClientId(notifySignalId);
            if (!signal)
                throw std::runtime_error("Failed to find signal by client signal id");
        }

        doRegisterProperty(name, metaType, std::move(getter), std::move(setter), isConstant, signal);
    }

    // TODO: assume that
    //      notifySignal = name + "Changed"; ?
    void registerProperty(const QByteArray& name, const QMetaType& metaType, PropertyGetterFunc&& getter, PropertySetterFunc&& setter, bool isConstant, const QByteArray& notifySignal)
    {
        std::optional<int> signal;
        if (!notifySignal.isEmpty())
        {
            signal = getSignalIndexByName(notifySignal);
            if (!signal)
                throw std::runtime_error("Failed to find signal by name");
        }

        doRegisterProperty(name, metaType, std::move(getter), std::move(setter), isConstant, signal);
    }

    void registerSignal(const QByteArray& name, const std::vector<QMetaType>& argMetaTypes, std::optional<int> clientSignalid)
    {
        if (!m_mob)
            throw std::runtime_error("Signal registration must be done before endMetaRegistration() call");

        if (clientSignalid.has_value())
        {
            // check that signal with given Id is not yet registered
            if (std::any_of(m_signals.begin(), m_signals.end(),
                [clientId = *clientSignalid](const auto& entry) {
                    const auto& entrySignalId = entry.second.m_clientId;
                    return entrySignalId && *entrySignalId == clientId;
                }))
                    throw std::runtime_error("Signal for given Id is registered already");
        }

        for (const QMetaType& type: argMetaTypes)
            type.registerType();

        QByteArray signature = generateFuncSignature(name, argMetaTypes);
        QMetaMethodBuilder builder = m_mob->addSignal(signature);
        const int localId = builder.index();
        auto [_, added] = m_signals.emplace(localId, SignalInfo{ name, clientSignalid });
        if (!added)
            throw std::runtime_error("Failed to register signal");

#ifdef _DEBUG
        m_signalSignatures.emplace(localId, signature);
#endif // _DEBUG
    }

    void registerSlot(const QByteArray& name, const std::vector<QMetaType>& argMetaTypes, SlotFunc&& func)
    {
        if (!m_mob)
            throw std::runtime_error("Slot registration must be done before endMetaRegistration() call");

        for (const QMetaType& type: argMetaTypes)
            type.registerType();

        QByteArray signature = generateFuncSignature(name, argMetaTypes);
        QMetaMethodBuilder builder = m_mob->addSlot(signature);
        const int localId = builder.index();

        m_slots.emplace(localId, SlotInfo{ std::move(func) });

#ifdef _DEBUG
        m_slotSignatures.emplace(localId, signature);
#endif // _DEBUG
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
            Q_ASSERT_X(false, Q_FUNC_INFO, "Called more than once");
        }
    }

    void emitSignal(QObject* obj, const QByteArray& name, const MetaMethodOutgoingParams& params)
    {
        if (auto idx = getSignalIndexByName(name))
            doEmitSignal(obj, *idx, params);
        else
            throw std::runtime_error("Failed to find signal by name");
    }

    void emitSignalId(QObject* obj, int clientSignalId, const MetaMethodOutgoingParams& params)
    {
        if (auto idx = getSignalIndexByClientId(clientSignalId))
            doEmitSignal(obj, *idx, params);
        else
            throw std::runtime_error("Failed to find signal by signal id");
    }

    // static void connect(const Impl& sender, SignalId signalId, const Impl& receiver, SignalOrSlotId signalOrSlotId)
    // {
    //     const QMetaMethod senderMethod = sender.getMetaMethod(signalId);
    //     const QMetaMethod receiverMethod = receiver.getMetaMethod(signalOrSlotId);
    //
    //     TODO: re-implement this
    //     QObject::connect(&sender.m_qobject, senderMethod,
    //                     &receiver.m_qobject, receiverMethod);
    // }

    const QMetaObject* getDynamicQMetaObject()
    {
        if (!m_mo)
            endMetaRegistration();

        return m_mo.get();
    }

private:
    void doRegisterProperty(const QByteArray& name, const QMetaType& metaType, PropertyGetterFunc&& getter, PropertySetterFunc&& setter, bool isConstant, std::optional<int> signalIndex)
    {
        if (!m_mob)
            throw std::runtime_error("Property registration must be done before endMetaRegistration() call");

        if (!metaType.isValid())
            throw std::runtime_error("Invalid property type");

        const bool writable = static_cast<bool>(setter);
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
#ifdef _DEBUG
            //Q_ASSERT(m_signalSignatures.at(idx) == type); // TODO: extract argument type from signal signature
#endif // _DEBUG
            builder.setNotifySignal(m_mob->method(idx));
        }

        const auto localId = builder.index();
        auto [_, added] = m_properties.emplace(localId, PropertyInfo{ metaType, std::move(getter), std::move(setter) });
        if (!added)
            throw std::runtime_error("Failed to register property");

#ifdef _DEBUG
        m_propertyNames.emplace(localId, name);
#endif // _DEBUG
    }

    void doEmitSignal(QObject* obj, SignalId id, void** params)
    {
        if (!m_mo)
            throw std::logic_error(__func__ + " called before endMetaRegistration()"s);
        QMetaObject::activate(obj, m_mo.get(), id, params);
    }

     void doEmitSignal(QObject* obj, SignalId id, const MetaMethodOutgoingParams& params)
     {
         const QMetaMethod method = getMetaMethod(id);
         auto params_copy = params; // Do the copy to handle mutability of composed variant data
         std::vector<void*> paramData = params_copy.getDataPtrs(method);
         doEmitSignal(obj, id, paramData.data());
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

        auto rustPtrGetter = dynamic_cast<const RustObjectGetter*>(o);
        if (!rustPtrGetter)
            throw std::runtime_error("Failed to get pointer to rust object");
        uint8_t* rustPtr = rustPtrGetter->getRustObject();

        if (rustPtr)
        {
            switch (call)
            {
                case QMetaObject::InvokeMetaMethod:
                    if (handleMetaCallInvoke(o, rustPtr, id, argv))
                        return -1;
                break;
                case QMetaObject::ReadProperty:
                    if (handleMetaCallReadProperty(rustPtr, id, argv))
                        return -1;
                break;
                case QMetaObject::WriteProperty:
                    if (handleMetaCallWriteProperty(rustPtr, id, argv))
                        return -1;
                break;
                default:
                break;
            }
        }

        return o->qt_metacall(call, id, argv);
    }

    bool handleMetaCallInvoke(QObject* o, uint8_t* clientPtr, int id, void** argv)
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
#ifdef _DEBUG
                Q_ASSERT(method.methodSignature() == m_signalSignatures.at(methodId));
#endif // _DEBUG
                doEmitSignal(o, methodId, argv);
                return true;
            }
            break;
            case QMetaMethod::Slot:
            {
                auto slotIt = m_slots.find(methodId);
                if (slotIt == m_slots.end())
                    return false;
#ifdef _DEBUG
                Q_ASSERT(method.methodSignature() == m_slotSignatures.at(methodId));
#endif // _DEBUG
                if (auto& callback = slotIt->second.m_callback)
                {
                    const MetaMethodIncomingParams params(method, argv);
                    callback(clientPtr, params);
                    return true;
                }
            }
            break;
            default:
            break;
        }

        return false;
    }

    bool handleMetaCallReadProperty(uint8_t* clientPtr, int id, void** argv)
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

        auto& getterFunc = propIt->second.m_getter;
        if (!getterFunc)
            return false;

        const QMetaProperty property = m_mo->property(id);
#ifdef _DEBUG
        Q_ASSERT(property.name() == m_propertyNames.at(propId));
#endif // _DEBUG
        const QVariant result = getterFunc(clientPtr);
        if (!QMetaType::convert(result.metaType(), result.data(), property.metaType(), dstArg))
            throw std::logic_error("Property type mismatch");

        return true;
    }

    bool handleMetaCallWriteProperty(uint8_t* clientPtr, int id, void** argv)
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

        auto& setterFunc = propIt->second.m_setter;
        if (!setterFunc)
            return false;

        const QMetaProperty property = m_mo->property(id);
#ifdef _DEBUG
        Q_ASSERT(property.name() == m_propertyNames.at(propId));
#endif // _DEBUG
        const auto v = QVariant::fromMetaType(property.metaType(), arg);
        setterFunc(clientPtr, v);

        return true;
    }

    static QByteArray generateFuncSignature(const QByteArray& name, const std::vector<QMetaType>& argMetaTypes)
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

#ifdef _DEBUG
        Q_ASSERT(method.methodSignature() == m_signalSignatures.at(id));
#endif // _DEBUG

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

    std::optional<int> getSignalIndexByClientId(int clientId) const
    {
        for (const auto& [idx, signalInfo] : m_signals)
        {
            const auto& curClientId = signalInfo.m_clientId;
            if (curClientId && *curClientId == clientId)
                return idx;
        }

        return std::nullopt;
    }

private:
    struct PropertyInfo
    {
        QMetaType m_type;
        PropertyGetterFunc m_getter;
        PropertySetterFunc m_setter;
    };

    struct SignalInfo
    {
        QByteArray m_name;
        std::optional<int> m_clientId;
    };

    struct SlotInfo
    {
        SlotFunc m_callback;
    };

private:
    std::unique_ptr<QMetaObjectBuilder> m_mob;
    std::unique_ptr<QMetaObject, QScopedPointerPodDeleter> m_mo;
    std::map<PropertyId, PropertyInfo> m_properties;
    std::map<SignalId, SignalInfo> m_signals;
    std::map<SlotId, SlotInfo> m_slots;

#ifdef _DEBUG
    std::map<PropertyId, QByteArray> m_propertyNames;
    std::map<SignalId, QByteArray> m_signalSignatures;
    std::map<SlotId, QByteArray> m_slotSignatures;
#endif // _DEBUG
};


DynamicMetaObjectData_Cpp::DynamicMetaObjectData_Cpp(const QMetaObject* staticMetaObj, const QByteArray& className)
    : m_impl(std::make_unique<Impl>(staticMetaObj, className))
{}

DynamicMetaObjectData_Cpp::~DynamicMetaObjectData_Cpp()
{}

void DynamicMetaObjectData_Cpp::setToQObject(QObject& dst) const
{
    QObjectPrivate::get(&dst)->metaObject = m_impl.get();
}

const QMetaObject* DynamicMetaObjectData_Cpp::getDynamicQMetaObject() const
{
    return m_impl->getDynamicQMetaObject();
}

void DynamicMetaObjectData_Cpp::addClassInfo(const QByteArray& name, const QByteArray& value) {
    m_impl->addClassInfo(name, value);
}

void DynamicMetaObjectData_Cpp::registerPropertyId(const QByteArray& name, const QMetaType& metaType, PropertyGetterFunc&& getter, PropertySetterFunc&& setter, bool isConstant, ClientSignalId notifySignal)
{
    m_impl->registerPropertyId(name, metaType, std::move(getter), std::move(setter), isConstant, notifySignal);
}

void DynamicMetaObjectData_Cpp::registerProperty(const QByteArray& name, const QMetaType& metaType, PropertyGetterFunc&& getter, PropertySetterFunc&& setter, bool isConstant, const QByteArray& notifySignal)
{
    m_impl->registerProperty(name, metaType, std::move(getter), std::move(setter), isConstant, notifySignal);
}

void DynamicMetaObjectData_Cpp::registerSignal(const QByteArray& name, const std::vector<QMetaType>& argMetaTypes, std::optional<ClientSignalId> clientSignalId)
{
    m_impl->registerSignal(name, argMetaTypes, clientSignalId);
}

void DynamicMetaObjectData_Cpp::registerSlot(const QByteArray& name, const std::vector<QMetaType>& argMetaTypes, SlotFunc&& callback)
{
    m_impl->registerSlot(name, argMetaTypes, std::move(callback));
}

void DynamicMetaObjectData_Cpp::endMetaRegistration()
{
    m_impl->endMetaRegistration();
}

void DynamicMetaObjectData_Cpp::emitSignal(QObject* obj, const QByteArray& name, const MetaMethodOutgoingParams& params) const
{
    m_impl->emitSignal(obj, name, params);
}

void DynamicMetaObjectData_Cpp::emitSignal(QObject* obj, ClientSignalId clientSignalId, const MetaMethodOutgoingParams& params) const
{
    m_impl->emitSignalId(obj, clientSignalId, params);
}
