import QtQuick 2.0
import "."

HttpRequest {
    property string accessToken: librespot.token

    function getCurrentPlayback() {
        executeApi("GET", "me/player")
    }

    function getTrack(trackId) {
        executeApi("GET", "tracks/" + trackId)
    }

    function search(query, type) {
        executeApi("GET", "search", {"q": query, "type": type})
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

        executeApi("PUT", "me/player/play", params, body)
    }

    function executeApi(method, path, params, data) {
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
            "params": params,
            "data": data
        })
    }
}
