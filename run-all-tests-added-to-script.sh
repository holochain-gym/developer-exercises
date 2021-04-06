#!/bin/bash

## basic exercises

# 0.entries
cd basic/0.entries 
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install &&
OUTPUT=$(npm test)
FAILED="$(echo "$OUTPUT" | grep -o '# fail  ')" # expected that zero tests fail
cd ../../.. #back to base folder
echo "$OUTPUT" #print output of tests to investigate
if [[ ! -z $FAILED ]]; then 
    echo "TESTS FAILED"
    exit 1 
fi
echo "TESTS PASSED"



# 1.hashes
cd basic/1.hashes
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install &&
OUTPUT=$(npm test)
FAILED="$(echo "$OUTPUT" | grep -o '# fail  ')" # expected that zero tests fail
cd ../../.. #back to base folder
echo "$OUTPUT" #print output of tests to investigate
if [[ ! -z $FAILED ]]; then 
    echo "TESTS FAILED"
    exit 1 
fi
echo "TESTS PASSED"

# 2.headers
cd basic/2.headers
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install &&
OUTPUT=$(npm test)
FAILED="$(echo "$OUTPUT" | grep -o '# fail  ')" # expected that zero tests fail
cd ../../.. #back to base folder
echo "$OUTPUT" #print output of tests to investigate
if [[ ! -z $FAILED ]]; then 
    echo "TESTS FAILED"
    exit 1 
fi
echo "TESTS PASSED"

# 3.elements  --> is just cargo test, no integration test
cd basic/3.elements
cargo test
cd ../.. #only 2 steps to get back to base folder here

# 4.links
cd basic/4.links
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install &&
OUTPUT=$(npm test)
FAILED="$(echo "$OUTPUT" | grep -o '# fail  ')" # expected that zero tests fail
cd ../../.. #back to base folder
echo "$OUTPUT" #print output of tests to investigate
if [[ ! -z $FAILED ]]; then 
    echo "TESTS FAILED"
    exit 1 
fi
echo "TESTS PASSED"

## intermediate exercises

# 1.paths
cd intermediate/1.paths
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install &&
OUTPUT=$(npm test)
FAILED="$(echo "$OUTPUT" | grep -o '# fail  ')" # expected that zero tests fail
cd ../../.. #back to base folder
echo "$OUTPUT" #print output of tests to investigate
if [[ ! -z $FAILED ]]; then 
    echo "TESTS FAILED"
    exit 1 
fi
echo "TESTS PASSED"

# 2.remote-call
cd intermediate/2.remote-call
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install &&
OUTPUT=$(npm test)
FAILED="$(echo "$OUTPUT" | grep -o '# fail  ')" # expected that zero tests fail
cd ../../.. #back to base folder
echo "$OUTPUT" #print output of tests to investigate
if [[ ! -z $FAILED ]]; then 
    echo "TESTS FAILED"
    exit 1 
fi
echo "TESTS PASSED"

# 3.capability-tokens
cd intermediate/3.capability-tokens
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install &&
OUTPUT=$(npm test)
FAILED="$(echo "$OUTPUT" | grep -o '# fail  ')" # expected that zero tests fail
cd ../../.. #back to base folder
echo "$OUTPUT" #print output of tests to investigate
if [[ ! -z $FAILED ]]; then 
    echo "TESTS FAILED"
    exit 1 
fi
echo "TESTS PASSED"
