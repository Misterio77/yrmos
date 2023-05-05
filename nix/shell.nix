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

  # Random, but stable secret key while developing
  # To avoid having to log in again all the time
  shellHook = ''
      export YRMOS_SECRET_KEY="$(${pkgs.libressl}/bin/openssl rand -base64 64)"
      echo "Use 'cargo watch -x run' para executar com reloads automáticos."
  '';
})
