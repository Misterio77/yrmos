{ pkgs ? import <nixpkgs> { }, ... }:

let
  manifest = (pkgs.lib.importTOML ../Cargo.toml).package;
  # Flake does not support submodule (https://github.com/NixOS/nix/issues/4423)
  # Fetch manually for now
  picocss = pkgs.fetchFromGitHub {
    owner = "picocss";
    repo = "pico";
    rev = "v1.4.4";
    sha256 = "sha256-k3ovaAa/mC+jO9rgyOZAq8FfwWJkK3uypwxZ6NXvFEo=";
  };
  excludeBaseNames = files: (path: _: builtins.all (x: (builtins.baseNameOf path) != x) files);
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = manifest.name;
  version = manifest.version;

  src = builtins.path {
    name = "yrmos-source";
    path = ../.;
    filter = (excludeBaseNames [
      "result"
      "target"
      "nixos.qcow"
      "nix"
      "flake.nix"
      "flake.lock"
      "README.md"
      "LICENSE"
      ".gitattributes"
      ".gitmodules"
      ".gitignore"
    ]);
  };

  cargoLock = {
    lockFile = ../Cargo.lock;
    outputHashes = {
      "maud-0.24.0" = "sha256-q4uLogTGH78GFgQm/tRK2NSo69H6/w6tD4TxUe9dEl4=";
      "chrono-humanize-0.2.1" = "sha256-9J1uGHjSEmJVAp7KceCc9q8G84VQ7MM5VT3mxiR52oQ=";
    };
  };

  preBuild = ''
    rm -df assets/scss/pico
    ln ${picocss} -sfT assets/scss/pico
  '';

  meta = with pkgs.lib; {
    description = manifest.description;
    homepage = manifest.homepage;
    license = licenses.mit;
    platforms = platforms.all;
  };
}
