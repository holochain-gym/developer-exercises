#!/bin/bash

# basic exercises
cd basic

# 0.entries
cd 0.entries 
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install
npm test
cd ..

# 1.hashes
cd ../1.hashes
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install
npm test
cd ..

# 2.headers
cd ../2.headers
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install
npm test
cd ..

# intermediate exercises
cd ../../intermediate

# 1.paths
cd 1.paths
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install
npm test
cd ..
