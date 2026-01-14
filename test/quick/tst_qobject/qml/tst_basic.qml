// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

import QtQuick 2.3
import QtTest 1.0

TestCase {
    name: "trivial_test"
    id: test

    function two_plus_two() {
        compare(2 + 2, 4)
    }

    function two_times_two() {
        compare(2 * 2, 4)
    }

    function two_devidedby_two() {
        compare(2 / 2, 1)
    }
}
