{
  description = "Smart2get website";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-22.11";
  };

  outputs = { self, nixpkgs, nix-colors }:
    let
      forAllSystems = nixpkgs.lib.genAttrs [ "x86_64-linux" ];
      pkgsFor = nixpkgs.legacyPackages;
    in
    rec {
      packages = forAllSystems (system: rec {
        default = site;
        site = pkgsFor.${system}.callPackage ./. { };
        serve = pkgsFor.${system}.writeShellScriptBin "serve" ''
          echo "Serving on http://localhost:4000"
          ${pkgsFor.${system}.webfs}/bin/webfsd -p 4000 -F -f index.html -r ${site}/public
        '';
      });

      apps = forAllSystems (system: {
        default = {
          type = "app";
          program = "${packages.${system}.serve}/bin/serve";
        };
      });
    };
}
