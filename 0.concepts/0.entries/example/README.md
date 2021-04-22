# Concept - Entries

## Example

[Creating entries explained](https://holochain-gym.github.io/developers/concepts/entries/)

### Setup - nix-shell
IMPORTANT: You need to run these exercises in the correct nix-shell.
A nix-shell sets up all the right dependencies for the holochain-gym.

In the base folder of this repository, developer-exercises, you will find
a `default.nix` file. Run the following command in your terminal:

```bash
nix-shell
```
The very first time you run this, it will take long time, somewhere between 20 and 80 minutes.
This is because it will download, install and compile everything you need. After that it will only take a second or two to run.
Then you can build with:

### Building

Use this simple script
```
./run_build.sh
```
or if you want to do it manually:

```bash
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown
hc dna pack workdir
```

### Testing

Use this simple script
```
./run_tests.sh
```
or if you want to do it manually:

```bash
cd test
npm install
npm tests
```
