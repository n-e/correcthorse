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
    depth=4 \
\
    -engine \
    name=correcthorse-$(git describe --always $REF) \
    cmd=./$TARGET \
    depth=4 \
\
    -each \
        proto=uci \
        tc=1+1 \
    -rounds 100 \
    -sprt elo0=10 elo1=0 alpha=0.05 beta=0.05 \
    -pgnout out.pgn