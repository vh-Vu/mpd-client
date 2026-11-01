import QtQuick
import QtQuick.Controls
import mpd_client;

ApplicationWindow {
    id: root
    visible: true
    title: qsTr("MPD Client")
    width: 600
    height: 200
    Button {
        anchors.centerIn: parent
        text: "Hello WorldAA!"
        onClicked: backend.say_hello()
    }
    Button{
        text: backend.is_play ? "Pause" : "Play" 
        onClicked: backend.toggle_play()
    }
    Backend{
        id: backend
    }
}
