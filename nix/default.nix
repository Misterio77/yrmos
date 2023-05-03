{ lib, rustPlatform, fetchFromGitHub, ... }:

let
  manifest = (lib.importTOML ../Cargo.toml).package;
  # Flake does not support submodule (https://github.com/NixOS/nix/issues/4423)
  # Fetch manually for now
  picocss = fetchFromGitHub {
    owner = "picocss";
    repo = "pico";
    rev = "v1.4.4";
    sha256 = "sha256-k3ovaAa/mC+jO9rgyOZAq8FfwWJkK3uypwxZ6NXvFEo=";
  };
in
rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;

  src =  lib.sourceByRegex ../. [
    "Cargo.toml"
    "Cargo.lock"
    "sqlx-data.json"
    "build.rs"
    "assets.*"
    "db.*"
    "src.*"
  ];

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

  meta = with lib; {
    description = manifest.description;
    homepage = manifest.homepage;
    license = licenses.mit;
    platforms = platforms.all;
  };
}
