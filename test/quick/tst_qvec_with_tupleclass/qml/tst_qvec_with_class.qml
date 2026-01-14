// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

import QtQuick
import QtQuick.Controls
import QtTest

TestCase {
    name: "QVecWithTupleClassTest"
    id: test

    ListView {
        id: listview
        model: listmodel
        delegate: Text {
            required property string _0
            required property string _1
            required property int _2
            property string display: _0
            property string decoration: _1
            property int value: _2
            text: _0
            color: _1
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

        compare(listview.itemAtIndex(0).display, "10")
        compare(listview.itemAtIndex(1).display, "20")
        compare(listview.itemAtIndex(2).display, "30")

        compare(listview.itemAtIndex(0).value, 10)
        compare(listview.itemAtIndex(1).value, 20)
        compare(listview.itemAtIndex(2).value, 30)

        listchanger.append()

        compare(listmodel.rowCount(), 4)
        compare(listview.count, 4)

        listchanger.append()

        compare(listmodel.rowCount(), 5)
        compare(listview.count, 5)

        wait(300)

        compare(listview.itemAtIndex(3).display, "50")
        compare(listview.itemAtIndex(3).value, 50)
        compare(listview.itemAtIndex(4).display, "50")
        compare(listview.itemAtIndex(4).value, 50)

        listchanger.changeAll()


        compare(listview.itemAtIndex(0).display, "1010")
        compare(listview.itemAtIndex(1).display, "2020")
        compare(listview.itemAtIndex(2).display, "3030")
        compare(listview.itemAtIndex(3).display, "5050")
        compare(listview.itemAtIndex(4).display, "5050")

        compare(listview.itemAtIndex(0).value, 20)
        compare(listview.itemAtIndex(1).value, 40)
        compare(listview.itemAtIndex(2).value, 60)
        compare(listview.itemAtIndex(3).value, 100)
        compare(listview.itemAtIndex(4).value, 100)

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
