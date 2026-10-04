#!/bin/bash

cargo build -r

rm out.pgn

cutechess-cli \
    -engine \
    name=jsengine-depth1 \
    cmd=./target/release/correcthorse \
    stderr=/dev/stdout \
    depth=1 \
\
    -engine \
    name=jsengine-depth2 \
    cmd=./target/release/correcthorse \
    depth=1 \
\
    -each \
        proto=uci \
        tc=1+1 \
    -rounds 1 \
    -debug \
    -pgnout out.pgn