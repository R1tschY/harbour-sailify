import QtQuick 2.0
import Sailfish.Silica 1.0

Page {
    id: page
    allowedOrientations: Orientation.All
    objectName: "LoginPage"

    function login(user, pass) {
        librespot.username = usernameField.text
        librespot.password = passwordField.text
        pageStack.push(Qt.resolvedUrl("LoginProgressPage.qml"), {}, PageStackAction.Immediate)
    }

    Connections {
        target: pageStack

        onCurrentPageChanged: {
            if (pageStack.currentPage === page) {
                librespot.logout()
            }
        }
    }

    SilicaFlickable {
        anchors.fill: parent

        Column {
            id: column

            width: page.width
            spacing: Theme.paddingLarge

            PageHeader {
                title: qsTr("Spotify Login")
            }

            Image {
                id: logo
                // TODO: image as local resource
                source: "https://storage.googleapis.com/pr-newsroom-wp/1/2018/11/Spotify_Logo_RGB_Green.png"
                y: Theme.paddingMedium
                height: Theme.itemSizeExtraLarge
                width: parent.width

                fillMode: Image.PreserveAspectFit
                asynchronous: true
            }

            TextField {
                id: usernameField
                label: qsTr("Username")
                placeholderText: label
                width: parent.width

                EnterKey.enabled: text.length > 0
                EnterKey.iconSource: "image://theme/icon-m-enter-next"
                EnterKey.onClicked: passwordField.focus = true
                inputMethodHints: Qt.ImhNoAutoUppercase | Qt.ImhLowercaseOnly
                    | Qt.ImhNoPredictiveText
            }

            PasswordField {
                id: passwordField

                EnterKey.enabled: text.length > 0
                EnterKey.iconSource: "image://theme/icon-m-enter-accept"
                EnterKey.onClicked:
                    login(
                        usernameField.text,
                        passwordField.text)
            }
        }

        VerticalScrollDecorator { }
    }
}
