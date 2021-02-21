import QtQuick 2.0
import Sailfish.Silica 1.0

DockedPanel {
    id: panel

    width: parent.width
    height: Theme.itemSizeMedium

    Behavior on height {
        PropertyAnimation { }
    }

    open: !Qt.inputMethod.visible && !!pageStack.currentPage

    BackgroundItem {
        id: playingItem
        width: parent.width
        height: Theme.itemSizeMedium

        Image {
            id: albumArt
            fillMode: Image.PreserveAspectCrop
            source: playingMetadata.albumImage
            width: Theme.itemSizeMedium
            height: Theme.itemSizeMedium
        }

        Label {
            id: nameLabel
            text: playingMetadata.name
            y: Theme.paddingSmall
            anchors {
                left: albumArt.right
                right: playPauseButton.left
                leftMargin: Theme.paddingMedium
                rightMargin: Theme.paddingMedium
            }
        }

        Label {
            id: artistsLabel
            text: playingMetadata.artists
            anchors {
                top: nameLabel.bottom
                left: albumArt.right
                right: playPauseButton.left
                leftMargin: Theme.paddingMedium
                rightMargin: Theme.paddingMedium
            }
        }

        IconButton {
            id: playPauseButton
            anchors.verticalCenter: parent.verticalCenter
            anchors.right: parent.right

            icon.source: librespot.paused ? "image://theme/icon-m-play"
                                          : "image://theme/icon-m-pause"

            onClicked: {
                if (librespot.paused) {
                    librespot.play()
                } else {
                    librespot.pause()
                }
            }
        }
    }
}
