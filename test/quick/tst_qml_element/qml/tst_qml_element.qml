// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

import QtQuick
import QtQuick.Controls
import QtTest

import tst_qml_element

TestCase {
    name: "AbstractModelTest"
    id: test

    Backend {
        id: normal_backend
    }

    function test_element() {
        compare(normal_backend.answerToEverything(), 42)
    }

    function test_singleton() {
        compare(SingletonBackend.answerToEverything(), 42)
    }
}
