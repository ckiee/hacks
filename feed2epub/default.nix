{ stdenv, bun }:
with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "feed2epub-ck";
  version = "git";
  src = ./dist;

  buildInputs = [ bun ];

  installPhase = ''
    echo 'dont forget to $ pnpm dist'
    mkdir -p $out/bin
    (echo "#!${bun}/bin/bun" && cat index.js) > $out/bin/feed2epub-ck
    chmod +x $out/bin/*
  '';
}
