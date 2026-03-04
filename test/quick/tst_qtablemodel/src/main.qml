// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

ApplicationWindow {

    visible: true
    width: 500
    height: 500
    required property var rustmodel

    ColumnLayout {
        anchors.fill: parent
        TableView {
            id: tableview
            model: rustmodel
            delegate: Text {
                required property int value
                required property string decoration
                required property string display
                text: display
                color: decoration
            }
            width: 500
            height: 400

        }
        RowLayout {
            Button {
                text: "append column"
                onClicked: rustmodel.addAColumn()
            }
            Button {
                text: "append row"
                onClicked: rustmodel.addARow()
            }
        }
    }
}
