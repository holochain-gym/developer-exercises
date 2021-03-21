#!/bin/bash

cd basic

for D in *; do
    if [ -d "${D}" ]; then
        cd ${D} && nix-shell . --run "CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown && hc dna pack workdir && cd tests && npm install && npm test"
        cd ..
    fi
done

cd ../intermediate

for D in *; do
    if [ -d "${D}" ]; then
        cd ${D} && nix-shell . --run "CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown && hc dna pack workdir && cd tests && npm install && npm test"
        cd ..
    fi
done