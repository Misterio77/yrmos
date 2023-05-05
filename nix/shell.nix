{ pkgs ? import <nixpkgs> { }, ... }:

let
  pgw = pkgs.writeShellScriptBin "pgw" ''
    command="''${1:-none}"
    if [ "$command" == "up" ]; then
      mkdir -p "$PGHOST" "$PGDATA"
      test -f "$PGDATA/PG_VERSION" && \
        echo "Postgres is already initialized." || \
        pg_ctl init
      pg_ctl status &>/dev/null && \
        echo "Postgres is already running." || \
        pg_ctl start -o "-k $PGHOST" -l "$PGLOG"
      psql -c '\q' &>/dev/null && \
        echo "The $PGDATABASE database already exists" || \
        psql postgres -c "CREATE DATABASE $PGDATABASE"
    elif [ "$command" == "down" ]; then
      pg_ctl status &>/dev/null && \
        pg_ctl stop || \
        echo "Postgres is already stopped."
      rm -rf "$PGDATA" && echo "Data deleted."
    else
      echo "Usage: $(basename $0) <up/down>"
    fi
  '';

  motd = pkgs.writeShellScriptBin "motd" ''
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[0;33m'
    BLUE='\033[0;34m'
    PURPLE='\033[0;35m'
    CYAN='\033[0;36m'  
    NC='\033[0m'

    echo -e $YELLOW"Sua shell e a aplicação estão configuradas para um postgres local."$NC
    echo -e "Próximos passos:"
    echo -e "1. Preparar a base:      " $GREEN"pgw up"$NC
    echo -e "2. Executar a aplicação: " $GREEN"cargo wr"$NC
    echo -e ""
    echo -e "Outros comandos:"
    echo -e "- Compilar e executar:   " $YELLOW"cargo"$NC
    echo -e "- Conectar à base:       " $YELLOW"psql"$NC
    echo -e "- Gerenciar a base:      " $YELLOW"pg_ctl"$NC
    echo -e "- Ver essa mensagem:     " $CYAN"motd"$NC
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
    # To generate secret
    libressl

    # Scripts
    motd
    pgw
  ] ++ (oa.nativeBuildInputs or []);

  # Random, but stable secret key while developing
  # To avoid having to log in again all the time
  shellHook = ''
    mkdir -p "$(pwd)/.db"
    export PGDATA="$(pwd)/.db/data"
    export PGHOST="$(pwd)/.db/socket"
    export PGLOG="$(pwd)/.db/log"
    export PGPORT="5430"
    export PGDATABASE="yrmos"
    export PGUSER="$USER"
    export YRMOS_DATABASE="postgres://$PGUSER/$PGDATABASE?host=$PGHOST&port=$PGPORT"
    export YRMOS_SECRET_KEY=$(openssl rand -base64 64)

    motd
  '';
})
