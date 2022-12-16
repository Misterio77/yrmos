{
  description = "Yrmos";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11";
  };

  outputs = { self, nixpkgs }:
    let
      forAllSystems = nixpkgs.lib.genAttrs [ "x86_64-linux" ];
      pkgsFor = nixpkgs.legacyPackages;
    in
    rec {
      nixosModules.default = import nix/module.nix;

      overlays.default = final: _prev: {
        yrmos = final.callPackage nix/default.nix { };
      };

      packages = forAllSystems (system: rec {
        default = yrmos;
        yrmos = pkgsFor.${system}.callPackage nix/default.nix { };
        vm = nixosConfigurations.yrmos.config.system.build.vm;
      });

      devShells = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage nix/shell.nix { };
      });

      hydraJobs = packages;

      # Para testes & desenvolvimento
      nixosConfigurations.yrmos = nixpkgs.lib.nixosSystem {
        modules = [
          ({ pkgs, ... }: {
            imports = [ nixosModules.default ];
            nixpkgs = {
              overlays = [ overlays.default ];
              hostPlatform = "x86_64-linux";
            };

            services.yrmos = {
              enable = true;
              openFirewall = true;
            };

            users.users.yrmos = {
              password = "yrmos";
              packages = [ pkgs.yrmos ];
              shell = pkgs.bashInteractive;
              extraGroups = [ "wheel" ];
            };
          })
        ];
      };

      nixConfig = {
        extra-substituers = [ "https://cache.m7.rs" ];
        extra-trusted-public-keys = [ "cache.m7.rs:kszZ/NSwE/TjhOcPPQ16IuUiuRSisdiIwhKZCxguaWg=" ];
      };
    };
}

