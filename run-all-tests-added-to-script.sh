#!/bin/bash

## basic exercises

# 0.entries
cd basic/0.entries 
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install
FAILED="$(npm test | grep -o 'npm ERR! Test failed.')"
cd ../../.. #back to base folder
if [[ -z $FAILED ]]; then 
    npm test #run tests again to have pretty printed logs to investigate
    exit 1 
fi

# 1.hashes
cd basic/1.hashes
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install
FAILED="$(npm test | grep -o 'npm ERR! Test failed.')"
cd ../../.. #back to base folder
if [[ -z $FAILED ]]; then 
    npm test #run tests again to have pretty printed logs to investigate
    exit 1 
fi
cd ../../..

# 2.headers
cd basic/2.headers
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install
FAILED="$(npm test | grep -o 'npm ERR! Test failed.')"
cd ../../.. #back to base folder
if [[ -z $FAILED ]]; then 
    npm test #run tests again to have pretty printed logs to investigate
    exit 1 
fi
cd ../../..

# 3.elements  --> is just cargo test, no integration test
cd basic/3.elements
cargo test
cd ../.. #only 2 steps to get back to base folder here

# 4.links
cd basic/4.links
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install
FAILED="$(npm test | grep -o 'npm ERR! Test failed.')"
cd ../../.. #back to base folder
if [[ -z $FAILED ]]; then 
    npm test #run tests again to have pretty printed logs to investigate
    exit 1 
fi
cd ../../..

## intermediate exercises

# 1.paths
cd intermediate/1.paths
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install
FAILED="$(npm test | grep -o 'npm ERR! Test failed.')"
cd ../../.. #back to base folder
if [[ -z $FAILED ]]; then 
    npm test #run tests again to have pretty printed logs to investigate
    exit 1 
fi
cd ../../..

# 2.capability-tokens
cd intermediate/2.capability-tokens
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install
FAILED="$(npm test | grep -o 'npm ERR! Test failed.')"
cd ../../.. #back to base folder
if [[ -z $FAILED ]]; then 
    npm test #run tests again to have pretty printed logs to investigate
    exit 1 
fi
cd ../../..

# 3.remote-call
cd intermediate/3.remote-call
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install
FAILED="$(npm test | grep -o 'npm ERR! Test failed.')"
cd ../../.. #back to base folder
if [[ -z $FAILED ]]; then 
    npm test #run tests again to have pretty printed logs to investigate
    exit 1 
fi
cd ../../..