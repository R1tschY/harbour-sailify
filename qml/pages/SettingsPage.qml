import QtQuick 2.0
import Sailfish.Silica 1.0

Page {
    SilicaFlickable {
        id: contentFlickable
        anchors.fill: parent
        contentHeight: contentColumn.height

        Column {
            id: contentColumn

            width: page.width

            SectionHeader { text: qsTr("Player") }

            ComboBox {
                label: qsTr("Quality (Bitrate)")
                currentIndex: 0
                menu: ContextMenu {
                    MenuItem { text: qsTr("Low (96 kB/s)") }
                    MenuItem { text: qsTr("Normal (160 kB/s)") }
                    MenuItem { text: qsTr("High (320 kB/s)") }
                }
            }

            ComboBox {
                label: qsTr("Normalisation")
                currentIndex: 0
                menu: ContextMenu {
                    MenuItem { text: qsTr("No") }
                    MenuItem { text: qsTr("0.0") }
                    MenuItem { text: qsTr("1.0") }
                }
            }

            TextSwitch {
                text: qsTr("Autoplay")
            }

            TextSwitch {
                text: qsTr("Gapless")
                description: "Activates the Doomsday device"
            }

            SectionHeader { text: qsTr("Cache") }

            TextSwitch {
                text: qsTr("Active")
                description: "Activates the Doomsday device"
            }
        }

        PullDownMenu {
            MenuItem {
                text: qsTr("About")
                onClicked: pageStack.push(Qt.resolvedUrl("AboutPage.qml"))
            }
        }

        VerticalScrollDecorator { flickable: contentFlickable }
    }
}
