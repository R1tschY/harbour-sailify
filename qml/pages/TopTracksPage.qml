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

        delegate: ResultListItem {
            id: itemItem

            name_: name
            images_: album.images
            fallbackIcon: "image://theme/icon-m-media-albums"
        }
    }

    Component.onCompleted: {
        listView.model.fetchTop("tracks", "medium_term")
    }
}
