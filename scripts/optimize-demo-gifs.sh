#!/usr/bin/env sh

# use gifsicle to optimize demo gifs found in demos/ and place them in static/images/demos/
# TODO automatically convert from mp4 too

for f in demos/*.gif; do
    gifsicle -i "$f" --optimize=3 --colors 128 --resize-fit-width 200 -o "static/images/$f";
    echo "optimized $f"
done
