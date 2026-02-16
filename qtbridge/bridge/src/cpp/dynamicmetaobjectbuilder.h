// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef DYNAMICMETAOBJECTBUILDER_H
#define DYNAMICMETAOBJECTBUILDER_H
#include <QMetaObject>
#include <cstdint>
#include <memory>
#include "rust/cxx.h"

class MetaMethodIncomingParams;
class MetaMethodOutgoingParams;
class QMetaType;
class QObject;
class QVariant;

/**
 * Runtime builder for QMetaObject.
 *
 * This class devoted to constructing a QMetaObject dynamically at runtime and binding
 * its properties accessors, and slots to Rust-provided callback functions.
 * It enables user-defined Rust objects to expose signals, slots and properties
 * through attached `QObject`s.
 *
 * The builder supports:
 *   - Declaring signals and emitting them.
 *   - Registering Qt properties with getters and setters implemented in Rust.
 *   - Registering slots that forward calls into Rust.
 *
  * This class is intended for internal use by the bridge layer.
 */
class DynamicMetaObjectBuilder
{
public:
    // Rust callbacks to be passed across the bridge
    using PropertyGetterFn = rust::Fn<QVariant(uint8_t* receiver)>;
    using PropertySetterFn = rust::Fn<void(uint8_t* receiver, const QVariant& value)>;
    using SlotCallbackFn   = rust::Fn<void(uint8_t* receiver, rust::Slice<const uint8_t* const> inputs, rust::Slice<uint8_t* const> output)>;

    DynamicMetaObjectBuilder(const QMetaObject* staticMetaObj, rust::Str className);

    void setToQObject(QObject& dst) const;
    const QMetaObject* getDynamicQMetaObject() const;

    void addClassInfo(rust::Str name, rust::Str value);
    void registerProperty(rust::Str name, const QMetaType& metaType, PropertyGetterFn getter, PropertySetterFn setter, rust::Str notifySignal);
    void registerPropertyReadOnly(rust::Str name, const QMetaType& metaType, PropertyGetterFn getter, bool isConstant, rust::Str notifySignal);
    void registerSignal(rust::Str name, rust::Slice<const QMetaType> argMetaTypes);
    void registerSlot(rust::Str name, rust::Slice<const QMetaType> argMetaTypes, const QMetaType& returnMetaType, SlotCallbackFn callback);
    void endMetaRegistration();

    void emitSignal(QObject& obj, rust::Str name, const MetaMethodOutgoingParams& params) const;

private:
    // Use pimpl idiom not to expose private APIs
    // and hide implementation details
    class Impl;
    std::unique_ptr<Impl> m_impl;
};

DynamicMetaObjectBuilder *createDynamicMetaObjectBuilder(rust::Str rustStructName, const QMetaObject& staticMeta);

#endif // #ifndef DYNAMICMETAOBJECTBUILDER_H
