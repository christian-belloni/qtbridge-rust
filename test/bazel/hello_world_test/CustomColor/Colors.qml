pragma Singleton

import QtQuick
import hello_world_test

QtObject {
  property color brandColor: Application.styleHints.colorScheme === Qt.ColorScheme.Light ? "green" : "red";
}
