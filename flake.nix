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
      forAllSystems = nixpkgs.lib.genAttrs [ "x86_64-linux" "aarch64-linux" ];
      pkgsFor = nixpkgs.legacyPackages;
      vmFor = forAllSystems (hostPlatform:
        nixpkgs.lib.nixosSystem {
          specialArgs = { inherit hostPlatform; inherit (self) outputs; };
          modules = [ nix/vm.nix ];
      });
    in
    rec {
      nixosModules.default = import nix/module.nix;

      overlays.default = final: _prev: {
        yrmos = final.callPackage nix/default.nix { };
      };

      packages = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage nix/default.nix { };
        vm = vmFor.${system}.config.system.build.vm;
      });

      devShells = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage nix/shell.nix { };
      });

      hydraJobs = packages;

      # Para testes & desenvolvimento
      nixosConfigurations.yrmos = vmFor."x86_64-linux";

    };
}

