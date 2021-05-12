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

    property int progressBarHeight: Theme.paddingSmall

    BackgroundItem {
        id: playingItem
        width: parent.width
        height: Theme.itemSizeMedium + progressBarHeight

        Rectangle {
            id: progressBar

            height: progressBarHeight
            width: librespot.duration ? parent.width * librespot.position / librespot.duration : 0
            color: Theme.highlightColor
            opacity: 0.70
        }

        Rectangle {
            anchors {
                left: progressBar.right
                right: parent.right
            }
            height: progressBarHeight
            color: Theme.highlightBackgroundColor
            opacity: Theme.highlightBackgroundOpacity
        }

        Image {
            id: albumArt
            fillMode: Image.PreserveAspectCrop
            source: playingMetadata.albumImage
            width: Theme.itemSizeMedium
            height: Theme.itemSizeMedium
            y: progressBarHeight
        }

        Label {
            id: nameLabel

            anchors {
                top: albumArt.top
                left: albumArt.right
                right: playPauseButton.left
                leftMargin: Theme.paddingMedium
                rightMargin: Theme.paddingMedium
            }

            text: playingMetadata.name
            truncationMode: TruncationMode.Fade
        }

        Label {
            id: artistsLabel

            anchors {
                top: nameLabel.bottom
                left: albumArt.right
                right: playPauseButton.left
                leftMargin: Theme.paddingMedium
                rightMargin: Theme.paddingMedium
            }

            text: playingMetadata.artists
            truncationMode: TruncationMode.Fade
        }

        IconButton {
            id: playPauseButton
            anchors {
                top: albumArt.top
                right: parent.right
                bottom: albumArt.bottom
            }

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
