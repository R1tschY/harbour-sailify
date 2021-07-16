import QtQuick 2.0
import Sailfish.Silica 1.0
import "../components"
import "../components/api"

Page {
    id: page

    property string artistId
    property string name

    SpotifyWebApiRequest {
        id: request
    }

    PageListView {
        id: listView

        placeholder: qsTr("This artist has no albums")

        header: PageHeader {
            title: name
        }

        delegate: ResultListItem {
            id: itemItem

            name_: name
            images_: images
            fallbackIcon: "image://theme/icon-m-media-albums"

            menu: ContextMenu {
                MenuItem {
                    text: qsTr("Play")
                    onClicked: request.play(null, uri, librespot.deviceId)
                }
            }

            onClicked: {
                pageStack.push(Qt.resolvedUrl("AlbumPage.qml"), {
                                   albumId: id, album: listView.model.get(index)
                               })
            }
        }

        section {
            property: "album_group"
            delegate: sectionHeading
        }

        Component {
            id: sectionHeading

            SectionHeader {
                id: sectionLabel

                x: Theme.horizontalPageMargin
                text: {
                    switch (section) {
                        case "album": return qsTr("Albums")
                        case "single": return qsTr("Singles and EPs")
                        case "appears_on": return qsTr("Appears on")
                        case "compilation": return qsTr("Compilations")
                        default: return "UNKNOWN"
                    }
                }
            }
        }
    }

    Component.onCompleted: {
        listView.model.fetchArtistsAlbums(
            artistId, ["album", "single", "appears_on", "compilation"])
    }
}
