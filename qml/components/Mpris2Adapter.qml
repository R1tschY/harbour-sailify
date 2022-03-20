import QtQuick 2.0
import org.nemomobile.mpris 1.0
import Sailify 0.1
// TODO: import Amber.Mpris 1.0

Object {
    MprisPlayer {
        id: mpris2Adapter

        serviceName: "sailify"
        identity: qsTr("Sailify")
        desktopEntry: "harbour-sailify"

        supportedMimeTypes: "spotify"

        canControl: true
        canSeek: false
        canGoNext: true
        canGoPrevious: true
        canPause: true
        canPlay: true

        playbackStatus: {
            switch (librespot.playbackState) {
            case SailifyPlayer.Playing: return Mpris.Playing
            case SailifyPlayer.Paused: return Mpris.Paused
            case SailifyPlayer.Stopped: return Mpris.Stopped
            default: return Mpris.InvalidPlaybackStatus
            }
        }

        onPauseRequested: librespot.pause()
        onPlayRequested: librespot.play()
        onPlayPauseRequested: {
            if (librespot.playbackState === SailifyPlayer.Playing) {
                librespot.pause()
            } else {
                librespot.play()
            }
        }
        onNextRequested: librespot.next()
        onPreviousRequested: librespot.previous()


        canQuit: true
        canRaise: false
        onQuitRequested: Qt.quit()
    }

    Connections {
        target: playingMetadata

        onMetadataChanged: {
            var metadata = playingMetadata
            if (metadata.uri) {
                mpris2Adapter.metadata = {
                    "mpris:trackid": "/" + metadata.uri.replace(/:/g, "/"),
                    "mpris:length": librespot.duration,
                    "mpris:artUrl": metadata.albumImage,
                    "xesam:album": metadata.albumName,
                    "xesam:artist": metadata.artistsAsList,
                    "xesam:title": metadata.name,
                    "xesam:trackNumber": metadata.trackNumber,
                    "xesam:discNumber": metadata.discNumber
                }
            } else {
                mpris2Adapter.metadata = {}
            }
        }
    }
}
