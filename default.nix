
# This is an example of what downstream consumers of holonix should do
# This is also used to dogfood as many commands as possible for holonix
# For example the release process for holonix uses this file
let

 # START HOLONIX IMPORT BOILERPLATE
  holonixPath = builtins.fetchTarball {
    url = "https://github.com/holochain/holonix/archive/90a19d5771c069dbcc9b27938009486b54b12fb7.tar.gz";
    sha256 = "11wv7mwliqj38jh1gda3gd0ad0vqz1d42hxnhjmqdp037gcd2cjg";
  };
  holonix = import (holonixPath) {
    includeHolochainBinaries = true;
    holochainVersionId = "custom";
    
    holochainVersion = { 
     rev = "d3a61446acaa64b1732bc0ead5880fbc5f8e3f31";
     sha256 = "0k1fsxg60spx65hhxqa99nkiz34w3qw2q4wspzik1vwpkhk4pwqv";
     cargoSha256 = "0fz7ymyk7g3jk4lv1zh6gbn00ad7wsyma5r7csa88myl5xd14y68";
     bins = {
       holochain = "holochain";
       hc = "hc";
     };
    };
  };
 # END HOLONIX IMPORT BOILERPLATE

in
with holonix.pkgs;
{
 dev-shell = stdenv.mkDerivation (holonix.shell // {
  name = "dev-shell";

  buildInputs = [ ]
   ++ holonix.shell.buildInputs
  ;
 });
}
