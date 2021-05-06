import QtQuick 2.0
import Sailfish.Silica 1.0
import "../components"
import "../components/api"

Page {
    id: page

    property string artistId
    property string name

    SilicaListView {
        id: listView
        anchors.fill: parent

        header: PageHeader {
            title: name
        }

        model: SpotifyWebApiListModel {
            id: listModel
            accessToken: librespot.token
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

        ViewPlaceholder {
            enabled: !listModel.busy && !listModel.errorType && listView.count === 0
            text: qsTr("No albums for this artist")
        }

        VerticalScrollDecorator { flickable: listView }
    }

    Component.onCompleted: {
        listModel.fetchArtistsAlbums(artistId, ["album"])
    }
}
