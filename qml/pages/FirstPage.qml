import QtQuick 2.0
import Sailfish.Silica 1.0


Page {
    id: page

    // To enable PullDownMenu, place our content in a SilicaFlickable
    SilicaFlickable {
        anchors.fill: parent

        // PullDownMenu and PushUpMenu must be declared in SilicaFlickable, SilicaListView or SilicaGridView
        PullDownMenu {
            MenuItem {
                text: qsTr("Configure librespot device")
                onClicked: pageStack.push(Qt.resolvedUrl("SecondPage.qml"))
            }
        }

        contentHeight: column.height

        // Place our content in a Column.  The PageHeader is always placed at the top
        // of the page, followed by our content.
        Column {
            id: column

            width: page.width
            spacing: Theme.paddingLarge

            PageHeader {
                title: qsTr("Sailify")
            }

            DetailItem {
                label: qsTr("Errors")
                value: librespot.error
            }

            DetailItem {
                label: qsTr("Active")
                value: librespot.active
            }
        }
    }

    Component.onCompleted: {
        if (!librespot.active && !librespot.error) {
            // not started yet -> try to connect
            librespot.start();
        }
    }
}


