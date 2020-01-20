#!/bin/bash

GO_DIR=`pwd`

cd $GO_DIR/../rendering/ && \
cargo build --release

// Disabled WasmPackPlugin
cd $GO_DIR/../rendering_wasm/ && \
wasm-pack build

cd $GO_DIR/../frontend/ && \
rm -rf ./node_modules/rendering_wasm
cp -r $GO_DIR/../rendering_wasm/pkg ./node_modules/rendering_wasm
yarn install && yarn prod

cp -r ./dist  ../backend/ \

cd $GO_DIR 
go build && \
   LISTENING_PORT=8087 LD_LIBRARY_PATH=/home/alvaro/workspace/source/saturno/rendering/target/release ./backend
