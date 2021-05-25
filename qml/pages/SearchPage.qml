import QtQuick 2.0
import Sailfish.Silica 1.0
import "../components"

Page {
    id: page

    property bool searched: false

    property string searchType: "artist"

    function typeToHeading(type) {
        switch (type) {
            case "artist": return qsTr("Search artists")
            case "album": return qsTr("Search albums")
            case "track": return qsTr("Search tracks")
            case "playlist": return qsTr("Search playlists")
        }
    }

    PageListView {
        id: listView

        placeholder: searched ? qsTr("Nothing found") : ""

        header: Column {
            width: page.width

            PageHeader {
                title: typeToHeading(searchType)
            }

            SearchField {
                id: searchField

                focus: true

                placeholderText: qsTr("Search")
                EnterKey.enabled: text.length > 0
                EnterKey.iconSource: "image://theme/icon-m-search"

                EnterKey.onClicked: search()
                Component.onCompleted: searchField.forceActiveFocus()
                onTextChanged: searchDebounceTimer.start()

                Timer {
                    id: searchDebounceTimer
                    interval: 1000
                    repeat: false

                    onTriggered: searchField.search()
                }

                function search() {
                    if (text) {
                        listView.model.search(text, [searchType])
                    } else {
                        listView.model.reset()
                    }
                    searchField.forceActiveFocus()
                }
            }
        }

        delegate: ResultListItem {
            id: itemItem

            name_: name
            images_: images
            fallbackIcon: "image://theme/icon-m-media-albums"

            onClicked: {
                if (type === "artist") {
                    pageStack.push(Qt.resolvedUrl("ArtistPage.qml"), {
                        "artistId": id,
                        "name":  name
                    })
                } else if (type === "album") {
                    pageStack.push(Qt.resolvedUrl("AlbumPage.qml"), {
                        "albumId": id,
                        "album": listView.model.get(index)
                    })
                } else {
                    console.log("No action for " + type)
                }

                keyValueStorage.pushEvent("searchResult", uri)
            }
        }

        dataDelegate: function(data) {
            switch (page.searchType) {
                case "artist": return data.artists
                case "album": return data.albums
                case "track": return data.tracks
                case "playlist": return data.playlists
            }
        }

        PullDownMenu {
            MenuItem {
                text: typeToHeading("album")
                onClicked: page.searchType = "album"
                visible: page.searchType !== "album"
            }

            MenuItem {
                text: typeToHeading("track")
                onClicked: page.searchType = "track"
                visible: page.searchType !== "track"
            }

            MenuItem {
                text: typeToHeading("artist")
                onClicked: page.searchType = "artist"
                visible: page.searchType !== "artist"
            }

            MenuItem {
                text: typeToHeading("playlist")
                onClicked: page.searchType = "playlist"
                visible: page.searchType !== "playlist"
            }
        }
    }
}


