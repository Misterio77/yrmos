{ pkgs ? import <nixpkgs> { }, ... }:

(pkgs.callPackage ./default.nix { }).overrideAttrs (oa: {
  nativeBuildInputs = with pkgs; [
    # Rust tooling
    rustc
    rust-analyzer
    rustfmt
    clippy
    sqlx-cli
    cargo-watch
    # Postgres tooling
    postgresql
    pgformatter
    sqls
    # SCSS tooling
    sass
  ] ++ oa.nativeBuildInputs;
})
