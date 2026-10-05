#!/bin/bash

set -e

mkdir -p bin

REF=HEAD

TARGET=bin/correcthorse-$(git describe --always $REF)

if [ ! -f "$TARGET" ]; then
    rm -r work || true
    mkdir -p work
    git archive $REF|tar --cd work -xf  -
    (cd work && cargo build -r)
    cp work/target/release/correcthorse $TARGET
fi;

cargo build -r

rm out.pgn || true

cutechess-cli \
    -engine \
    name=correcthorse-$(git describe --always --dirty) \
    cmd=./target/release/correcthorse \
    stderr=/dev/stdout \
\
    -engine \
    name=correcthorse-$(git describe --always $REF) \
    cmd=./$TARGET \
    depth=4 \
\
    -each \
        proto=uci \
        tc=0:5+1 \
    -rounds 100 \
    -concurrency 4 \
    -sprt elo0=0 elo1=100 alpha=0.05 beta=0.05 \
    -pgnout out.pgn