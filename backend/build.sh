#!/bin/bash

GO_DIR=`pwd`

cd $GO_DIR/../rendering/ && \
cargo build --release

cd $GO_DIR/../frontend/ && \
yarn install && yarn prod

cp -r ./dist  ../backend/ \

cd $GO_DIR 
go build && \
   LISTENING_PORT=8087 LD_LIBRARY_PATH=/home/alvaro/workspace/source/saturno/rendering/target/release ./backend
