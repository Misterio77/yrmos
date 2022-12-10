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
    database = mkOption {
      type = types.nullOr types.str;
      description = "Connection string for database.";
      default = "postgres:///yrmos?user=yrmos&host=/var/run/postgresql";
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
    environmentFile = mkOption {
      type = types.nullOr types.path;
      description = "File path containing environment variables (secret key, for example) for the server";
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
  };

  config = mkIf cfg.enable {
    systemd.services.yrmos = {
      description = "yrmos";
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ];
      serviceConfig = {
        ExecStart = "${cfg.package}/bin/yrmos";
        Restart = "always";
        User = cfg.user;
        EnvironmentFile = cfg.environmentFile;
      };
      environment = {
        YRMOS_ADDRESS = cfg.address;
        YRMOS_PORT = toString cfg.port;
        YRMOS_DATABASE = cfg.database;
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
