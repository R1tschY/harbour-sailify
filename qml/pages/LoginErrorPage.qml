import QtQuick 2.0
import Sailfish.Silica 1.0

Page {
    id: page

    property bool allowLogout: true

    SilicaFlickable {
        id: flickable

        anchors.fill: parent

        Column {
            id: column

            width: page.width
            spacing: Theme.paddingLarge

            PageHeader {
                id: header
                title: qsTr("Login Error")
            }

            Label {
                id: label
                anchors {
                    right: parent.right
                    rightMargin: Theme.horizontalPageMargin
                    left: parent.left
                    leftMargin: Theme.horizontalPageMargin
                }
                text: librespot.errorString || qsTr("Unknown error")

                color: Theme.secondaryHighlightColor
                wrapMode: Text.Wrap
            }

            Button {
               text: qsTr("Retry")
               anchors {
                   horizontalCenter: parent.horizontalCenter
               }
               onClicked: pageStack.replace(
                    Qt.resolvedUrl("LoginProgressPage.qml"), {}, PageStackAction.Immediate)
            }
        }

        PullDownMenu {           
            active: allowLogout

            MenuItem {
                id: logout
                text: qsTr("Logout")
                onClicked: app.logout()
                visible: allowLogout
            }
        }

        VerticalScrollDecorator { flickable: flickable }
    }
}
