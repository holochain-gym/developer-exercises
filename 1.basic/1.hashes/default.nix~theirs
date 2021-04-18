let 
  holonixPath = builtins.fetchTarball {
    url = "https://github.com/holochain/holonix/archive/90a19d5771c069dbcc9b27938009486b54b12fb7.tar.gz";
    sha256 = "11wv7mwliqj38jh1gda3gd0ad0vqz1d42hxnhjmqdp037gcd2cjg";
  };
  holonix = import (holonixPath) {
    includeHolochainBinaries = true;
    holochainVersionId = "custom";
    
    holochainVersion = { 
     rev = "9a7219edf355a2028d38f1c1303dd5c9a76bd886";
     sha256 = "17ij6i4jw4lqqlrrfcsf33g8a12bmhfhyqy5yzz7y2i61y8sllx4";
     cargoSha256 = "19faydkxid1d2s0k4jks6y6plgchdhidcckacrcs841my6dvy131";
     bins = {
       holochain = "holochain";
       hc = "hc";
     };
    };
  };
in holonix.main
