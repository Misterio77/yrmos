{ config, lib, pkgs, ... }:

with lib;
let cfg = config.services.yrmos;
in
{
  options.services.yrmos = {
    enable = mkEnableOption "yrmos";
    package = mkOption {
      type = types.package;
      default = pkgs.callPackage ./default.nix { };
      description = ''
        The package implementing yrmos
      '';
    };
    address = mkOption {
      type = types.str;
      default = "0.0.0.0";
      description = "Address to bind to.";
    };
    port = mkOption {
      type = types.int;
      default = 8080;
      description = "Port number to bind to.";
    };
    database = {
      host = mkOption {
        type = types.str;
        default = "localhost";
        description = "Database host address, if using TCP.";
      };

      port = mkOption {
        type = types.port;
        default = config.services.postgresql.port;
        description = "Database host port, if using TCP.";
      };

      name = mkOption {
        type = types.str;
        default = cfg.user;
        description = "Database name.";
      };

      user = mkOption {
        type = types.str;
        default = cfg.user;
        description = "Database user.";
      };

      password = mkOption {
        type = types.str;
        default = "";
        description = "Database password, if using TCP. Stored in cleartext in the store!";
      };

      passwordFile = mkOption {
        type = types.nullOr types.path;
        default = null;
        description = "File containing database password, if using TCP.";
      };

      socket = mkOption {
        type = types.nullOr types.path;
        default = if cfg.database.createLocally then "/run/postgresql" else null;
        description = "Path to the unix socket file to use for authentication. Set to null to use TCP instead.";
      };

      createLocally = mkOption {
        type = types.bool;
        default = true;
        description = "Whether to create a local database automatically.";
      };
    };
    environmentFile = mkOption {
      type = types.nullOr types.path;
      description = "File path containing extra environment variables (cookie secret key, for example) for the server";
      default = null;
    };
    openFirewall = mkOption {
      type = types.bool;
      default = false;
      description = "Whether to open port in the firewall for the server.";
    };
    user = mkOption {
      type = types.str;
      default = "yrmos";
      description = "Service user that will run the daemon.";
    };
    logLevel = mkOption {
      type = types.enum [ "debug" "info" "warn" "error" ];
      default = "warn";
      description = "Log verbosity.";
    };
  };

  config = mkIf cfg.enable {
    assertions = [
      {
        assertion = cfg.database.createLocally -> cfg.database.user == cfg.user;
        message = "If using services.yrmos.database.createLocally, the database and service user must match.";
      }
      {
        assertion = cfg.database.createLocally -> cfg.database.socket != null;
        message = "If using services.yrmos.database.createLocally, the socket path must be non-null.";
      }
    ];

    # If we're to create a local postgres db
    services.postgresql = optionalAttrs cfg.database.createLocally {
      enable = mkDefault true;
      ensureDatabases = [ cfg.database.name ];
      ensureUsers = [{
        name = cfg.database.user;
        ensurePermissions = {
          "DATABASE ${cfg.database.name}" = "ALL PRIVILEGES";
        };
      }];
    };

    systemd.services.yrmos = let
      envFile = "/tmp/yrmos.env";

      dbPassword = if cfg.database.passwordFile != null
      then "$(cat ${cfg.database.passwordFile})"
      else cfg.database.password;

      connectionString = if cfg.database.socket != null
      then "postgres://${cfg.database.user}/${cfg.database.name}?host=${cfg.database.socket}"
      else "postgres://${cfg.database.user}:${dbPassword}@${cfg.database.host}:${toString cfg.database.port}/${cfg.database.name}";
    in {
      description = "yrmos";
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ] ++ optional cfg.database.createLocally "postgresql.service";
      preStart = ''
        cat > "${envFile}" <<EOF
        YRMOS_DATABASE="${connectionString}"
        EOF
        chmod 700 "${envFile}"
      '';
      serviceConfig = {
        ExecStart = "${cfg.package}/bin/yrmos";
        Restart = "always";
        User = cfg.user;
        EnvironmentFile = [ cfg.environmentFile "-${envFile}" ];
      };
      environment = {
        YRMOS_ADDRESS = cfg.address;
        YRMOS_PORT = toString cfg.port;
        YRMOS_LOG_LEVEL = cfg.logLevel;
      };
    };

    users = {
      users.${cfg.user} = {
        description = "yrmos service user";
        isSystemUser = true;
        group = cfg.user;
      };
      groups.${cfg.user} = { };
    };

    networking.firewall =
      mkIf cfg.openFirewall { allowedTCPPorts = [ cfg.port ]; };
  };
}
