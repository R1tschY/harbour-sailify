import QtQuick 2.0
import Sailfish.Silica 1.0
import Sailify 0.1
import "pages"

ApplicationWindow
{
    Librespot {
        id: librespot
    }

    initialPage: Component { FirstPage { } }
    cover: Qt.resolvedUrl("cover/CoverPage.qml")
}


