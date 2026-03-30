// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QOBJECTPROXYCPP_RUST_BRIDGE__H_
#define _QOBJECTPROXYCPP_RUST_BRIDGE__H_
#include <QMetaObject>
#include <QObject>
#include <QQmlListProperty>
#include "qtbridge-runtime/src/cpp/rustobjectgetter.h"
#include "qtbridge-interfaces/src/qobject/proxy_rust_bridge.rs.h"

namespace rust::bridge {

class QObjectProxyCpp : public QObject, public RustObjectGetter
{
    using Base = QObject;

public:
    QObjectProxyCpp(uint8_t* rustObj, QObjectProxyRust* rustProxy);
    ~QObjectProxyCpp();

private:
    QObjectProxyRust* m_rustProxy;
};

// Functions for object construction
QObjectProxyCpp* create_QObjectProxyCpp(uint8_t* rustObj, QObjectProxyRust* rustProxy);
QObjectProxyCpp* create_QObjectProxyCpp_At(uint8_t* addr, uint8_t* rustObj, QObjectProxyRust* rustProxy);
const QMetaObject& staticQMetaObjectOf_QObjectProxyCpp();
size_t sizeOf_QObjectProxyCpp();
size_t alignOf_QObjectProxyCpp();
QMetaType qmetaTypeListOf_QObjectProxyCpp();

} // namespace rust::bridge

#endif // _QOBJECTPROXYCPP_RUST_BRIDGE__H_
