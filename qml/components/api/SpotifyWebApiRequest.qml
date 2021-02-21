import QtQuick 2.0
import "."

HttpRequest {
    property string accessToken: ""

    function getCurrentPlayback() {
        executeApi("GET", "me/player")
    }

    function getTrack(trackId) {
        executeApi("GET", "tracks/" + trackId)
    }

    function executeApi(method, path, params) {
        console.log("Web API " + method + " " + path)
        if (!accessToken) {
            console.error("Request with without token not possible")
            return
        }

        execute({
            "method": method,
            "url": "https://api.spotify.com/v1/" + path,
            "headers": {
                "Authorization": "Bearer " + accessToken
            },
            "responseType": "json",
            "params": params
        })
    }
}
