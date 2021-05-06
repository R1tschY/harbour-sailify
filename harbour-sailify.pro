# !!! ONLY PSEUDO QML PROJECT TO USE QT CREATOR !!!

TARGET = harbour-sailify

CONFIG += sailfishapp

DISTFILES += qml/*.qml \
    qml/components/CurrentlyPlayingMetadata.qml \
    qml/components/CurrentlyPlayingPanel.qml \
    qml/components/IconListItem.qml \
    qml/components/NavigationItem.qml \
    qml/components/Object.qml \
    qml/components/PressEffect.qml \
    qml/components/ResultListItem.qml \
    qml/components/api/SpotifyWebApiListModel.qml \
    qml/components/api/SpotifyWebApiRequest.qml \
    qml/pages/ArtistPage.qml \
    qml/pages/MainNavigationPage.qml \
    qml/pages/SearchPage.qml
