import QtQuick 2.0
import Sailfish.Silica 1.0
import "../components"
import "../components/api"

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
            readonly property string _imageUrl: {
                var images = album.images
                if (images) {
                    if (images.length > 0) {
                        return images[0].url
                    } else if (images.count > 0) {
                        return images.get(0).url
                    } else {
                        return ""
                    }
                } else {
                    return ""
                }
            }

            Image {
                source: _imageUrl
                fillMode: Image.PreserveAspectCrop
                width: page.width
                height: page.width
                asynchronous: true
            }

            PageHeader {
                title: album.name
            }
        }

        delegate: ResultListItem {
            id: itemItem
            // TODO: track_number / duration_ms / disc_number / explicit

            name_: name
            playing: librespot.trackUri === uri
            fallbackIcon: "image://theme/icon-m-media-songs"

            onClicked: request.play(uri, "spotify:album:" + albumId, librespot.deviceId)
        }
    }

    // TODO: copyrights[].text, artists[].name, release_date

    onAlbumIdChanged: listView.model.fetchAlbumsTracks(albumId)
}
