# !/bin/bash

## CHECK NIX-SHELL 

# check if we are running in the nix-shell defined for this exercises
./../../../check_running_in_gym_nix_shell.sh
# check result of script and exit when not in right nix-shell
[ $? -eq 0 ] || exit 1


# show fancy text in terminal if we are running in the right nix-shell
echo " ****HOLOCHAIN-GYM RUNNING TESTS ****"

## THE ACTUAL COMMANDS 

# package your compiled wasm in the holochain dna format
hc dna pack workdir
# jump in test dir
cd tests/ 
# doublecheck if all test dependencies are installed
npm install
# run tests and save output
OUTPUT=$(npm test)
# jump back to base dir of exercise
cd ..
# print output
echo "$OUTPUT"
