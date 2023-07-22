{ pkgs, outputs, hostPlatform ? "x86_64-linux", hostPort ? 8080, ... }: {
  imports = [ outputs.nixosModules.default ];
  nixpkgs = {
    inherit hostPlatform;
    overlays = [ outputs.overlays.default ];
  };

  services.yrmos = {
    enable = true;
    port = 8080;
    openFirewall = true;
  };

  # Configurações da VM
  virtualisation.vmVariant = {
    virtualisation = {
      graphics = false;
      forwardPorts = [
        { host.port = hostPort; guest.port = 8080; }
      ];
    };
    users = {
      motd = let p = toString hostPort; in ''
        +--- --- --- --- --- --- --- --- --- --- --- --- --- ---+
        | Bem vindo a VM do Yrmos!                              |
        | O sistema está disponível em http://localhost:${p}    |
        | Para sair, desligue a VM ou aperte Ctrl+A X           |
        +--- --- --- --- --- --- --- --- --- --- --- --- --- ---+
      '';
      users.yrmos = {
        extraGroups = [ "wheel" ];
        useDefaultShell = true;
      };
    };
    services.getty.autologinUser = "yrmos";
    security.sudo.wheelNeedsPassword = false;
  };
}
