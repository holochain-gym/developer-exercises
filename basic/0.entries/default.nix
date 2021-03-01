# This is an example of what downstream consumers of holonix should do
# This is also used to dogfood as many commands as possible for holonix
# For example the release process for holonix uses this file
let

 # point this to your local config.nix file for this project
 # example.config.nix shows and documents a lot of the options
 config = import ./config.nix;

 # START HOLONIX IMPORT BOILERPLATE
 holonix = config.holonix.importFn { };
 # END HOLONIX IMPORT BOILERPLATE

in holonix.pkgs.mkShell {
  inputsFrom = [
    holonix.shell
  ];
  inherit (config) buildInputs;
}
