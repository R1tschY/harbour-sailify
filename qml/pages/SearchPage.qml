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

        delegate: Item {
            width: page.width
            height: itemItem.height

            property string _name: name
            property var _images: images

            ResultListItem {
                id: itemItem

                name: _name
                images: _images
                fallbackIcon: "image://theme/icon-m-media-albums"

                onClicked: {
                    var props = {
                        "artistId": modelData.id,
                        "name":  modelData.name
                    }
                    pageStack.push(Qt.resolvedUrl("ArtistPage.qml"), props)
                }
            }
        }

        dataDelegate: function(data) {
            return data.artists
        }
    }
}


