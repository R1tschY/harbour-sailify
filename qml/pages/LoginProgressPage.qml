import QtQuick 2.0
import Sailfish.Silica 1.0

Page {
    id: page

    Component.onCompleted: {
        switch (librespot.connectionStatus) {
        case 0: // disconnected
        case 100: // panic
            librespot.login();
            break;
        case 1: // connecting
            progressLabel.text = qsTr("Logging in …")
            break;
        case 2: // connected
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
            case 0: // disconnected
            case 100:
                onError()
                break;
            case 1: // connecting
                progressLabel.text = qsTr("Logging in …")
                break;
            case 2: // connected
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
        if (librespot.errorKind === "missing-credentials") {
            pageStack.replace(Qt.resolvedUrl("LoginPage.qml"), {}, PageStackAction.Immediate)
        } else {
            pageStack.replace(Qt.resolvedUrl("LoginErrorPage.qml"), {}, PageStackAction.Immediate)
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
        text: qsTr("Preparing …")
        anchors.top: busyIndicator.bottom
        anchors.topMargin: Theme.paddingMedium
        horizontalAlignment: Text.AlignHCenter
        wrapMode: Text.WordWrap
        color: Theme.secondaryHighlightColor
        font.pixelSize: Theme.fontSizeLarge
    }
}
