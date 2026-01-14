// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

import QtQuick
import QtQuick.Controls
import QtTest

TestCase {
    name: "QVecTest"
    id: test

    ListView {
        id: listview
        model: listmodel
        delegate: Text {
            required property string _0
            required property int _1
            text: _0 + ": " + _1
        }
        width: 500
        height: 500
        visible: true
    }

    function test_statics() {
        compare(listmodel.columnCount(), 1)
    }

    function test_basic() {
        compare(listmodel.rowCount(), 3)
        compare(listview.count, 3)

        compare(listview.itemAtIndex(0)._0, "10")
        compare(listview.itemAtIndex(1)._0, "20")
        compare(listview.itemAtIndex(2)._0, "30")

        compare(listview.itemAtIndex(0)._1, 10)
        compare(listview.itemAtIndex(1)._1, 20)
        compare(listview.itemAtIndex(2)._1, 30)

        listchanger.append()

        compare(listmodel.rowCount(), 4)
        compare(listview.count, 4)

        listchanger.append()

        compare(listmodel.rowCount(), 5)
        compare(listview.count, 5)

        wait(300)

        compare(listview.itemAtIndex(3)._0, "50")
        compare(listview.itemAtIndex(3)._1, 50)
        compare(listview.itemAtIndex(4)._0, "50")
        compare(listview.itemAtIndex(4)._1, 50)

        listchanger.changeAll()

        compare(listview.itemAtIndex(0)._0, "60")
        compare(listview.itemAtIndex(0)._1, 60)
        compare(listview.itemAtIndex(2)._0, "60")
        compare(listview.itemAtIndex(2)._1, 60)

        listchanger.removeLast()

        compare(listmodel.rowCount(), 4)
        compare(listview.count, 4)

        listchanger.removeLast()
        listchanger.removeLast()
        listchanger.removeLast()
        listchanger.removeLast()

        compare(listmodel.rowCount(), 0)
        compare(listview.count, 0)

        listchanger.removeLast()

        compare(listmodel.rowCount(), 0)
        compare(listview.count, 0)
    }
}
