{ pkgs ? import <nixpkgs> { }, ... }:

pkgs.mkShell {
  inputsFrom = [ (pkgs.callPackage ./default.nix { }) ];
  buildInputs = with pkgs; [
    # Rust tooling
    rustc
    rust-analyzer
    rustfmt
    clippy
    # Postgres tooling
    postgresql
    pgformatter
    sqls
  ];
}
