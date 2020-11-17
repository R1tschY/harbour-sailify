import QtQuick 2.0
import Sailfish.Silica 1.0
import Sailify 0.1
import "pages"

ApplicationWindow
{
    id: app

    property bool darkMode: Theme.colorScheme == Theme.LightOnDark

    Librespot {
        id: librespot
    }

    initialPage: Component { LoginProgressPage { } }
    cover: Qt.resolvedUrl("cover/CoverPage.qml")
}


