import QtQuick 2.0
import "api"

Object {
    id: metadata

    SpotifyWebApiRequest {
        id: request

        onSuccess: {
            var data = responseData
            uri = data.uri || ""
            name = data.name
            trackNumber = data.track_number
            discNumber = data.disc_number
            albumName = data.album.name
            albumImage = data.album.images[0].url
            artistsAsList = data.artists.map(function(artist) { return artist.name; })
            artists = artistsAsList.join(", ")
            metadataChanged()
        }
    }

    property string uri: ""
    property string name: ""
    property string artists: ""
    property var artistsAsList: []
    property int trackNumber: -1
    property int discNumber: -1

    property string albumName: ""
    property string albumImage: ""

    signal metadataChanged()

    Connections {
        target: librespot
        onTrackUriChanged: {
            if (trackUri) {
                request.getTrack(parseSpotifyId(trackUri)[2])
            } else {
                uri = ""
                name = "Not playing"
                trackNumber = -1
                albumName = ""
                albumImage = ""
                artists = ""
            }
        }
    }

    function parseSpotifyId(id) {
        return id.split(":", 3)
    }

    function findImage(images, size) {
        for(var i = 0; i < images.length; i++) {
            var image = images[i]
            Math.max(image.height, image.width)
        }
    }
}
