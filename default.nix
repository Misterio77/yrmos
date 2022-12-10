{ pkgs ? import <nixpkgs> { }, ... }:

let manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = manifest.name;
  version = manifest.version;

  src = pkgs.lib.cleanSource ./.;

  cargoLock.lockFile = ./Cargo.lock;

  postInstall = ''
    mkdir -p $out/etc
    cp -r migrations $out/etc
  '';

  meta = with pkgs.lib; {
    description = manifest.description;
    homepage = manifest.homepage;
    license = licenses.mit;
    platforms = platforms.all;
  };
}
