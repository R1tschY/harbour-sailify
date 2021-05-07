import QtQuick 2.0

ListModel {
    id: model

    property alias accessToken: request.accessToken
    property int limit: -1
    readonly property alias busy: request.busy
    readonly property bool completlyFetched: _nextOffset >= total

    // readonly
    property int total: -1
    property string errorType
    property string errorMessage

    property int _nextOffset: 0
    property string _path
    property var _params: null

    property var request: SpotifyWebApiRequest {
        id: request
        onSuccess: {

            var data = response.data

            if (response.status !== 200) {
                model.errorType = response.status
                model.errorMessage = data.error.message
                console.error("Web API Error " + model.errorType + ": " + model.errorMessage)
                return;
            }

            if (model._nextOffset !== data.offset) {
                console.warn("double result")
                return;
            }
            var items = data.items

            console.log("Web API Success: " + (data.offset + items.length) + " / " + data.total)

            model.total = data.total
            model._nextOffset = data.offset + data.limit

            for (var i = 0; i < items.length; i++) {
                model.append(items[i])
            }
        }

        onError: {
            model.errorType = errorType
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
        var params = {}
        if (include_groups) {
            params["include_groups"] = include_groups
        }
        fetchFirst("artists/" + artistId + "/albums", params)
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

}
