# Holochain Gym - Developer Exercises

These are code exercises for the [developer part of the holochain gym](https://holochain-gym.github.io/developers/).

To get started, clone this repository and follow the guides on the gym to complete the exercises from this repository.

## Install Nix

If you haven't yet, install Nix:

```bash
curl -L https://nixos.org/nix/install | sh
. ~/.nix-profile/etc/profile.d/nix.sh
nix-env -iA cachix -f https://cachix.org/api/v1/install
cachix use holochain-ci
```

## Nix Setup

Go into the root folder of this repository and run:

```bash
nix-shell
npm install
```

After this you have the development environment set up and ready. You can then navigate to each of the exercises to start your workout!
