{
  description = "Steam Deck input remapper";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs: with inputs; flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import nixpkgs {
      inherit system;
      overlays = [
        rust-overlay.overlays.default
      ];
    };

    rusttoolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
    cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
    sharedDeps = with pkgs; [ rusttoolchain pkg-config ];
  in {
    packages = rec {
      steam-deck-remapper = pkgs.rustPlatform.buildRustPackage {
        pname = cargoToml.package.name;
        version = cargoToml.package.version;
        src = ./.;
        cargoLock = {
          outputHashes = {};
          lockFile = ./Cargo.lock;
        };

        nativeBuildInputs = sharedDeps;
      };
          
      default = steam-deck-remapper;
    };


    devShell = pkgs.mkShell {
      buildInputs = sharedDeps;
    };
  }) // {
    homeManagerModules.default = import ./nix/hm-module.nix self;
  };
}
