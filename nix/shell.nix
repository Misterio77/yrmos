{ pkgs ? import <nixpkgs> { }, ... }:

let
  colors = ''
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[0;33m'
    BLUE='\033[0;34m'
    PURPLE='\033[0;35m'
    CYAN='\033[0;36m'  
    NC='\033[0m'
  '';
  yrmos_help = pkgs.writeShellScriptBin "yrmos_help" ''
    ${colors}
    echo -e $PURPLE"Próximos passos:"$NC
    echo -e "1. Iniciar o postgres:   "$GREEN"pg_ctl start"$NC
    echo -e "2. Executar a aplicação: "$GREEN"cargo wr"$NC
    echo -e ""
    echo -e "Outros comandos disponíveis:"
    echo -e "- Build tool:      "$YELLOW"cargo"$NC
    echo -e "- Conectar à base: "$BLUE"psql"$NC
    echo -e "- Gerir a base:    "$BLUE"pg_ctl"$NC
    echo -e ""
    echo -e "- Ver essa mensagem:     "$PURPLE"yrmos_help"$NC
    echo -e "- Inicializar novamente: "$PURPLE"yrmos_init"$NC
  '';
  yrmos_init = pkgs.writeShellScriptBin "yrmos_init" ''
    ${colors}
    echo -e $PURPLE"Inicializando ambiente do Yrmos..."$NC

    # Postgres
    mkdir -p "$PGDATA"
    if [ -z "$(ls -A "$PGDATA")" ]; then
      pg_ctl init -o "-A trust" > /dev/null
      echo "unix_socket_directories = '$PGDATA'" >> "$PGDATA/postgresql.conf"
      echo -e $GREEN"Postgres inicializado."$NC
    else
      echo -e $YELLOW"Postgres já foi inicializado. Apague '$PGDATA' caso queira recomeçar."$NC
    fi

    echo -e $PURPLE"Ambiente inicializado."$NC
    echo -e ""
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

    # Scripts
    yrmos_help
    yrmos_init
  ] ++ (oa.nativeBuildInputs or []);

  shellHook = ''
    export PGCOLOR="always"
    export PGDATA="$(pwd)/.database"

    export PGHOST="$PGDATA"
    export PGPORT="5430"
    export PGDATABASE="postgres"
    export PGUSER="$USER"
    export YRMOS_DATABASE="psql://$PGUSER/$PGDATABASE?host=$PGHOST&port=$PGPORT"
    export YRMOS_SECRET_KEY=$(${pkgs.libressl}/bin/openssl rand -base64 64)

    yrmos_init
    yrmos_help
  '';
})
