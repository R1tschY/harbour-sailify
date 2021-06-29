import QtQuick 2.0
import Nemo.Notifications 1.0
import "."
import ".."

Object {
    id: root

    property string accessToken: librespot.token
    property alias busy: request.busy

    Notification {
        id: notification
    }

    function _sendError(errorType, errorMessage) {
        notification.previewSummary = errorMessage
        notification.publish()
        console.error("Web API error: " + errorType + ": " + errorMessage)
        root.error(errorType, errorMessage)
    }

    HttpRequest {
        id: request

        onFinished: root.finished(response)

        onSuccess: {
            if (response.status >= 200 && response.status < 300) {
                root.success(response)
            } else {
                var error = response.data.error || {}
                _sendError(
                    "http-" + response.status,
                    error.message || error.reason || (response.status + ": " + JSON.stringify(response.data)))
            }
        }

        onError: _sendError(errorType, errorMessage)
    }

    // signals

    signal finished(var response)
    signal error(string errorType, string errorMessage)
    signal success(var response)

    // calls

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

        request.execute({
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
