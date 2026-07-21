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
cp -r "icons" "$DEST"
cp -r "pkg" "$DEST"
rm -f "$DEST/pkg/.gitignore"
cp index.html "$DEST"
cp main.css "$DEST"