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

    function search(query) {
        executeApi("GET", "search", {"q": query, "type": "artist"})
    }

    function executeApi(method, path, params) {
        console.log("Web API " + method + " " + path)
        if (!accessToken) {
            console.error("Request without token not possible")
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
