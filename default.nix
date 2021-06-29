let 
  holonixPath = builtins.fetchTarball {
    url = "https://github.com/holochain/holonix/archive/014d28000c8ed021eb84000edfe260c22e90af8c.tar.gz";
    sha256 = "sha256:0hl5xxxjg2a6ymr44rf5dfvsb0c33dq4s6vibva6yb76yvl6gwfi";
  };
  holonix = import (holonixPath) {
    includeHolochainBinaries = true;
    holochainVersionId = "custom";
    
    holochainVersion = { 
     rev = "d3a61446acaa64b1732bc0ead5880fbc5f8e3f31";
     sha256 = "0k1fsxg60spx65hhxqa99nkiz34w3qw2q4wspzik1vwpkhk4pwqv";
     cargoSha256 = "sha256:1n4kp19f1in8fbpv6f52ci1imixfmchyy9991fflqnmivb23maf2";
     bins = {
       holochain = "holochain";
       hc = "hc";
     };
    };
    holochainOtherDepsNames = ["lair-keystore"];
  };
in holonix.main
