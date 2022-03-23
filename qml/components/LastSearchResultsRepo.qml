import QtQuick 2.0
import QtQuick.LocalStorage 2.0

QtObject {
    property string dataBaseId

    property var _db

    onDataBaseIdChanged: {
        if (dataBaseId) {
            console.log("Load DB " + dataBaseId)
            _db = LocalStorage.openDatabaseSync(dataBaseId, "", "", 1000000)
            if (_db.version === "") {
                console.log("Upgrade DB " + dataBaseId
                    + " from version " + _db.version + " to version 1")
                _db.changeVersion("", "1", function (tx) {
                    tx.executeSql(
                        'CREATE TABLE IF NOT EXISTS last_search_results (' +
                        ' uri TEXT PRIMARY KEY, ' +
                        ' timestamp INTEGER, ' +
                        ' json TEXT' +
                        ')')
                })
            }
        } else {
            _db = null
        }
    }

    function get() {
        var res = []
        _db.readTransaction(function (tx) {
            var results = tx.executeSql('SELECT json FROM last_search_results ORDER BY timestamp DESC')

            var length = results.rows.length
            for (var i = 0; i < length; i++) {
                res.push(JSON.parse(results.rows.item(i).json))
            }
        })
        return res
    }

    function put(uri, timestamp, object) {
        _db.transaction(function (tx) {
            tx.executeSql(
                'INSERT INTO last_search_results' +
                ' (uri, timestamp, json) VALUES (?, ?, ?)' + 
                ' ON CONFLICT(uri) DO UPDATE SET timestamp=excluded.timestamp,json=excluded.json',
                [uri, timestamp, JSON.stringify(object)])
        })
    }

    function del(uri) {
        _db.transaction(function (tx) {
            tx.executeSql('DELETE FROM last_search_results WHERE uri = ?', [uri])
        })
    }

    function clear() {
        _db.transaction(function (tx) {
            tx.executeSql('DELETE FROM last_search_results')
        })
    }
}
