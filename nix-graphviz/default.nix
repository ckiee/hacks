with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "nix-graphviz";
  version = "git";
  src = ./.;
  buildInputs = [ nodejs-16_x ];
  inherit graphviz nix;
  installPhase = ''
    mkdir -p $out/bin
    cp bin.js $out/bin/nix-graphviz
    substituteAllInPlace $out/bin/nix-graphviz
  '';
}
