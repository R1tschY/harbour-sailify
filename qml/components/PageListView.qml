import QtQuick 2.0
import Sailfish.Silica 1.0
import "api"


SilicaListView {
    property string fallbackIcon: "image://theme/icon-m-music"
    property alias placeholder: viewPlaceholder.text
    property alias fetchBatchSize: listModel.limit

    property bool _complete: false

    id: root
    anchors.fill: parent

    header: PageHeader {
        title: name
    }

    footer: Item {
        width: page.width
        height: visible ? busyIndicator.implicitHeight + Theme.paddingSmall * 2 : 0
        visible: listView.count > 0 && !listModel.completlyFetched

        BusyIndicator {
            id: busyIndicator
            anchors.centerIn: parent

            size: BusyIndicatorSize.Medium

            color: Theme.secondaryHighlightColor
            running: parent.visible
        }
    }

    model: SpotifyWebApiListModel {
        id: listModel
        accessToken: librespot.token
    }

    function ensureContent() {
        if (_complete
                && !listModel.completlyFetched
                && contentHeight - contentY - height < 1000) {
            listModel.fetchNext()
        }
    }

    onContentYChanged: ensureContent()
    onContentHeightChanged: ensureContent()
    onAtYEndChanged: {
        if (atYEnd) {
            ensureContent()
        }
    }

    Component.onCompleted: {
        _complete = true
        ensureContent()
    }

    PageBusyIndicator {
        running: listModel.busy
    }

    ViewPlaceholder {
        id: viewPlaceholder
        enabled: !listModel.busy && !listModel.errorType && listView.count === 0
        text: qsTr("No items found")
    }

    VerticalScrollDecorator { flickable: root }
}
