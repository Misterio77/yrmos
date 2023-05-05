{ pkgs ? import <nixpkgs> { }, ... }:

let
  db-script = pkgs.writeShellScriptBin "db" ''
    set -e
    operation="''${1:-none}"
    if [ "$operation" == "up" ]; then
      shift
      mkdir -p "$PGDATA" "$PGHOST"
      pg_ctl --silent init
      pg_ctl --silent start -l "$PGLOG" -o "-k $PGHOST"
      psql -c 'CREATE DATABASE yrmos' postgres
      echo "Database created"
      touch "src/main.rs" # Make server reload if watching
    elif [ "$operation" == "down" ]; then
      shift
      pg_ctl --silent status && pg_ctl --silent stop || true
      rm "$PGDATA" "$PGHOST" -rf
      echo "Database destroyed"
    elif [ "$operation" == "logs" ]; then
      less "$PGLOG"
    elif [ "$operation" == "tail" ]; then
      tail "$PGLOG"
    else
      exec -a db pg_ctl "$@"
    fi
  '';
in
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
    db
    # SCSS tooling
    sass
    # Custom DB script wrapper
    db-script
  ] ++ (oa.nativeBuildInputs or []);

  # Random, but stable secret key while developing
  # To avoid having to log in again all the time
  shellHook = ''
      export PGDATA="$(pwd)/.db/data"
      export PGHOST="$(pwd)/.db/socket"
      export PGLOG="$(pwd)/.db/log"

      export PGPORT="5430"
      export PGDATABASE="yrmos"
      export PGUSER="$USER"

      export YRMOS_DATABASE="postgres://$PGUSER/$PGDATABASE?host=$PGHOST&port=$PGPORT"
      export YRMOS_SECRET_KEY=$(${pkgs.libressl}/bin/openssl rand -base64 64)

      echo "Sua shell está configurada para ter uma base local de postgres."
      echo "Use o comando 'db up' para criar e 'db down' para destruir."
      echo "Use 'cargo watch -x run' para executar a aplicação com reloads."
  '';
})
