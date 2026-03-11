// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

import QtQuick
import QtQuick.Controls
import QtTest

TestCase {
    name: "AbstractModelTest"
    id: test

    TableView {
        id: tableview
        model: rustmodel
        reuseItems: false
        delegate: Text {
            required property int value
            required property string decoration
            required property string display
            text: display
            color: decoration
        }
        width: 500
        height: 500
        visible: true
    }

    function check_sync() {
        tableview.forceLayout()

        compare(tableview.rows, rustmodel.rowCount())
        compare(tableview.columns, rustmodel.columnCount())

        for (let i = 0; i < rustmodel.rowCount(); ++i) {
            for (let j = 0; j < rustmodel.columnCount(); ++j) {
                let index = rustmodel.index(i, j)
                compare(tableview.itemAtCell(Qt.point(i,j)).value, rustmodel.data(index, 2), "[" + i + "," +j + "].value")
                compare(tableview.itemAtCell(Qt.point(i,j)).decoration, rustmodel.data(index, 1), "[" + i + "," +j + "].decoration")
                compare(tableview.itemAtCell(Qt.point(i,j)).display, rustmodel.data(index, 0), "[" + i + "," +j + "].display")
            }
        }
    }

    function test_init() {
        rustmodel.resetItems()
        compare(rustmodel.rowCount(), 0)
        compare(rustmodel.columnCount(), 0)
        check_sync()
    }

    function test_add() {
        rustmodel.resetItems()
        rustmodel.addAColumn()
        rustmodel.addARow()
        rustmodel.addAColumn()
        rustmodel.addARow()
        rustmodel.addAColumn()
        compare(rustmodel.rowCount(), 2)
        compare(rustmodel.columnCount(), 3)
        check_sync()
    }

    function test_add_then_pop() {
        rustmodel.resetItems()
        rustmodel.addAColumn()
        rustmodel.addAColumn()
        rustmodel.addAColumn()
        rustmodel.addARow()
        rustmodel.addARow()
        rustmodel.addARow()
        compare(rustmodel.rowCount(), 3)
        compare(rustmodel.columnCount(), 3)
        check_sync()
        rustmodel.popARow()
        compare(rustmodel.rowCount(), 2)
        rustmodel.popARow()
        compare(rustmodel.rowCount(), 1)
        rustmodel.popARow()
        compare(rustmodel.rowCount(), 0)
        compare(rustmodel.columnCount(), 3)
        rustmodel.popAColumn()
        rustmodel.popAColumn()
        rustmodel.popAColumn()
        compare(rustmodel.rowCount(), 0)
        compare(rustmodel.columnCount(), 0)
        check_sync()
    }

    function test_insert_then_remove() {
        rustmodel.resetItems()
        rustmodel.insertAColumn()
        rustmodel.insertAColumn()
        rustmodel.insertAColumn()
        rustmodel.insertARow()
        rustmodel.insertARow()
        rustmodel.insertARow()
        compare(rustmodel.rowCount(), 3)
        compare(rustmodel.columnCount(), 3)
        check_sync()
        rustmodel.removeARow()
        compare(rustmodel.rowCount(), 2)
        rustmodel.removeARow()
        compare(rustmodel.rowCount(), 1)
        rustmodel.removeARow()
        compare(rustmodel.rowCount(), 0)
        compare(rustmodel.columnCount(), 3)
        rustmodel.removeAColumn()
        rustmodel.removeAColumn()
        rustmodel.removeAColumn()
        compare(rustmodel.rowCount(), 0)
        compare(rustmodel.columnCount(), 0)
        check_sync()
    }

    function test_insert_then_set_and_get() {
        rustmodel.resetItems()
        rustmodel.insertAColumn()
        rustmodel.insertAColumn()
        rustmodel.insertARow()
        rustmodel.insertARow()
        compare(rustmodel.rowCount(), 2)
        compare(rustmodel.columnCount(), 2)
        check_sync()
        rustmodel.changeItemAt(1,1)
        compare(rustmodel.data(rustmodel.index(1, 1), 0), "black")
        compare(rustmodel.data(rustmodel.index(1, 1), 1), "changed")
        compare(rustmodel.data(rustmodel.index(1, 1), 2), 0)
        check_sync()
    }
}
