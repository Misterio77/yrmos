{
  description = "Yrmos";
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-22.11";
  outputs = { self, nixpkgs }:
    let
      forAllSystems = nixpkgs.lib.genAttrs [ "x86_64-linux" ];
      pkgsFor = nixpkgs.legacyPackages;
    in
    rec {
      packages = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./. { };
      });
      hydraJobs = packages;
    };
}
