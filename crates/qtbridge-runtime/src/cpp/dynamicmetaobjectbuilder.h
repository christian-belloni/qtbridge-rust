// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef DYNAMICMETAOBJECTBUILDER_H
#define DYNAMICMETAOBJECTBUILDER_H
#include <QMetaObject>
#include <cstdint>
#include <memory>
#include "rust/cxx.h"

class DynamicMetaObjectData;
class QMetaType;

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
    DynamicMetaObjectBuilder(const QMetaObject* staticMetaObj, rust::Str className);
    ~DynamicMetaObjectBuilder();


    void addClassInfo(rust::Str name, rust::Str value);
    void registerProperty(rust::Str name, uint32_t propId, const QMetaType& metaType, bool isConstant, rust::Str notifySignal);
    void registerSignal(rust::Str name, rust::Slice<const QMetaType> argMetaTypes);
    void registerSlot(rust::Str name, uint32_t slotId, rust::Slice<const QMetaType> argMetaTypes, const QMetaType& returnMetaType);
    void endMetaRegistration();

    const DynamicMetaObjectData* takeDynamicMetaObjectData();

private:
    // Use pimpl idiom not to expose private APIs
    // and hide implementation details
    class Impl;
    std::unique_ptr<Impl> m_impl;
};

std::unique_ptr<DynamicMetaObjectBuilder> createDynamicMetaObjectBuilder(rust::Str rustStructName, const QMetaObject& staticMeta);

#endif // #ifndef DYNAMICMETAOBJECTBUILDER_H
