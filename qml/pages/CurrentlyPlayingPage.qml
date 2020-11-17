import QtQuick 2.0
import Sailfish.Silica 1.0


Page {
    id: page

    SilicaFlickable {
        anchors.fill: parent
        contentWidth: parent.width
        contentHeight: parent.height

        Column {
            id: column

            anchors {
                top: parent.top
            }

            width: page.width
            spacing: Theme.paddingLarge

            PageHeader {
                title: qsTr("Sailify")
            }

            DetailItem {
                label: qsTr("Error")
                value: librespot.error
            }

            DetailItem {
                label: qsTr("Active")
                value: librespot.active
            }

            DetailItem {
                label: qsTr("Paused")
                value: librespot.paused ? "paused" : "playing"
            }

            DetailItem {
                label: qsTr("Position")
                value: librespot.position
            }

            DetailItem {
                label: qsTr("Duration")
                value: librespot.duration
            }

            DetailItem {
                label: qsTr("Track")
                value: librespot.trackUri
            }

            Row {
                anchors.horizontalCenter: parent.horizontalCenter
                spacing: Theme.paddingMedium

                IconButton {
                    anchors.verticalCenter: parent.verticalCenter
                    icon.source: "image://theme/icon-m-previous"
                    onClicked: librespot.previous()
                }

                IconButton {
                    id: playPauseButton

                    anchors.verticalCenter: parent.verticalCenter
                    icon.source: librespot.paused ? "image://theme/icon-l-play"
                                                  : "image://theme/icon-l-pause"

                    onClicked: {
                        if (librespot.paused) {
                            librespot.play()
                        } else {
                            librespot.pause()
                        }
                    }
                }

                IconButton {
                    anchors.verticalCenter: parent.verticalCenter
                    icon.source: "image://theme/icon-m-next"
                    onClicked: librespot.next()
                }
            }
        }
    }
}


