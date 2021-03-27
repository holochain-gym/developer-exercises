#!/bin/bash

cd basic
cd 0.entries 
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install
npm test
cd ..

cd ../1.hashes
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install
npm test
cd ..

cd ../2.headers
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install
npm test
cd ..

cd ../3.elements
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install
npm test
cd ..

cd ../../intermediate

cd 1.paths
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install
npm test
cd ..
