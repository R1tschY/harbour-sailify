import QtQuick 2.0
import Sailfish.Silica 1.0
import Sailify 0.1

Page {
    id: page

    Component.onCompleted: {
        switch (librespot.connectionStatus) {
        case SailifyPlayer.Disconnected:
        case SailifyPlayer.Crashed:
            librespot.start();
            break;
        case SailifyPlayer.Connecting:
            progressLabel.text = qsTr("Connecting …")
            break;
        case SailifyPlayer.Connected:
            onComplete()
            break;
        }
    }

    Connections {
        target: librespot

        onConnectionStatusChanged: onComplete()
        onErrorOccurred: onComplete()
    }

    Timer {
        id: changeTimer

        interval: 50
        repeat: false
        running: false

        onTriggered: {
            switch (librespot.connectionStatus) {
            case SailifyPlayer.Disconnected:
            case SailifyPlayer.Crashed:
                onError()
                break;
            case SailifyPlayer.Connecting:
                progressLabel.text = qsTr("Connecting …")
                break;
            case SailifyPlayer.Connected:
                pageStack.replaceAbove(null, Qt.resolvedUrl("MainNavigationPage.qml"))
                break;
            }
        }
    }

    function onComplete() {
        pageStack.completeAnimation()
        changeTimer.start()
    }

    function onError(error) {
        pageStack.completeAnimation()
        if (librespot.errorKind === SailifyPlayer.MissingCredentials) {
            pageStack.replace(Qt.resolvedUrl("LoginPage.qml"), {}, PageStackAction.Immediate)
        } else {
            pageStack.replace(Qt.resolvedUrl("LoginErrorPage.qml"), { allowLogout: false }, PageStackAction.Immediate)
        }
    }

    PageBusyIndicator {
        id: busyIndicator
        running: true
        anchors {
            verticalCenter: parent.verticalCenter
            horizontalCenter: parent.horizontalCenter
        }
    }

    Label {
        id: progressLabel
        width: page.width
        text: qsTr("Loading …")
        anchors.top: busyIndicator.bottom
        anchors.topMargin: Theme.paddingMedium
        horizontalAlignment: Text.AlignHCenter
        wrapMode: Text.WordWrap
        color: Theme.secondaryHighlightColor
        font.pixelSize: Theme.fontSizeLarge
    }
}
