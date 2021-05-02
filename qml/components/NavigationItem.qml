import QtQuick 2.0
import Sailfish.Silica 1.0

IconListItem {
    property var page
    property var pageProperties: ({})

    onClicked: {
        pageStack.push(page, pageProperties)
    }
}
