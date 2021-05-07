#!/bin/bash

cd 1.basic

for D in *; do
    if [ -d "${D}" ]; then
        echo Testing ${D}
        cd ${D}/solution && CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown && hc dna pack workdir && cd tests && npm install && npm test
        cd ../../..
    fi
done

cd ../2.intermediate

for D in *; do
    if [ -d "${D}" ]; then
        echo Testing ${D}
        cd ${D}/solution && CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown && hc dna pack workdir && cd tests && npm install && npm test
        cd ..
    fi
done
