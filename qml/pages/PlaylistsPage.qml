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

        delegate: Item {
            width: page.width
            height: itemItem.height

            property string _name: name
            property var _images: images

            ResultListItem {
                id: itemItem

                name: _name
                images: _images
                fallbackIcon: "image://theme/icon-m-media-playlists"
            }
        }
    }

    Component.onCompleted: {
        listView.model.fetchTop("tracks", "medium_term")
    }
}
