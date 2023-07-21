{
  description = "A cargo subcommand to query or set the version of a crate.";

  inputs = {
    cargo2nix = {
      inputs = {
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
        rust-overlay.follows = "rust-overlay";
      };
      url = "github:cargo2nix/cargo2nix";
    };
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;

          overlays = [ cargo2nix.overlays.default ];
        };

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "latest";
          packageFun = import ./Cargo.nix;
          extraRustComponents = ["clippy" "rustfmt"];
        };

      in rec {
        devShell = rustPkgs.workspaceShell {};
        packages = {};
      }
    );
}
