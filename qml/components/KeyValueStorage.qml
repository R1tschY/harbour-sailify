import QtQuick 2.0
import QtQuick.LocalStorage 2.0

QtObject {
    property string dataBaseId

    property var _db

    onDataBaseIdChanged: {
        if (dataBaseId) {
            _db = LocalStorage.openDatabaseSync(dataBaseId, "1", "", 1000000)
            _db.transaction(function (tx) {
                tx.executeSql('CREATE TABLE IF NOT EXISTS kv (key TEXT PRIMARY KEY, value TEXT)')
                tx.executeSql('CREATE TABLE IF NOT EXISTS events (type TEXT, timestamp INT, value TEXT)')
                tx.executeSql('CREATE INDEX IF NOT EXISTS idx_events_type ON events (type)')
            })
        } else {
            _db = null
        }
    }

    // Key Value

    function get(key) {
        var res
        _db.transaction(function (tx) {
            var results = tx.executeSql('SELECT value FROM kv WHERE key = ?', [key])
            if (results.rows.length > 0) {
                res = JSON.parse(results.rows.item(0).value)
            }
        })
        return res
    }

    function put(key, value) {
        _db.transaction(function (tx) {
            tx.executeSql(
                'INSERT INTO kv (key, value) VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value=excluded.value',
                [key, JSON.stringify(value)])
        })
    }

    function del(key) {
        _db.transaction(function (tx) {
            tx.executeSql('DELETE FROM kv WHERE key = ?', [key])
        })
    }

    function clear() {
        _db.transaction(function (tx) {
            tx.executeSql('DELETE FROM kv')
        })
    }

    // Events

    function getEvents(type, limit) {
        var res = []
        _db.transaction(function (tx) {
            var results
            if (limit) {
                results = tx.executeSql('SELECT value FROM events WHERE type = ? ORDER BY timestamp', [type])
            } else {
                results = tx.executeSql('SELECT value FROM events WHERE type = ? ORDER BY timestamp LIMIT ?', [type, limit])
            }

            var length = results.rows.length
            for (var i = 0; i < length; i++) {
                res.push(JSON.parse(results.rows.item(i).value))
            }
        })
        return res
    }

    function pushEvent(type, value) {
        var timestamp = new Date().getTime()
        _db.transaction(function (tx) {
            tx.executeSql(
                'INSERT INTO events (type, timestamp, value) VALUES (?, ?, ?)',
                [type, timestamp, JSON.stringify(value)])
        })
    }

    function clearEvents() {
        _db.transaction(function (tx) {
            tx.executeSql('DELETE FROM events')
        })
    }
}
