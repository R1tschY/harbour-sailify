import QtQuick 2.0
import Sailfish.Silica 1.0
import Sailify 0.1
import "pages"
import "components"

ApplicationWindow
{
    id: app

    bottomMargin: playingPanel.parent === contentItem
                  ? 0 : playingPanel.visibleSize

    property bool darkMode: Theme.colorScheme === Theme.LightOnDark

    initialPage: Component { LoginProgressPage { } }
    cover: Qt.resolvedUrl("cover/CoverPage.qml")

    Librespot {
        id: librespot
    }

    CurrentlyPlayingPanel {
        id: playingPanel
        z: 1
    }

    CurrentlyPlayingMetadata {
        id: playingMetadata
    }

    KeyValueStorage {
        id: keyValueStorage

        dataBaseId: "configuration"
    }

    Mpris2Adapter { }

    NetworkMonitor {
        id: networkMonitor
    }
}


