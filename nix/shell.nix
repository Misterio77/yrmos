{ pkgs ? import <nixpkgs> { }, ... }:

let
  motd = pkgs.writeShellScriptBin "motd" ''
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[0;33m'
    BLUE='\033[0;34m'
    PURPLE='\033[0;35m'
    CYAN='\033[0;36m'  
    NC='\033[0m'

    echo -e $YELLOW"Sua shell e a aplicação estão configuradas para um postgres local."$NC
    echo -e ""
    echo -e "Inicializar o postgres:             " $GREEN"pg_ctl init"$NC
    echo -e "Executar o postgres:                " $GREEN"pg_ctl start -o \"-k \$PGSOCKET\""$NC
    echo -e "Status do postgres:                 " $CYAN"pg_ctl status"$NC
    echo -e "Parar o postgres:                   " $RED"pg_ctl stop"$NC
    echo -e ""
    echo -e "Criar a base:                       " $GREEN"psql postgres -c \"CREATE DATABASE $PGDATABASE\""$NC
    echo -e "Conectar na base:                   " $CYAN"psql"$NC
    echo -e "Apagar a base:                      " $RED"psql -c \"DROP DATABASE $PGDATABASE\""$NC
    echo -e ""
    echo -e "Compilar aplicação:                 " $GREEN"cargo build"$NC
    echo -e "Rodar aplicação:                    " $GREEN"cargo run"$NC
    echo -e "Recompilar e rodar automaticamente: " $YELLOW"cargo watch -x run"$NC
    echo -e ""
    echo -e "Ver essa mensagem novamente:        " $PURPLE"motd"$NC
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
    # Message of the day
    motd
  ] ++ (oa.nativeBuildInputs or []);

  # Random, but stable secret key while developing
  # To avoid having to log in again all the time
  shellHook = ''
      mkdir -p "$(pwd)/.db"
      export PGDATA="$(pwd)/.db/data"
      export PGSOCKET="$(pwd)/.db/socket"
      mkdir -p $PGSOCKET

      export PGPORT="5430"
      export PGDATABASE="yrmos"
      export PGUSER="$USER"
      export PGHOST="$PGSOCKET"

      export YRMOS_DATABASE="postgres://$PGUSER/$PGDATABASE?host=$PGHOST&port=$PGPORT"
      export YRMOS_SECRET_KEY=$(${pkgs.libressl}/bin/openssl rand -base64 64)

      motd
  '';
})
