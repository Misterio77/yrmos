{ pkgs ? import <nixpkgs> { } }:

let
  gems = pkgs.bundlerEnv {
    name = "website-env";
    inherit (pkgs) ruby;
    gemfile = ./Gemfile;
    lockfile = ./Gemfile.lock;
    gemset = ./gemset.nix;
  };
in
pkgs.stdenv.mkDerivation {
  name = "yrmos";
  src = ./.;

  JEKYLL_ENV = "production";

  buildInputs = [ gems pkgs.ruby ];

  buildPhase = ''
    ${gems}/bin/bundle exec jekyll build
  '';

  installPhase = ''
    mkdir -p $out
    cp -Tr _site $out/public
  '';
}
