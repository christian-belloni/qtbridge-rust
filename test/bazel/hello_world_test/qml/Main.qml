import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Fusion
import hello_world_test
// import 'qrc:/qt/qml/CustomColor'
import CustomColor

ApplicationWindow {
  visible: true
  title: qsTr("main_window")
  color: Colors.brandColor

  Button {
    anchors.centerIn: parent
    text: Backend.counter
    onClicked: {
      Backend.reset()
    }
  }

  Component.onCompleted: {
    Backend.startup()
  }
}
