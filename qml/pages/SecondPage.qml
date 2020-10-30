import QtQuick 2.0
import Sailfish.Silica 1.0


Dialog {
    id: page

    SilicaFlickable {
        anchors.fill: parent

        contentHeight: column.height

        Column {
            id: column

            width: page.width
            spacing: Theme.paddingLarge

            PageHeader {
                title: qsTr("Configure device")
            }

            TextField {
                id: usernameField
                label: "Username"
                placeholderText: label
                width: parent.width

                EnterKey.iconSource: "image://theme/icon-m-enter-next"
                EnterKey.onClicked: passwordField.focus = true
            }

            PasswordField {
                id: passwordField
                EnterKey.iconSource: "image://theme/icon-m-enter-accept"
                EnterKey.onClicked: accept()
            }

            Label {
                text: "Device support is powered by Librespot 0.X"
            }
        }

        VerticalScrollDecorator {}
    }

    onOpened: {
        usernameField.text = librespot.username
        passwordField.text = librespot.password
    }

    onDone: {
        if (result == DialogResult.Accepted) {
            librespot.username = usernameField.text
            librespot.password = passwordField.text
            librespot.start()
        }
    }
}





