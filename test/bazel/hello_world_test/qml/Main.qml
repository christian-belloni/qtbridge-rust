import QtQuick
import QtQuick.Controls
import hello_world_test

ApplicationWindow {

    visible: true
    title: qsTr("Minimal QML app")

    Button {
        anchors.centerIn: parent
        text: "Hello World!"
        onClicked: Backend.say_hello()
    }
}
