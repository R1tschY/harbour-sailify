import QtQuick 2.0
import Sailfish.Silica 1.0
import Sailify 0.1

CoverBackground {
    property bool active: librespot.playbackState !== SailifyPlayer.Stopped && !!playingMetadata.albumImage

    Item {
        visible: active
        anchors.fill: parent

        Image {
            id: albumArtImage
            anchors.fill: parent

            fillMode: Image.PreserveAspectCrop
            source: playingMetadata.albumImage
            sourceSize.height: parent.height
        }

        OpacityRampEffect {
            direction: OpacityRamp.TopToBottom
            offset: 0.15
            slope: 1
            sourceItem: albumArtImage
        }

//        Label {
//            id: label
//            anchors.centerIn: parent
//            text: playingMetadata.name
//        }
    }

    CoverActionList {
        enabled: active
        iconBackground: true

        CoverAction {
            iconSource: librespot.playbackState === SailifyPlayer.Playing
                        ? "image://theme/icon-cover-pause"
                        : "image://theme/icon-cover-play"
            onTriggered: {
                if (librespot.playbackState === SailifyPlayer.Playing) {
                    librespot.pause()
                } else {
                    librespot.play()
                }
            }
        }

        CoverAction {
            iconSource: "image://theme/icon-cover-next-song"
            onTriggered: librespot.next()
        }
    }
}


