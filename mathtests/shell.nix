{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell {
  buildInputs = [ libglvnd xorg.libXinerama xorg.libXext xorg.libX11 ];
}
