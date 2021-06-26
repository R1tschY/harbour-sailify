import QtQuick 2.0
import Sailfish.Silica 1.0
import "../components/api"


Page {
    id: page

    SilicaFlickable {
        anchors.fill: parent
        contentWidth: parent.width
        contentHeight: column.height

        Column {
            id: column

            anchors {
                top: parent.top
            }

            width: page.width
            spacing: Theme.paddingSmall

            PageHeader {
                title: qsTr("Sailify")
            }

            Image {
                fillMode: Image.PreserveAspectCrop
                source: playingMetadata.albumImage
                width: page.width
                height: page.width
            }

            DetailItem {
                label: qsTr("Error")
                value: librespot.errorString
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

            DetailItem {
                label: qsTr("Track URI")
                value: playingMetadata.uri
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
                        if (librespot.playbackStatus === "playing") {
                            librespot.pause()
                        } else {
                            librespot.play()
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

        PullDownMenu {
            MenuItem {
                text: qsTr("Logout")
                onClicked: {
                    librespot.logout()
                    pageStack.replaceAbove(null, Qt.resolvedUrl("LoginPage.qml"))
                }
            }
        }
    }
}


