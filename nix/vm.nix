{ pkgs, outputs, ... }: {
  imports = [ outputs.nixosModules.default ];
  nixpkgs = {
    overlays = [ outputs.overlays.default ];
    hostPlatform = "x86_64-linux";
  };

  services.yrmos = {
    enable = true;
    openFirewall = true;
  };

  users.users.nixos = {
    isNormalUser = true;
    extraGroups = [ "wheel" ];
    initialHashedPassword = "";
  };
  services.getty.autologinUser = "nixos";
  security.sudo.wheelNeedsPassword = false;
  services.openssh.enable = true;
}
