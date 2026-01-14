// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

import QtQuick
import QtQuick.Controls

ApplicationWindow {
    width: 800
    height: 600
    visible: true

    required property var listmodel
    required property var listchanger

    ListView {
        id: listview
        anchors.fill: parent
        anchors.margins: 10

        model: listmodel

        delegate: Text {
            text: _0

        }
    }

    Row {
        anchors.bottom: parent.bottom
        Button {
            text: "Add 50"
            onClicked: listchanger.append()
        }
        Button {
            text: "Remove last"
            onClicked: listchanger.removeLast()
        }
        Button {
            text: "Change all"
            onClicked: listchanger.changeAll()
        }
    }

}
