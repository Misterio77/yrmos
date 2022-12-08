{ stdenv, ruby, bundlerEnv }:

let
  gems = bundlerEnv {
    name = "website-env";
    inherit ruby;
    gemfile = ./Gemfile;
    lockfile = ./Gemfile.lock;
    gemset = ./gemset.nix;
  };
in
stdenv.mkDerivation {
  name = "yrmos";
  src = ./.;

  JEKYLL_ENV = "production";

  buildInputs = [ gems ruby ];

  buildPhase = ''
    ${gems}/bin/bundle exec jekyll build
  '';

  installPhase = ''
    mkdir -p $out
    cp -Tr _site $out/public
  '';
}
