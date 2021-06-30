import QtQuick 2.0
import org.freedesktop.contextkit 1.0

Object {
    property bool online: networkState.value === "connected"

    ContextProperty {
        id: networkState
        key: "Internet.NetworkState"
    }
}
