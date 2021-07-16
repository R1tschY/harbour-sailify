import QtQuick 2.0
import Sailfish.Silica 1.0
import "../components"
import "../components/api"
import "../spotifyUtils.js" as SpotifyUtils

Page {
    id: page

    property string albumId
    property var album

    SpotifyWebApiRequest {
        id: request
    }

    PageListView {
        id: listView

        placeholder: qsTr("This album has no tracks")

        header: Column {
            width: page.width

            readonly property string _imageUrl: SpotifyUtils.chooseImage(album.images, width)

            Image {
                source: _imageUrl
                fillMode: Image.PreserveAspectCrop
                width: page.width
                height: page.width
                asynchronous: true
                visible: !!_imageUrl
            }

            PageHeader {
                title: album.name
            }

            ColumnView {
                width: parent.width
                model: album.copyrights
                itemHeight: Theme.itemSizeSmall

                delegate: Label {
                    anchors {
                        left: parent.left
                        leftMargin: Theme.horizontalPageMargin
                        right: parent.right
                        rightMargin: Theme.horizontalPageMargin
                    }

                    truncationMode: TruncationMode.Fade
                    text: modelData.text
                    font.pixelSize: Theme.fontSizeExtraSmall
                }
            }
        }

        delegate: AlbumTrackListItem {
            id: itemItem
            // TODO: explicit

            name_: name
            playing: librespot.trackUri === uri
            trackNumber: track_number
            artists_: artists
            durationMs: duration_ms

            onClicked: request.play(uri, "spotify:album:" + albumId, librespot.deviceId)
        }

        section {
            property: "disc_number"
            delegate: sectionHeading
        }

        Component {
            id: sectionHeading

            SectionHeader {
                text: qsTr("Disc %1").arg(section)
            }
        }
    }

    // TODO: copyrights[].text, artists[].name, release_date

    // TODO: fetch album and init model with track information
    onAlbumIdChanged: listView.model.fetchAlbumsTracks(albumId)
}
