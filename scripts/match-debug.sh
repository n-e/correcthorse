#!/bin/bash

cargo build -r

rm out.pgn

cutechess-cli \
    -engine \
    "name=correcthorse" \
    cmd=./target/release/correcthorse \
    stderr=/dev/stdout \
\
    -engine \
    "name=correcthorse" \
    cmd=./target/release/correcthorse \
\
    -each \
        proto=uci \
        tc=0:5+1 \
    -rounds 1 \
    -debug \
    -pgnout out.pgn