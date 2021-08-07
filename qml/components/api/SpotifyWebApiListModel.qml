import QtQuick 2.0

ListModel {
    id: model

    property alias accessToken: request.accessToken
    property int limit: -1
    readonly property alias busy: request.busy
    readonly property bool completlyFetched: _nextOffset >= total

    property var dataDelegate

    // readonly
    property int total: -1
    property string errorType
    property string errorMessage

    property int _nextOffset: -1
    property string _path
    property var _params: null

    property var request: SpotifyWebApiRequest {
        id: request
        onSuccess: {
            var data = responseData
            if (dataDelegate) {
                data = dataDelegate(data)
            }

            if (data.offset === undefined) {
                console.error("offset key missing, got keys " + Object.keys(data))
                return;
            }

            if (model._nextOffset !== data.offset) {
                console.warn("double result: " + model._nextOffset + " " + data.offset)
                return;
            }

            var items = data.items

            console.log("Web API Success: " + (data.offset + items.length) + " / " + data.total)

            model.total = data.total
            model._nextOffset = data.offset + data.limit

            for (var i = 0; i < items.length; i++) {
                var item = items[i]
                if (item.is_playable !== false) {
                    model.append(items[i])
                }
            }
        }
    }

    function fetchFirst(path, params) {
        total = -1
        _nextOffset = 0
        _path = path
        _params = params ? params : {}

        model.clear()
        _fetch()
    }

    function reset() {
        total = -1
        _nextOffset = -1
        _path = ""
        _params = null

        request.abort()
        model.clear()
    }

    function fetchNext() {
        if (_nextOffset < total && !busy) {
            _fetch()
        }
    }

    function _fetch() {
        if (limit > 0) {
            _params["limit"] = limit
        } else {
            delete _params["limit"]
        }

        _params["offset"] = _nextOffset
        errorType = ""
        request.executeApi("GET", _path, _params)
    }

    // APIS

    function fetchArtistsAlbums(artistId, include_groups) {
        var params = { market: "from_token" }
        if (include_groups) {
            params["include_groups"] = include_groups
        }
        fetchFirst("artists/" + artistId + "/albums", params)
    }

    function fetchAlbumsTracks(albumId) {
        fetchFirst("albums/" + albumId + "/tracks", { market: "from_token" })
    }

    function fetchSavedTracks() {
        fetchFirst("me/tracks")
    }

    function fetchTop(type, timeRange) {
        var params = {}
        if (timeRange) {
            params["time_range"] = timeRange
        }
        fetchFirst("me/top/" + type, params)
    }

    function fetchPlaylists() {
        fetchFirst("me/playlists")
    }

    function search(query, type) {
        var params = {
            q: query,
            market: "from_token"
        }
        if (type) {
            params["type"] = type
        }
        fetchFirst("search", params)
    }
}
