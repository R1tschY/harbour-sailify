import QtQuick 2.0
import "."

HttpRequest {
    property string accessToken: librespot.token

    function getCurrentPlayback() {
        executeApi("GET", "me/player", { market: "from_token" })
    }

    function getTrack(trackId) {
        executeApi("GET", "tracks/" + trackId, { market: "from_token" })
    }

    function search(query, type) {
        executeApi("GET", "search", {"q": query, "type": type, "market": "from_token" })
    }

    function getAlbum(albumId) {
        executeApi("GET", "albums/" + albumId, { market: "from_token" })
    }

    function play(trackUri, contextUri, deviceId, positionMs) {
        var params = {}
        if (deviceId) {
            params["device_id"] = deviceId
        }

        var body = {}
        if (contextUri) {
            body["context_uri"] = contextUri
        }
        if (trackUri) {
            body["offset"] = { uri: trackUri }
        }
        if (positionMs) {
            body["position_ms"] = positionMs
        }

        executeApi("PUT", "me/player/play", params, body, "text")
    }

    function executeApi(method, path, params, data, responseType) {
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
            "responseType": responseType || "json",
            "params": params,
            "data": data
        })
    }
}
