// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

import QtQuick
import QtQuick.Controls
import qtbridge

ApplicationWindow {
   visible: true
    title: qsTr("Minimal QML app")

    Button {
        anchors.centerIn: parent
        text: "Hello World!"
        onClicked: Backend.sayHello()
    }

    Component.onCompleted: closeTimer.start()
    Timer {
        id: closeTimer
        interval: 1
        onTriggered: Qt.quit()
    }
}
