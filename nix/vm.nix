{ pkgs, outputs, hostPlatform ? "x86_64-linux", ... }: {
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
        { host.port = 8080; guest.port = 8080; }
      ];
    };
    users = {
      motd = ''
        +--- --- --- --- --- --- --- --- --- --- --- --- --- ---+
        | Bem vindo a VM do Yrmos!                              |
        | O sistema está disponível em http://localhost:8080    |
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
