#!/bin/bash

# concepts_entries=0.concepts/0.entries 
# concepts_hashes=0.concepts/1.hashes
# concepts_headers=0.concepts/2.headers
# concepts_source-chain=0.concepts/3.source-chain
# concepts_source-chain=0.concepts/4.dht
# concepts_source-chain=0.concepts/5.validation

basic_entries=1.basic/solutions/0.entries
basic_hashes=1.basic/solutions/1.hashes
basic_links=1.basic/solutions/2.links
basic_headers=1.basic/solutions/3.headers
basic_elements=1.basic/solutions/4.elements
basic_source_chain=1.basic/solutions/5-source-chain

intermediate_paths=2.intermediate/solutions/1.paths
intermediate_remote_call=2.intermediate/solutions/2.remote-call
intermediate_capability_tokens=2.intermediate/solutions/3.capability-tokens

### TODO change script to use ./run_build.sh and ./run_tests.sh to make sure scripts are up to date

## basic exercises
# entries
cd $basic_entries && 
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install &&
OUTPUT=$(npm test)
FAILED="$(echo "$OUTPUT" | grep -o '# fail  ')" # expected that zero tests fail
cd ../../../.. #back to base folder
echo "$OUTPUT" #print output of tests to investigate
if [[ ! -z $FAILED ]]; then 
    echo "TESTS FAILED"
    exit 1 
fi
echo "TESTS PASSED"

# # hashes
cd $basic_hashes && 
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install &&
OUTPUT=$(npm test)
FAILED="$(echo "$OUTPUT" | grep -o '# fail  ')" # expected that zero tests fail
cd ../../../.. #back to base folder
echo "$OUTPUT" #print output of tests to investigate
if [[ ! -z $FAILED ]]; then 
    echo "TESTS FAILED"
    exit 1 
fi
echo "TESTS PASSED"

# headers
cd $basic_headers
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install &&
OUTPUT=$(npm test)
FAILED="$(echo "$OUTPUT" | grep -o '# fail  ')" # expected that zero tests fail
cd ../../../.. #back to base folder
echo "$OUTPUT" #print output of tests to investigate
if [[ ! -z $FAILED ]]; then 
    echo "TESTS FAILED"
    exit 1 
fi
echo "TESTS PASSED"

# elements  --> is just cargo test, no integration test
cd $basic_elements
cargo test
cd ../../.. 

# links
cd $basic_links
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install &&
OUTPUT=$(npm test)
FAILED="$(echo "$OUTPUT" | grep -o '# fail  ')" # expected that zero tests fail
cd ../../../.. #back to base folder
echo "$OUTPUT" #print output of tests to investigate
if [[ ! -z $FAILED ]]; then 
    echo "TESTS FAILED"
    exit 1 
fi
echo "TESTS PASSED"

# source-chain
cd $basic_source_chain
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install &&
OUTPUT=$(npm test)
FAILED="$(echo "$OUTPUT" | grep -o '# fail  ')" # expected that zero tests fail
cd ../../../.. #back to base folder
echo "$OUTPUT" #print output of tests to investigate
if [[ ! -z $FAILED ]]; then 
    echo "TESTS FAILED"
    exit 1 
fi
echo "TESTS PASSED"

## intermediate exercises

# 1.paths
cd $intermediate_paths
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install &&
OUTPUT=$(npm test)
FAILED="$(echo "$OUTPUT" | grep -o '# fail  ')" # expected that zero tests fail
cd ../../../.. #back to base folder
echo "$OUTPUT" #print output of tests to investigate
if [[ ! -z $FAILED ]]; then 
    echo "TESTS FAILED"
    exit 1 
fi
echo "TESTS PASSED"

# 2.remote-call
cd $intermediate_remote_call
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install &&
OUTPUT=$(npm test)
FAILED="$(echo "$OUTPUT" | grep -o '# fail  ')" # expected that zero tests fail
cd ../../../.. #back to base folder
echo "$OUTPUT" #print output of tests to investigate
if [[ ! -z $FAILED ]]; then 
    echo "TESTS FAILED"
    exit 1 
fi
echo "TESTS PASSED"

# 3.capability-tokens
cd $intermediate_capability_tokens
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown 
hc dna pack workdir
cd tests
npm install &&
OUTPUT=$(npm test)
FAILED="$(echo "$OUTPUT" | grep -o '# fail  ')" # expected that zero tests fail
cd ../../../.. #back to base folder
echo "$OUTPUT" #print output of tests to investigate
if [[ ! -z $FAILED ]]; then 
    echo "TESTS FAILED"
    exit 1 
fi
echo "TESTS PASSED"