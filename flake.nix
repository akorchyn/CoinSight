{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-22.11"; # We want to use packages from the binary cache
    flake-utils.url = "github:numtide/flake-utils";
    # shells
    devenv.url = "github:cachix/devenv";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = inputs@{ self, nixpkgs, flake-utils, devenv, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem 
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs  = import nixpkgs {
            inherit system overlays;
          };

          rust-env = with pkgs; {
            PROTOC = "${pkgs.protobuf}/bin/protoc";
            RUSTUP_TOOLCHAIN = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml)).toolchain.channel; # for dylint
          };
          rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        in rec {
          devShells = {
            default = devenv.lib.mkShell {
              inherit inputs pkgs;
              modules = [
                {
                  packages = with pkgs; [rust-toolchain binaryen protobuf];
                  env = rust-env;
                  enterShell = "echo csshell";
                }
              ];
            };
          };
        }
      );
} 
