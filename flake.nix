{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-23.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
      inherit (pkgs) lib;
      inherit (pkgs.stdenv) isDarwin;
    in {
      devShells.default = pkgs.mkShell {
        name = "tucano";
        packages = let
          commonPackages = with pkgs; [
            # The usual Rust profile
            (rust-bin.stable."1.79.0".default.override {
              extensions = ["rust-src" "rust-analyzer" "llvm-tools"];
            })
            # We need a nightly version of rustfmt to format this crate
            (writeShellApplication {
              name = "cargo-nightly-fmt";
              runtimeInputs = [
                (rust-bin.selectLatestNightlyWith (toolchain:
                  toolchain.minimal.override {
                    extensions = ["rustfmt"];
                  }))
              ];
              text = ''cargo fmt "$@"'';
            })
            cargo-nextest

            gnumake

            pkg-config
            llvmPackages_16.llvm
            llvmPackages_16.bintools
            libiconv
          ];
          darwinPackages = lib.optionals isDarwin (with pkgs.darwin.apple_sdk.frameworks; [
            CoreFoundation
            CoreServices
            SystemConfiguration
            IOKit
          ]);
        in (commonPackages ++ darwinPackages);
      };
    });
}
