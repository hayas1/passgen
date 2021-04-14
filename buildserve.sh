#!/bin/sh -e
wasm-pack build --dev --target web --out-name wasm --out-dir ./public
cp ./static/* ./public/
miniserve ./public --index index.html

