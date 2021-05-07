import QtQuick 2.0
import Sailfish.Silica 1.0
import "../components"

Page {
    id: page

    property string artistId
    property string name

    PageListView {
        id: listView

        fallbackIcon: "image://theme/icon-m-media-albums"
        placeholder: qsTr("No albums for this artist")

        header: PageHeader {
            title: name
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
                fallbackIcon: "image://theme/icon-m-media-albums"
            }
        }
    }

    Component.onCompleted: {
        listView.model.fetchArtistsAlbums(artistId, ["album"])
    }
}
