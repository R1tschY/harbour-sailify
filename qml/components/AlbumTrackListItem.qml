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

    property alias name_: titleLabel.text
    property bool playing: false
    property int trackNumber
    property var artists_
    property int durationMs

    Label {
        id: trackNumberLabel

        color: playing ? Theme.highlightColor : Theme.secondaryColor
        text: trackNumber

        width: font.pixelSize * 1.5
        height: Theme.itemSizeMedium
        horizontalAlignment: Text.Right
        verticalAlignment: Text.AlignVCenter
        anchors {
            left: parent.left
            leftMargin: Theme.horizontalPageMargin
        }
    }

    Label {
        id: titleLabel
        truncationMode: TruncationMode.Fade

        color: playing ? Theme.highlightColor : Theme.primaryColor

        anchors {
            left: trackNumberLabel.right
            leftMargin: Theme.paddingSmall
            right: parent.right
            bottom: parent.verticalCenter
        }
    }

    Label {
        id: artistsLabel

        text: SpotifyUtils.joinNames(artists_)
        truncationMode: TruncationMode.Fade
        color: Theme.secondaryColor
        font.pixelSize: Theme.fontSizeSmall

        anchors {
            left: trackNumberLabel.right
            leftMargin: Theme.paddingSmall
            top: parent.verticalCenter
            right: durationLabel.left
        }
    }

    Label {
        id: durationLabel

        text: SpotifyUtils.durationMsToString(durationMs)
        color: Theme.secondaryColor
        font.pixelSize: Theme.fontSizeSmall

        anchors {
            top: parent.verticalCenter
            right: parent.right
            rightMargin: Theme.horizontalPageMargin
        }
    }
}
