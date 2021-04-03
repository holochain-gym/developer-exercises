#!/bin/bash
if [[ -z $IN_NIX_SHELL ]]; then
    echo "I looks like you are NOT running in a nix-shell"
    echo "Go to the base folder of the developer-exercises, "
    echo "where you find default.nix, "
    echo "and run 'nix-shell' in the command line."
    exit 1
fi

## in the future add check to see if we are running in the nix-shell for holochain-gym
#
# if [[ -z $IN_NIX_SHELL_GYM ]]; then
#     echo "I looks like you are NOT running in THE RIGHT nix-shell"
#     echo "for the holochain-gym exercises."
#     echo "Go to the base folder of the developer-exercises, "
#     echo "where you find default.nix, "
#     echo "and run 'nix-shell' in the command line."
#     exit 1
# fi

## in the future add check to see if are running an old configuration of the holochain-gym nix-shell
# echo "I looks like you are NOT running in an OLD nix-shell configuration"