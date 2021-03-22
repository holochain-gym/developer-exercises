# 2. Capability Tokens Exercise

[Problem statement](https://holochain-gym.github.io/developers/intermediate/capability-tokens/)

## Exercise

- Open your IDE in [the zome](/intermediate/2.capability-tokens/zomes/exercise).
- Code all `unimplemented!()` functions.
- Compile and run the tests, and check that all tests are passing.

---

### Building

First, enter the nix-shell in running this command in this folder:

```bash
nix-shell
```

This will take some time, but will provide you with the latest RSM binaries. Then you can build with:

```bash
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown
hc dna pack workdir
```

### Testing

```bash
cd test
npm install
npm test
```
