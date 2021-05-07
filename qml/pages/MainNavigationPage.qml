import QtQuick 2.0
import Sailfish.Silica 1.0
import "../components"

Page {
    id: page

    SilicaFlickable {
        id: flickable
        anchors.fill: parent
        contentHeight: deviceColumn.height

        Column {
            id: deviceColumn
            width: page.width

            PageHeader {
                title: qsTr("Navigation")
            }

            NavigationItem {
                title: qsTr("Search")
                source: "image://theme/icon-m-search"
                page: Qt.resolvedUrl("SearchPage.qml")
            }

            NavigationItem {
                title: qsTr("Playing")
                source: "image://theme/icon-m-music"
                page: Qt.resolvedUrl("CurrentlyPlayingPage.qml")
            }

            NavigationItem {
                title: qsTr("Saved tracks")
                source: "image://theme/icon-m-like"
                page: Qt.resolvedUrl("SavedTracksPage.qml")
            }

            NavigationItem {
                title: qsTr("Top tracks")
                source: "image://theme/icon-m-media-songs"
                page: Qt.resolvedUrl("TopTracksPage.qml")
            }

            NavigationItem {
                title: qsTr("Playlists")
                source: "image://theme/icon-m-media-playlists"
                page: Qt.resolvedUrl("PlaylistsPage.qml")
            }

            NavigationItem {
                title: qsTr("Genre & Mood")
                source: "image://theme/icon-m-ambience"
                page: Qt.resolvedUrl("CurrentlyPlayingPage.qml")
            }

            NavigationItem {
                title: qsTr("Top Stuff")
                source: "image://theme/icon-m-like"
                page: Qt.resolvedUrl("CurrentlyPlayingPage.qml")
            }

            NavigationItem {
                title: qsTr("My Stuff")
                source: "image://theme/icon-m-person"
                page: Qt.resolvedUrl("CurrentlyPlayingPage.qml")
            }

            NavigationItem {
                title: qsTr("New & Featured")
                source: "image://theme/icon-m-health"
                page: Qt.resolvedUrl("CurrentlyPlayingPage.qml")
            }

            NavigationItem {
                title: qsTr("History")
                source: "image://theme/icon-m-backup"
                page: Qt.resolvedUrl("CurrentlyPlayingPage.qml")
            }

            NavigationItem {
                title: qsTr("Recommended")
                source: "image://theme/icon-m-acknowledge"
                page: Qt.resolvedUrl("CurrentlyPlayingPage.qml")
            }
        }

        VerticalScrollDecorator {}
    }
}
