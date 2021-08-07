import QtQuick 2.0
import Nemo.Notifications 1.0
import "."
import ".."

Object {
    id: root

    readonly property string accessToken: librespot.accessToken
    readonly property bool busy: _waitForAccessToken || request.busy

    property var _prepared: null
    property bool _waitForAccessToken: false

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
                var match = /max-age=(\d+)/.exec(response.getHeader("cache-control"))
                var cacheControlMaxAge = match ? match[1] : -1
                var etag = response.getHeader("etag")

                if (cacheControlMaxAge > 0 && response.config.method === "GET") {
                    var expires = Date.now() + cacheControlMaxAge
                    console.log("CACHE PUT", response.config.url, etag, cacheControlMaxAge)
                    spotifyApiCache.put(response.config.url, etag, expires, response.data)
                } else {
                    console.log("CACHE IGNORE", response.config.url, response.getHeader("cache-control"))
                }

                root.success(response.data)
            } else {
                var error = response.data.error || {}
                _sendError(
                    "http-" + response.status,
                    error.message || (response.status + ": " + JSON.stringify(response.data)))
            }
        }

        onError: _sendError(errorType, errorMessage)
    }

    Connections {
        target: librespot
        onAccessTokenChanged: {
            if (!_waitForAccessToken || _prepared === null) {
                return
            }

            _executePrepared()
        }
    }

    // signals

    signal finished(var response)
    signal error(string errorType, string errorMessage)
    signal success(var responseData)

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
        _reset()

        if (!accessToken) {
            _sendError("Request without access token not possible")
            return
        }

        // check cache
        var url = "https://api.spotify.com/v1/" + path
        if (params) {
            url += '?' + request._paramsToQueryString(params)
        }

        var etag
        if (method === "GET") {
            var cached = spotifyApiCache.get(url)
            if (cached) {
                console.info("CACHE HIT", url)
                etag = cached.etag
                root.finished({data: cached.data})
                root.success(cached.data)
                return;
            }
        }

        _prepared = {
            "method": method,
            "url": url,
            "headers": {},
            "responseType": responseType || "json",
            "data": data
        }
        if (librespot.accessTokenExpiresIn < 120) {
            _waitForAccessToken = true
            console.info("Refreshing access token for request")
            librespot.refreshAccessToken()
        } else {
            _executePrepared()
        }
    }

    function _executePrepared() {
        if (!accessToken || librespot.accessTokenExpiresIn === 0) {
            _sendError("Got no valid access token")
            return
        }

        _waitForAccessToken = false
        _prepared["headers"]["Authorization"] = "Bearer " + accessToken
        request.execute(_prepared)
    }

    function _reset() {
        _waitForAccessToken = false
        _prepared = null
    }

    // actions

    function abort() {
        _reset()
        request.abort()
    }
}
