import QtQuick 2.0
import Sailfish.Silica 1.0

Page {
    id: page

    Component.onCompleted: {
        switch (librespot.connectionStatus) {
        case 0: // disconnected
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
        interval: 500
        onTriggered: {
            switch (librespot.connectionStatus) {
            case 0: // disconnected
                onError(librespot.error || "Unknown")
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

        if (pageStack.depth > 1) {
            pageStack.replace(Qt.resolvedUrl("ErrorPage.qml"), {
                "header": qsTr("Login Error"),
                "text": error
            }, PageStackAction.Immediate)
        } else {
            var pages = [{
                "page": Qt.resolvedUrl("LoginPage.qml"),
            }]

            if (librespot.errorKind !== "MissingCredentials") {
                pages.push({
                    "page": Qt.resolvedUrl("ErrorPage.qml"),
                    "properties": {
                        "header": qsTr("Login Error"),
                        "text": error
                    }
                })
            }
            pageStack.replaceAbove(null, pages, {}, PageStackAction.Immediate)
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
