# Hash Exercise

[Creating entries explained](https://holochain-gym.github.io/developers/basic/1.hashes/)

## Exercise

- Open your IDE in [the zome](/basic/1.hashes/zomes/exercise).
- Add everything to create entries. Read the [explanation](https://holochain-gym.github.io/developers/basic/hashes/) for this.
- Code all `unimplemented!()` functions.
- Compile and run the tests, and check that the test passes.

---

### Building

First, enter the nix-shell in running this command in this folder:

```bash
nix-shell
```

The first time this will take several minutes or longer, but will provide you with the latest RSM binaries. Then you can build with:

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
