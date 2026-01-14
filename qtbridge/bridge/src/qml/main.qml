// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

import QtQuick
import QtQuick.Controls
Text {
    text: "Hello Rust!"
    Component.onCompleted: closeTimer.start()
    Timer {
        id: closeTimer
        interval: 1
        onTriggered: Qt.quit()
    }
}
