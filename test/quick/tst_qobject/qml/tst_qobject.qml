// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

import QtQuick 2.3
import QtTest 1.0

TestCase {
    name: "SignalSlotTests"
    id: test

    SignalSpy {
        id: spy_triggered
        target: testObject
        signalName: "triggered"
    }

    SignalSpy {
        id: spy_string
        target: testObject
        signalName: "stringSignal"
    }

    SignalSpy {
        id: spy_intProp64
        target: testObject
        signalName: "intProp64Changed"
    }

    SignalSpy {
        id: spy_intProp32
        target: testObject
        signalName: "intProp32Changed"
    }

    SignalSpy {
        id: spy_intProp8
        target: testObject
        signalName: "intProp8Changed"
    }

    SignalSpy {
        id: spy_intGetterProp
        target: testObject
        signalName: "intGetterPropChanged"
    }

    SignalSpy {
        id: spy_boolProp
        target: testObject
        signalName: "boolPropChanged"
    }

    SignalSpy {
        id: spy_floatProp
        target: testObject
        signalName: "floatPropChanged"
    }

    SignalSpy {
        id: spy_stringProp
        target: testObject
        signalName: "stringPropChanged"
    }

    QtObject {
        id: testObjectMirror
        property bool boolProp: testObject.boolProp
        property bool boolPropFunction: false
        property int intProp64: testObject.intProp64
        property int intProp64Function: 0
        property int intProp32: testObject.intProp32
        property int intProp32Function: 0
        property int intProp8: testObject.intProp8
        property int intProp8Function: 0
        property int intGetterProp: testObject.intGetterProp
        property int intGetterPropFunction: 0
        property int intConstProp: testObject.intConstProp
        property real floatProp: testObject.floatProp
        property real floatPropFunction: 0
        property string stringProp: testObject.stringProp
        property string stringPropFunction: ""
    }

    Connections {
        target: testObject

        function onBoolPropChanged() {
            testObjectMirror.boolPropFunction = testObject.boolProp;
        }

        function onIntProp64Changed() {
            testObjectMirror.intProp64Function = testObject.intProp64;
        }

        function onIntProp32Changed() {
            testObjectMirror.intProp32Function = testObject.intProp32;
        }

        function onIntProp8Changed() {
            testObjectMirror.intProp8Function = testObject.intProp8;
        }

        function onIntGetterPropChanged() {
            testObjectMirror.intGetterPropFunction = testObject.intGetterProp;
        }

        function onFloatPropChanged() {
            testObjectMirror.floatPropFunction = testObject.floatProp;
        }

        function onStringPropChanged() {
            testObjectMirror.stringPropFunction = testObject.stringProp;
        }
    }

    function test_void_signal() {
        compare(spy_triggered.count, 0)
        testObject.trigger()
        compare(spy_triggered.count, 1)
    }

    function test_string_signal() {
        compare(spy_string.count, 0)

        testObject.stringSlot("hello rust")
        compare(spy_string.count, 1)
        compare(spy_string.signalArguments[0][0], "hello rust")

        testObject.stringSlot("ワンピース")
        compare(spy_string.signalArguments[1][0], "ワンピース")
    }

    function test_bool_property() {
        testObject.boolProp = true;
        compare(spy_boolProp.count, 1)
        compare(testObjectMirror.boolProp, testObject.boolProp)
        compare(testObjectMirror.boolPropFunction, testObject.boolProp)
        testObject.boolProp = true;
        compare(spy_boolProp.count, 1)
        compare(testObjectMirror.boolProp, testObject.boolProp)
        compare(testObjectMirror.boolPropFunction, testObject.boolProp)
        testObject.boolProp = false;
        compare(spy_boolProp.count, 2)
        compare(testObjectMirror.boolProp, testObject.boolProp)
        compare(testObjectMirror.boolPropFunction, testObject.boolProp)

    }

    function test_int64_property() {
        testObject.intProp64 = 5;
        compare(spy_intProp64.count, 1)
        compare(testObjectMirror.intProp64, testObject.intProp64)
        compare(testObjectMirror.intProp64Function, testObject.intProp64)
        testObject.intProp64 = 42;
        compare(spy_intProp64.count, 2)
        compare(testObjectMirror.intProp64, testObject.intProp64)
        compare(testObjectMirror.intProp64Function, testObject.intProp64)
        testObject.intProp64 = 42.2;
        compare(spy_intProp64.count, 2)
        compare(testObjectMirror.intProp64, testObject.intProp64)
        compare(testObjectMirror.intProp64Function, testObject.intProp64)
        testObject.intProp64 = 365.5;
        compare(spy_intProp64.count, 3)
        compare(testObjectMirror.intProp64, testObject.intProp64)
        compare(testObjectMirror.intProp64Function, testObject.intProp64)
    }

    function test_int32_property() {
        testObject.intProp32 = 5;
        compare(spy_intProp32.count, 1)
        compare(testObjectMirror.intProp32, testObject.intProp32)
        compare(testObjectMirror.intProp32Function, testObject.intProp32)
        testObject.intProp32 = 42;
        compare(spy_intProp32.count, 2)
        compare(testObjectMirror.intProp32, testObject.intProp32)
        compare(testObjectMirror.intProp32Function, testObject.intProp32)
        testObject.intProp32 = 42.2;
        compare(spy_intProp32.count, 2)
        compare(testObjectMirror.intProp32, testObject.intProp32)
        compare(testObjectMirror.intProp32Function, testObject.intProp32)
        testObject.intProp32 = 365.5;
        compare(spy_intProp32.count, 3)
        compare(testObjectMirror.intProp32, testObject.intProp32)
        compare(testObjectMirror.intProp32Function, testObject.intProp32)
    }

    function test_int_getter_property() {
        testObject.intGetterProp = 5;
        compare(spy_intGetterProp.count, 1)
        compare(testObjectMirror.intGetterProp, testObject.intGetterProp)
        compare(testObjectMirror.intGetterPropFunction, testObject.intGetterProp)
        testObject.intGetterProp = 42;
        compare(spy_intGetterProp.count, 2)
        compare(testObjectMirror.intGetterProp, testObject.intGetterProp)
        compare(testObjectMirror.intGetterPropFunction, testObject.intGetterProp)
        testObject.intGetterProp = 42.2;
        compare(spy_intGetterProp.count, 2)
        compare(testObjectMirror.intGetterProp, testObject.intGetterProp)
        compare(testObjectMirror.intGetterPropFunction, testObject.intGetterProp)
        testObject.intGetterProp = 365.5;
        compare(spy_intGetterProp.count, 3)
        compare(testObjectMirror.intGetterProp, testObject.intGetterProp)
        compare(testObjectMirror.intGetterPropFunction, testObject.intGetterProp)
    }

    function test_int_const_property() {
        var threw = false
        try {
            testObject.intConstProp = 5;
        } catch (e) {
            threw = true
            compare(e.message, "Cannot assign to read-only property \"intConstProp\"")
        }
        verify(threw, "Expected assigning to const property to throw")
        compare(testObject.intConstProp, 0)
        compare(testObjectMirror.intConstProp, 0)
    }

    function test_float_property() {
        testObject.floatProp = 5;
        compare(spy_floatProp.count, 1)
        compare(testObjectMirror.floatProp, testObject.floatProp)
        compare(testObjectMirror.floatPropFunction, testObject.floatProp)
        testObject.floatProp = 42;
        compare(spy_floatProp.count, 2)
        compare(testObjectMirror.floatProp, testObject.floatProp)
        compare(testObjectMirror.floatPropFunction, testObject.floatProp)
        testObject.floatProp = 42.2;
        compare(spy_floatProp.count, 3)
        compare(testObjectMirror.floatProp, testObject.floatProp)
        compare(testObjectMirror.floatPropFunction, testObject.floatProp)
        testObject.floatProp = 365.5;
        compare(spy_floatProp.count, 4)
        compare(testObjectMirror.floatProp, testObject.floatProp)
        compare(testObjectMirror.floatPropFunction, testObject.floatProp)
    }

    function test_string_property() {
        testObject.stringProp = "Hello rust";
        compare(spy_stringProp.count, 1)
        compare(testObjectMirror.stringProp, testObject.stringProp)
        compare(testObjectMirror.stringPropFunction, testObject.stringProp)
        testObject.stringProp = "Hello Qml!";
        compare(spy_stringProp.count, 2)
        compare(testObjectMirror.stringProp, testObject.stringProp)
        compare(testObjectMirror.stringPropFunction, testObject.stringProp)
        testObject.stringProp = "Hello Qml!";
        compare(spy_stringProp.count, 2)
        compare(testObjectMirror.stringProp, testObject.stringProp)
        compare(testObjectMirror.stringPropFunction, testObject.stringProp)
        testObject.stringProp = "Hello Qt!";
        compare(spy_stringProp.count, 3)
        compare(testObjectMirror.stringProp, testObject.stringProp)
        compare(testObjectMirror.stringPropFunction, testObject.stringProp)
    }
}
