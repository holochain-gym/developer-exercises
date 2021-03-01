{
 # extend the shell with buildInputs specific to this project
 buildInputs = [ ];

 # configure holonix itself
 holonix = rec {

  # true = use a github repository as the holonix base (recommended)
  # false = use a local copy of holonix (useful for debugging)
  use-github = true;

  # configure the remote holonix github when use-github = true
  github = {

   # can be any github ref
   # branch, tag, commit, etc.
   ref = "pr/bump-holo-nixpkgs-add-hc";

   # the github owner of the holonix repo
   owner = "holochain";

   # the name of the holonix repo
   repo = "holonix";
  };

  # configuration for when use-github = false
  local = {
   # the path to the local holonix copy
   path = ../holonix;
  };

  config = {
    includeHolochainBinaries = true;
  };

  importFn = args: import (
     if use-github
     then builtins.fetchTarball (with github; {
        url = "https://github.com/${owner}/${repo}/archive/${ref}.tar.gz";
       })
     else local.path
     ) config // args;
 };
}
