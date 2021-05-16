# !!! ONLY PSEUDO QML PROJECT TO USE QT CREATOR !!!

TARGET = harbour-sailify

CONFIG += sailfishapp

DISTFILES += qml/*.qml \
    qml/components/AlbumTrackListItem.qml \
    qml/components/CurrentlyPlayingMetadata.qml \
    qml/components/CurrentlyPlayingPanel.qml \
    qml/components/IconListItem.qml \
    qml/components/NavigationItem.qml \
    qml/components/Object.qml \
    qml/components/PageListView.qml \
    qml/components/PressEffect.qml \
    qml/components/ResultListItem.qml \
    qml/components/api/SpotifyWebApiListModel.qml \
    qml/components/api/SpotifyWebApiRequest.qml \
    qml/pages/AlbumPage.qml \
    qml/pages/ArtistPage.qml \
    qml/pages/MainNavigationPage.qml \
    qml/pages/PlaylistsPage.qml \
    qml/pages/SavedTracksPage.qml \
    qml/pages/SearchPage.qml \
    qml/pages/TopTracksPage.qml \
    qml/spotifyUtils.js
