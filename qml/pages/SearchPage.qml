import QtQuick 2.0
import Sailfish.Silica 1.0
import "../components"

Page {
    id: page

    property bool searched: false

    PageListView {
        id: listView

        placeholder: searched ? qsTr("Nothing found") : ""

        header: Column {
            width: page.width

            PageHeader {
                title: qsTr("Search")
            }

            SearchField {
                id: searchField

                placeholderText: qsTr("Search")
                EnterKey.enabled: text.length > 0
                EnterKey.iconSource: "image://theme/icon-m-search"

                EnterKey.onClicked: listView.model.search(text, ["artist"])

                Component.onCompleted: searchField.forceActiveFocus()
            }
        }

        delegate: ResultListItem {
            id: itemItem

            name_: name
            images_: images
            fallbackIcon: "image://theme/icon-m-media-albums"

            onClicked: {
                var props = {
                    "artistId": id,
                    "name":  name
                }
                pageStack.push(Qt.resolvedUrl("ArtistPage.qml"), props)
            }
        }

        dataDelegate: function(data) {
            return data.artists
        }
    }
}


