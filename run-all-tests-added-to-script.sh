#!/bin/bash

dir_entries=basic/0.entries 
dir_hashes=basic/1.hashes
dir_links=basic/2.links
dir_headers=basic/3.headers
dir_elements=basic/4.elements
dir_source-chain=basic/5.source-chain

dir_paths=intermediate/1.paths
dir_remote-call=intermediate/2.remote-call
dir_capability-tokens=intermediate/3.capability-tokens

## basic exercises
# entries
cd $dir_entries 
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

# hashes
cd $dir_hashes
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

# headers
cd $dir_headers
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

# elements  --> is just cargo test, no integration test
cd $dir_elements
cargo test
cd ../.. #only 2 steps to get back to base folder here

# links
cd $dir_links
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

# source-chain
cd $dir_source-chain
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
cd $dir_paths
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
cd $dir_remote-call
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
cd $dir_capability-tokens
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