{
 # extend the shell with buildInputs specific to this project
 buildInputs = [ ];

 # configure holonix itself
 holonix = {
  
  # true = use a github repository as the holonix base (recommended)
  # false = use a local copy of holonix (useful for debugging)
  use-github = true;

  # configure the remote holonix github when use-github = true
  github = {

   # can be any github ref
   # branch, tag, commit, etc.
   ref = "master";

   # the github owner of the holonix repo
   owner = "holochain";

   # the name of the holonix repo
   repo = "holonix";
  };

  # configuration for when use-github = false
  local = {
   # the path to the local holonix copy
   path = ./.;
  };

 };

 release = {
  commit = "________________________________________";
  version = {
   current = "_._._";
   previous = "_._._";
  };

  hook = {
   preflight = ''
echo "<your preflight script here>"
   '';
   version = ''
echo "<your versioning script here>"
   '';
   publish = ''
echo "<your publishing script here>"
   '';
  };

  github = {
   owner = "<your github owner here>";
   repo = "<your repo name here>";
   template = ''
   {{ changelog }}
   <your release template markdown here>
   '';
  };
 };

 holo-nixpkgs = rec {
  use-github = true;

  github = rec {
   # can be any github ref
   # branch, tag, commit, etc.
   ref = "pr/holochain-bin-group";

   # the github owner of the holonix repo
   owner = "Holo-Host";

   # the name of the holonix repo
   repo = "holo-nixpkgs";

  };

  # configuration for when use-github = false
  local = {
   # the path to the local holonix copy
   path = ../holo-nixpkgs;
  };

  importFn = _: import (
     if use-github
     then builtins.fetchTarball (with github; {
        url = "https://github.com/${owner}/${repo}/archive/${ref}.tar.gz";
         }
       )
     else local.path
    ) {}
    ;
 };
}
