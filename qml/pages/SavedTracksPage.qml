import QtQuick 2.0
import Sailfish.Silica 1.0
import "../components"

Page {
    id: page

    PageListView {
        id: listView

        placeholder: qsTr("No saved tracks")

        header: PageHeader {
            title: qsTr("Saved tracks")
        }

        properties: ["track"]

        delegate: ResultListItem {
            id: itemItem

            name_: track.name
            images_: track.album.images
            fallbackIcon: "image://theme/icon-m-media-songs"
        }
    }

    Component.onCompleted: {
        listView.model.fetchSavedTracks()
    }
}
