import QtQuick 2.0
import QtQuick.LocalStorage 2.0

QtObject {
    property string dataBaseId

    property var _db

    onDataBaseIdChanged: {
        if (dataBaseId) {
            _db = LocalStorage.openDatabaseSync(dataBaseId, "1", "", 1000000)
            _db.transaction(function (tx) {
                tx.executeSql('CREATE TABLE IF NOT EXISTS http_cache (url TEXT PRIMARY KEY, etag TEXT, expires INT, data TEXT)')
            })
        } else {
            _db = null
        }
    }

    function get(url) {
        var res
        _db.transaction(function (tx) {
            var results = tx.executeSql(
                'SELECT data, etag FROM http_cache WHERE url = ? AND expires > ?',
                [url, Date.now()])
            if (results.rows.length > 0) {
                var item = results.rows.item(0)
                res = { etag: item.etag, data: JSON.parse(item.data) }
            }
        })
        return res
    }

    function put(url, etag, expires, data) {
        _db.transaction(function (tx) {
            tx.executeSql(
                'INSERT INTO http_cache (url, etag, expires, data) VALUES (?, ?, ?, ?)' +
                        ' ON CONFLICT(url) DO UPDATE SET' +
                        ' etag=excluded.etag,' +
                        ' expires=excluded.expires,' +
                        ' data=excluded.data',
                [url, etag, expires, JSON.stringify(data)])
        })
    }

    function removeExpired() {
        _db.transaction(function (tx) {
            tx.executeSql('DELETE FROM http_cache WHERE expires <= ?; VACUUM', [Date.now()])
        })
    }

    function clear() {
        _db.transaction(function (tx) {
            tx.executeSql('DELETE FROM http_cache; VACUUM', [key])
        })
    }
}
