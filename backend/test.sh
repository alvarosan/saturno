#!/bin/bash

GO_DIR=`pwd`

cd $GO_DIR/server && \
LD_LIBRARY_PATH=/home/alvaro/workspace/source/saturno/rendering/target/release go test

cd $GO_DIR
