#!/bin/bash

BACKEND_DIR=`pwd`/backend

cd $BACKEND_DIR

go build && \
   LISTENING_PORT=8087 \
   LD_LIBRARY_PATH=/home/alvaro/workspace/source/external/saturno/rendering/target/release \
   ./backend
