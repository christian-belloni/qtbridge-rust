// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

import QtQuick
import QtQuick.Controls

ApplicationWindow {
    width: 400
    height: 200
    visible: true

    Image {
        source: "qrc:/data/rustacean-orig-noshadow.png"
        anchors.centerIn: parent
        anchors.horizontalCenterOffset: -100
        width: 100
        height: width / implicitWidth * implicitHeight
    }

    Image {
        source: "qrc:/data/rustacean-flat-happy.png"
        anchors.centerIn: parent
        width: 100
        height: width / implicitWidth * implicitHeight
    }

    Image {
        source: "qrc:/data2/rustacean-orig-noshadow.png"
        anchors.centerIn: parent
        anchors.horizontalCenterOffset: 100
        width: 100
        height: width / implicitWidth * implicitHeight
    }

    TextEdit {
        textDocument.source: "qrc:/data/text/example.txt"
        anchors.centerIn: parent
        anchors.verticalCenterOffset: 50
    }

    TextEdit {
        textDocument.source: "qrc:/text/data/example.txt"
        anchors.centerIn: parent
        anchors.verticalCenterOffset: 70
    }
}
