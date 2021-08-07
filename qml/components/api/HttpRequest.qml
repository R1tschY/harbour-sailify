import QtQuick 2.0

QtObject {
    id: request

    // readonly

    //! Error of last request
    property string errorType: ""
    property string errorMessage: ""

    //! XMLHttpRequest.readyState of last request
    //! 0 UNSENT
    //! 1 OPENED
    //! 2 HEADERS_RECEIVED
    //! 3 LOADING
    //! 4 DONE
    property int readyState: 0

    //! Received JSON data of last request
    property var data: null
    //! HTTP status code of last request
    property var status: 0
    //! HTTP status text of last request
    property var statusText: 0

    // computed

    readonly property bool busy: readyState != 0 && readyState != 4

    // private members

    property var _req: null

    // signals

    signal finished(var response)
    signal error(string errorType, string errorMessage)
    signal success(var response)

    // functions

    function abort() {
        if (_req !== null) {
            _req.abort()
        }
    }

    function getResponseHeader(header) {
        if (_req !== null) {
            return _req.getResponseHeader(header)
        }
    }

    function get(url, config) {
        console.log("GET " + url)
        var configObj = config || {}
        configObj.url = url
        configObj.method = "GET"
        return execute(configObj)
    }

    function readyStateToString(readyState) {
        switch (req.readyState) {
            case 0:
                return "unsent"
            case 1:
                return "opened"
            case 2:
                return "headers-received"
            case 3:
                return "loading"
            case 4:
                return "done"
            default:
                return "unknown (" + readyState + ")"
        }
    }

    function _paramsToQueryString(params) {
        var qs = '';
        var first = true
        for (var key in params) {
            if (params.hasOwnProperty(key)) {
                if (first) {
                    first = false
                } else {
                    qs += '&'
                }
                qs += encodeURIComponent(key) + '=' + encodeURIComponent(params[key]);
            }
        }
        return qs
    }

    function execute(config) {
        if (request._req !== null) {
            request._req.abort()
        }

        var req = new XMLHttpRequest();
        request._req = req

        req.onreadystatechange = function() {
            if (req === null && request._req !== req) {
                return;
            }

            var readyState = req.readyState
            request.readyState = readyState
            if (readyState !== 4 || req.status === 0) {
                return
            }

            var resData
            if (!config.responseType || config.responseType === 'text') {
                resData = req.responseText
            } else if (config.responseType === 'json') {
                resData = JSON.parse(req.responseText)
            } else {
                resData = req.response
            }

            var response = {
                data: resData,
                status: req.status,
                statusText: req.statusText,
                config: config,
                request: req,

                get headers() { return _parseHeaders(req.getAllResponseHeaders()) },
                getHeader: function(name) { return req.getResponseHeader(name) }
            }
            request.status = req.status
            request.statusText = req.statusText
            request.data = req.data

            _finish(response)
            req = null;
        }

        req.onabort = function() {
            if (!req) return;
            _finishWithError("aborted", qsTr("Request aborted"))
            req = null;
        }

        req.onerror = function() {
            if (!req) return;
            _finishWithError("network-error", qsTr("Network error, please check your connection"))
            req = null;
        }

        req.ontimeout = function() {
            if (!req) return;
            _finishWithError("timeout", qsTr("Timeout error, please check your connection"))
            req = null;
        }

        request.readyState = 0

        var url = config.url
        if (config.params) {
            url += '?' + _paramsToQueryString(config.params)
        }

        console.debug("HTTP " + config.method + " " + url)
        req.open(config.method.toUpperCase(), url, true);

        var headers = config.headers
        for (var header in headers) {
            if (headers.hasOwnProperty(header)) {
                req.setRequestHeader(header, headers[header])
            }
        }

        if (config.responseType === 'json') {
            req.setRequestHeader("Accept", "application/json;charset=UTF-8")
        }

        if (config.data) {
            req.setRequestHeader("Content-Type", "application/json;charset=UTF-8")
            req.send(JSON.stringify(config.data));
        } else {
            req.send();
        }
        return req
    }

    function _finishWithError(errorType, errorMessage) {
        request.errorType = errorType
        request.errorMessage = errorMessage || ""
        request.error(errorType, errorMessage || "")
        request.finished(null)
    }

    function _finish(response) {
        request.errorType = ""
        request.success(response)
        request.finished(response)
    }

    function _parseHeaders(headers) {
        var result = {}
        var lines = headers.split("\n")
        var key
        var value
        var line
        var split

        for (var i = 0; i < lines.length; i++) {
            line = lines[i]
            split = line.indexOf(":")
            key = line.substr(0, split).trim()
            value = line.substr(split + 1).trim()

            if (key) {
                result[key] = result[key] ? result[key] + ", " + value : value
            }
        }

        return result
    }
}
