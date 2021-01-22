# 1. Paths Exercise

[Problem statement](https://holochain-gym.github.io/developers/intermediate/paths/)

## Exercise

- Open your IDE in [the zome](/intermediate/1.paths/zomes/paths_exercise).
- Code all `unimplemented!()` functions.
- Compile and run the tests, and check that all tests are passing.

### Requirements

- Having run through [holochain RSM installation](https://github.com/holochain/holochain-dna-build-tutorial), with `holochain` and `dna-util` already installed

### Building

```bash
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown
dna-util -c paths_exercise.dna.workdir/
```

### Testing

After having built the DNA:

```bash
cd test
npm install
npm test
```
