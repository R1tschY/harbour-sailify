import QtQuick 2.0
import Sailfish.Silica 1.0
import "../components"
import "../components/api"

Page {
    id: page

    SpotifyWebApiRequest {
        id: searchRequest

        accessToken: librespot.token

        function update(query) {
            if (query.length > 0) {
                searchRequest.search(query)
            } else {
                listView.model = 0
            }
        }

        onSuccess: {
            console.log(JSON.stringify(response))
            var data = response.data
            listView.model = data.artists.items
        }
    }

    SilicaListView {
        id: listView
        anchors.fill: parent

        //model:

        header: SearchField {
            id: searchField

            placeholderText: qsTr("Search")
            EnterKey.enabled: text.length > 0
            EnterKey.iconSource: "image://theme/icon-m-search"

            EnterKey.onClicked: searchRequest.update(text)

            Component.onCompleted: searchField.forceActiveFocus()
        }

        delegate: ResultListItem {
            id: itemItem

            name: modelData.name
            images: modelData.images
        }

        ViewPlaceholder {
            enabled: listView.count === 0
            text: qsTr("Nothing found")
        }

        VerticalScrollDecorator { flickable: listView }
    }
}
