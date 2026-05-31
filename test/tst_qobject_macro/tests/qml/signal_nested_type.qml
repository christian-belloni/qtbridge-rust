// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

import QtQuick
import QtTest
import tst_qobject_macro

TestCase {
    property int intValue: 0
    property real floatValue: 0
    property string stringValue: ""

    Connections {
        target: Backend
        function onData_changed(data) {
            intValue = data.intValue
            floatValue = data.floatValue
            stringValue = data.stringValue
        }
    }

    function test_nested_type_signal() {
        Backend.emit_data_changed();
        compare(intValue, 42)
        compare(floatValue, 0.25)
        compare(stringValue, "Some string")
    }
}
