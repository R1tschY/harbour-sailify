/*
 * Copyright 2019 Richard Liebscher <richard.liebscher@gmail.com>.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

import QtQuick 2.0
import Sailfish.Silica 1.0

BackgroundItem {
    id: root
    width: parent.width
    height: Theme.itemSizeMedium

    property var images
    property string fallbackIcon: "image://theme/icon-m-music"
    property alias name: _title.text

    readonly property string _image: {
        if (images && images.length > 0) {
            return images[images.length - 1].url
        } else {
            return ""
        }
    }
    readonly property bool _fallback: !_image || image.status === Image.Error
    readonly property var _imageSize: _fallback ? undefined : Theme.itemSizeMedium

    Rectangle {
        id: imageBox
        height: Theme.itemSizeMedium
        width: Theme.itemSizeMedium
        gradient: Gradient {
            GradientStop {
                position: 0.0
                color: Theme.rgba(Theme.primaryColor, 0.1)
            }
            GradientStop {
                position: 1.0
                color: Theme.rgba(Theme.primaryColor, 0.05)
            }
        }

        HighlightImage {
            id: image

            anchors.centerIn: parent
            width: _imageSize
            height: _imageSize

            highlighted: root.highlighted
            asynchronous: true
            sourceSize.width: _imageSize
            sourceSize.height: _imageSize
            opacity: image.status === Image.Ready ? 1 : 0
            visible: opacity > 0
            source: _fallback ? fallbackIcon : _image

            // TODO: Animate opacity
            // TODO: fallback on error
        }

        BusyIndicator {
            size: BusyIndicatorSize.Small
            anchors.centerIn: parent
            running: image.status === Image.Loading
        }
    }

    Label {
        id: _title
        truncationMode: TruncationMode.Fade

        anchors {
            left: imageBox.right
            leftMargin: Theme.paddingLarge
            right: parent.right
            verticalCenter: parent.verticalCenter
        }
    }
}
