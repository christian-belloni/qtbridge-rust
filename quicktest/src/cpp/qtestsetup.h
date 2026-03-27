// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef QTESTSETUP_H
#define QTESTSETUP_H

#include <QtQuickTest>
#include <QQmlEngine>
#include <QQmlContext>
#include <QGuiApplication>

class QtQuickTestSetup : public QObject
{
    Q_OBJECT

public:
    QtQuickTestSetup(const QVariantMap &props) : properties(props) {}

public slots:
    void applicationAvailable()
    {

    }
    void qmlEngineAvailable(QQmlEngine *engine)
    {
        QQmlContext *context = engine->rootContext();

        for (auto it = properties.constBegin(); it != properties.constEnd(); ++it) {
            context->setContextProperty(it.key(), it.value());
        }
    }

    void cleanupTestCase()
    {

    }
private:
    QVariantMap properties;
};

#include "qtestsetup.moc"

#endif // #ifndef QTESTSETUP_H
