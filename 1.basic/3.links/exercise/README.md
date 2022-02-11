# Links Exercise

[Working with links](https://holochain-gym.github.io/developers/basic/links/)

## To-Do

Implement challenge test for the end of the chapter

---

### Setup - nix-shell
IMPORTANT: You need to run these exercises in the correct nix-shell.
A nix-shell sets up all the right dependencies for the holochain-gym.

In the base folder of this repository, developer-exercises, you will find
a `default.nix` file. Run the following command in your terminal:

```bash
nix-shell
```
The very first time you run this, it will take a few minutes.
This is because it will download, install and compile everything you need. After that it will only take a second or two to run.
Then you can build with:


### Testing

```bash
cd tests
npm install
npm test
```