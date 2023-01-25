{
  description = "Yrmos";

  nixConfig = {
    extra-substituters = [ "https://cache.m7.rs" ];
    extra-trusted-public-keys = [ "cache.m7.rs:kszZ/NSwE/TjhOcPPQ16IuUiuRSisdiIwhKZCxguaWg=" ];
  };

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
        specialArgs = { inherit (self) outputs; };
        modules = [ nix/vm.nix ];
      };

    };
}

