// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

import QtQuick
import QtQuick.Controls
import QtTest

TestCase {
    name: "AbstractModelTest"
    id: test


    ListView {
        id: listview
        model: listmodel
        delegate: Text {
            required property int display
            text: display
        }
        width: 500
        height: 500
        visible: true
    }

    function test_base_functions() {
        compare(listmodel.rowCount(), 5)
        compare(listview.count, 5)

        compare(listview.itemAtIndex(0).display, 1)
        compare(listview.itemAtIndex(1).display, 2)
        compare(listview.itemAtIndex(2).display, 3)
        compare(listview.itemAtIndex(3).display, 10)
        compare(listview.itemAtIndex(4).display, 100)
    }
}
