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

import QtQuick 2.6
import Sailfish.Silica 1.0
import QtGraphicalEffects 1.0
import "../spotifyUtils.js" as SpotifyUtils

ListItem {
    id: root
    width: parent.width
    contentHeight: Theme.itemSizeMedium

    property alias name_: _title.text
    property var images_
    property bool playing: false
    property string fallbackIcon: "image://theme/icon-m-music"

    readonly property string _image: SpotifyUtils.chooseImage(images_, Theme.itemSizeMedium)
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

        Image {
            id: image

            anchors.centerIn: parent
            width: _imageSize
            height: _imageSize

            asynchronous: true
            sourceSize.width: _imageSize
            sourceSize.height: _imageSize
            opacity: image.status === Image.Ready ? 1 : 0
            visible: opacity > 0
            source: _fallback ? fallbackIcon : _image
            fillMode: Image.PreserveAspectCrop

            layer {
                enabled: root.highlighted
                effect: PressEffect { source: image }
            }

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

        color: playing ? Theme.highlightColor : Theme.primaryColor

        anchors {
            left: imageBox.right
            leftMargin: Theme.paddingLarge
            right: parent.right
            verticalCenter: parent.verticalCenter
        }
    }
}
