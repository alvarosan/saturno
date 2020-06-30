#!/bin/bash

GO_DIR=`pwd`

cd $GO_DIR/../rendering_wasm/ && \
wasm-pack build

cd $GO_DIR/../frontend/
rm -rf ./node_modules/rendering_wasm
cp -r $GO_DIR/../rendering_wasm/pkg ./node_modules/rendering_wasm
npm install && npm run dev
