import QtQuick 2.0
import Sailfish.Silica 1.0
import "../components"
import "../components/api"

Page {
    id: page

    property string artistId
    property string name

    SpotifyWebApiRequest {
        id: request


    }

    PageListView {
        id: listView

        fallbackIcon: "image://theme/icon-m-media-albums"
        placeholder: qsTr("No albums for this artist")

        header: PageHeader {
            title: name
        }

        delegate: ResultListItem {
            id: itemItem

            name_: name
            images_: images
            fallbackIcon: "image://theme/icon-m-media-albums"

            menu: ContextMenu {
                MenuItem {
                    text: qsTr("Play")
                    onClicked: request.play(null, uri)
                }
            }
        }
    }

    Component.onCompleted: {
        listView.model.fetchArtistsAlbums(artistId, ["album"])
    }
}
