import QtQuick 2.0
import Sailfish.Silica 1.0

Page {
    id: page
    allowedOrientations: Orientation.All

    function login(user, pass, server) {
        librespot.username = usernameField.text
        librespot.password = passwordField.text
        pageStack.push(Qt.resolvedUrl("LoginProgressPage.qml"))
    }

    SilicaFlickable {
        anchors.fill: parent

        Column {
            id: column

            width: page.width
            spacing: Theme.paddingLarge

            PageHeader {
                title: qsTr("Matrix Login")
            }

//            Image {
//                id: logo
//                source: "qrc:det/icons/matrix-logo.svg"
//                y: Theme.paddingMedium
//                height: Theme.itemSizeExtraLarge
//                width: parent.width

//                fillMode: Image.PreserveAspectFit
//                asynchronous: true
//            }

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
                        passwordField.text,
                        serverField.text)
            }
        }

        VerticalScrollDecorator { }
    }
}
