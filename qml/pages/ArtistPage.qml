import QtQuick 2.0
import Sailfish.Silica 1.0
import "../components"
import "../spotifyUtils.js" as SpotifyUtils

Page {
    id: page

    property string artistId
    property var artist

    SilicaFlickable {
        id: contentFlickable
        anchors.fill: parent
        contentHeight: contentColumn.height

        Column {
            id: contentColumn

            width: page.width

            readonly property string _imageUrl: SpotifyUtils.chooseImage(artist.images, width)

            Image {
                source: parent._imageUrl
                fillMode: Image.PreserveAspectCrop
                width: parent.width
                height: parent.width
                asynchronous: true
                visible: !!parent._imageUrl
            }

            PageHeader {
                title: artist.name
            }

            IconListItem {
                title: qsTr("Top Tracks")
                source: "image://theme/icon-m-media-songs"
                onClicked: pageStack.push(Qt.resolvedUrl("ArtistTopTracks.qml"), {
                    artistId: artistId,
                    name: artist.name
                })
            }

            IconListItem {
                title: qsTr("Discography")
                source: "image://theme/icon-m-media-albums"
                onClicked: pageStack.push(Qt.resolvedUrl("Discography.qml"), {
                    artistId: artistId,
                    name: artist.name
                })
            }

            IconListItem {
                title: qsTr("Related Artists")
                source: "image://theme/icon-m-media-artists"
                onClicked: pageStack.push(Qt.resolvedUrl("RelatedArtists.qml"), {
                    artistId: artistId,
                    name: artist.name
                })
            }
        }

        VerticalScrollDecorator { flickable: contentFlickable }
    }
}
