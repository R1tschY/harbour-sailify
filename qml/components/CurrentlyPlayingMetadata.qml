import QtQuick 2.0
import "api"

Object {
    SpotifyWebApiRequest {
        id: request
        accessToken: librespot.token

        onSuccess: {
            console.log(JSON.stringify(response))
            var data = response.data
            uri = data.uri
            name = data.name
            trackNumber = data.track_number
            albumName = data.album.name
            albumImage = data.album.images[0].url
            artists = data.artists.map(function(artist) { return artist.name; }).join(", ")
        }
    }

    property string uri: ""
    property string name: ""
    property string artists: ""
    property int trackNumber: -1

    property string albumName: ""
    property string albumImage: ""

    Connections {
        target: librespot
        onTrackUriChanged: {
            if (value) {
                request.getTrack(parseSpotifyId(value)[2])
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
