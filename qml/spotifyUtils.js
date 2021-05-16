.pragma library

function joinNames(objects) {
    if (objects.map) {
        return objects.map(function(a) { return a.name; }).join(", ")
    } else {
        var res = [];
        for (var i = 0; i < objects.count; i++) {
            res.push(objects.get(i).name)
        }
        return res.join(", ")
    }
}


function durationMsToString(durationMs) {
    var durationSec = durationMs / 1000

    var min = Math.floor(durationSec / 60)
    var sec = Math.round(durationSec % 60)

    if (sec >= 10) {
        return min + ":" + sec
    } else {
        return min + ":0" + sec
    }
}
