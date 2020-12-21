#!/bin/bash -ex

SVG=$1
DIR=`dirname "$SVG"`
NAME=`basename "$SVG"`
NAME=${NAME%.*}

for SIZE in 86 108 128 172
do
    mkdir -p "$DIR/${SIZE}x${SIZE}"
    inkscape -w $SIZE -h $SIZE "$SVG" -e "$DIR/${SIZE}x${SIZE}/$NAME.png"
    optipng "$DIR/${SIZE}x${SIZE}/$NAME.png"
done
