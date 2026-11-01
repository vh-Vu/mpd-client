import QtQuick
import QtQuick.Controls
import mpd_client;

ApplicationWindow {

    visible: true
    title: qsTr("MPD Client")

    Button {
        anchors.centerIn: parent
        text: "Hello World!"
        onClicked: Backend.say_hello()
    }
}
