import QtQuick
import QtQuick.Controls



Item {
    id: container
    width: 600
    height: 20

    property real currentPosition: 0.0 
    property real duration: 0.0
    property var is_play: false

    signal seekRequested(real position)
    onCurrentPositionChanged:{
        playAnimation.stop();
        slider.value = currentPosition;
        slider.to = duration;
        playAnimation.duration = (container.duration - currentPosition) * 1000;
        if (is_play){
            playAnimation.start();
        }        
    }

    Text{
        // id: 
        text: `${formatDuration(slider.value)}/${formatDuration(container.duration)}`
        //   + " "  + slider.value + " " + is_play
        color: "red"
        anchors{
            left: slider.right
        }
        function formatDuration(seconds) {
            if (!seconds || seconds <= 0) return "--:--";
            var mins = Math.floor(seconds / 60);
            var secs = Math.floor(seconds % 60);
            return mins + ":" + (secs < 10 ? "0" : "") + secs;
        }
    }
    Slider {
        id: slider
        anchors.fill: parent
        from: 0.0
        to: container.duration

        value: targetValue 

        NumberAnimation {
            id: playAnimation
            target: slider
            property: "value"
            from: slider.value
            to: container.duration
            duration: (container.duration - slider.value) * 1000
            running: !slider.pressed && is_play
        }
        onPressedChanged:{
            seekRequested(slider.value)
        }
        onMoved: {
            playAnimation.stop();
            // seekRequested(slider.value)
        }
    }
}