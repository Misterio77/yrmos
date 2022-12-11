{ pkgs ? import <nixpkgs> { }, ... }:

let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
  # Flake does not support submodule (https://github.com/NixOS/nix/issues/4423)
  # Fetch manually for now
  picocss = pkgs.fetchFromGitHub {
    owner = "picocss";
    repo = "pico";
    rev = "ff30e814ec345de49fd2ef12ac392a4a238b764c";
    sha256 = "sha256-3APZCRY9vOwK0vMpMkFAUyD9wobOX6+PlR45e+uY0EE=";
  };
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = manifest.name;
  version = manifest.version;

  src = pkgs.lib.cleanSource ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
    outputHashes = {
      "maud-0.24.0" = "sha256-q4uLogTGH78GFgQm/tRK2NSo69H6/w6tD4TxUe9dEl4=";
    };
  };

  preBuild = ''
    ln ${picocss} -sf scss/pico
  '';

  postInstall = ''
    mkdir -p $out/etc
    cp -r db $out/etc
  '';

  meta = with pkgs.lib; {
    description = manifest.description;
    homepage = manifest.homepage;
    license = licenses.mit;
    platforms = platforms.all;
  };
}
