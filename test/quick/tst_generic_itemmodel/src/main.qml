// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

ApplicationWindow {
    width: 800
    height: 600
    visible: true

    required property var rustmodel
    required property var rustmodel2

    RowLayout {
        anchors.fill: parent
        anchors.margins: 10
        TreeView {
            id: treeView
            clip: true

            Layout.fillWidth: true
            Layout.fillHeight: true

            selectionModel: ItemSelectionModel {}

            // The model needs to be a QAbstractItemModel
            model: rustmodel

            delegate: TreeViewDelegate {}
        }

        TreeView {
            id: treeView2
            clip: true

            Layout.fillWidth: true
            Layout.fillHeight: true

            selectionModel: ItemSelectionModel {}

            // The model needs to be a QAbstractItemModel
            model: rustmodel2

            delegate: TreeViewDelegate {}
        }
    }
}
