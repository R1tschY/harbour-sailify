import QtQuick 2.0
import Sailfish.Silica 1.0
import "../components"

Page {
    id: page

    PageListView {
        id: listView

        placeholder: qsTr("No playlists")

        header: PageHeader {
            title: qsTr("Playlists")
        }

        delegate: ResultListItem {
            id: itemItem

            name_: name
            images_: images
            fallbackIcon: "image://theme/icon-m-media-playlists"
        }
    }

    Component.onCompleted: {
        listView.model.fetchPlaylists()
    }
}
