import QtQuick 2.0

QtObject {
    id: request

    // readonly

    //! Error of last request
    property string errorType: ""

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
    signal error(string errorType)
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
            if (readyState !== 4) {
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
            // console.log("FINISHED " + JSON.stringify(resData))

            var response = {
                data: resData,
                status: req.status,
                statusText: req.statusText,
                headers: req.getAllResponseHeaders(),
                config: config,
                request: req
            }
            request.status = req.status
            request.statusText = req.statusText
            request.data = req.data

            _finish(response)
            req = null;
        }

        req.onabort = function() {
            if (!req) return;
            _finishWithError("aborted")
            req = null;
        }

        req.onerror = function() {
            if (!req) return;
            _finishWithError("network-error")
            req = null;
        }

        req.ontimeout = function() {
            if (!req) return;
            _finishWithError("timeout")
            req = null;
        }

        request.readyState = 0

        var url = config.url
        if (config.params) {
            url += '?' + _paramsToQueryString(config.params)
        }

        var requestData = config.data === undefined ? null : config.data
        req.open(config.method.toUpperCase(), url, true);

        var headers = config.headers
        for (var header in headers) {
            if (headers.hasOwnProperty(header)) {
                req.setRequestHeader(header, headers[header])
            }
        }

        if (config.responseType === 'json') {
            req.setRequestHeader("Accept", "application/json")
        }

        req.send();
        return req
    }

    function _finishWithError(errorType) {
        request.errorType = errorType
        request.error(errorType)
        request.finished(null)
    }

    function _finish(response) {
        request.errorType = ""
        request.success(response)
        request.finished(response)
    }
}
