#!/bin/bash

## CHECK NIX-SHELL 

# check if we are running in the nix-shell defined for this exercises
./../../../check_running_in_gym_nix_shell.sh
# check result of script and exit when not in right nix-shell
[ $? -eq 0 ] || exit 1


# show fancy text in terminal if we are running in the right nix-shell
echo " ****HOLOCHAIN-GYM BUILDING ZOME ****"

## THE ACTUAL COMMANDS 

# compile your rust code into a wasm binary
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown
