# Links Exercise

[Working with links](https://holochain-gym.github.io/developers/basic/links/)

## To-Do

Implement challenge test for the end of the chapter

---

### Building

Change directory to this folder and run the nix-shell from terminal:

```bash
nix-shell
```

The first time executing this command might take several minutes but will provide you with the latest RSM binaries.
Then you can build with:

```bash
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown
hc dna pack workdir
```

### Testing

Use this simple script
```
./run_tests
```
or if you want to do it manually:

```bash
cd test
npm install
npm tests
```
