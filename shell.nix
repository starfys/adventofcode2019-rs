let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
in
{ pkgs ? import <nixpkgs> { overlays = [ moz_overlay ]; }}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    # Specific rust branch
    latest.rustChannels.nightly.rust
    # For serenity
    pkgconfig
    openssl
  ];
}

