#!/bin/bash

if [[ $# -ne 1 ]]; then
    echo "pass destination"
    exit 1;
fi

DEST="$1"

wasm-pack build --target web

if [[ $? -ne 0 ]]; then
    exit 1;
fi

echo "Copying files"
rm -rf "$DEST/montana-solitaire"
mkdir "$DEST/montana-solitaire"
cp -r "icons" "$DEST/montana-solitaire/icons"
cp -r "pkg" "$DEST/montana-solitaire/pkg"
rm -f "$DEST/montana-solitaire/pkg/.gitignore"
cp index.html "$DEST/montana-solitaire"
cp main.css "$DEST/montana-solitaire"