#!/usr/bin/env bash

mkdir -p stickers-out
for file in stickers-original/*; do
    newfile=stickers-out/$(basename $file)
    echo "converting $file to $newfile"
    if [ -f "$file" ] && [ ! -f "$newfile" ]; then
        magick $file -resize 2048@ -background none -bordercolor none \
            -gravity center -border 20 -write mpr:in -resize 200% \
            -channel A -morphology dilate disk:10 +channel -fill white \
            -colorize 100 -resize 50% mpr:in -composite \
	    -channel A -evaluate multiply 5 $newfile
    fi
done
