{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    utils.url = "github:numtide/flake-utils";
    nix-filter.url = "github:numtide/nix-filter";

    oxalica-rust = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ self, nixpkgs, utils, nix-filter, oxalica-rust, poetry2nix, ... }:
    let
      systems = [ "x86_64-linux" ];
    in utils.lib.eachSystem systems (system:
      let
        pkgs = import nixpkgs {
          inherit system;

          overlays = [
            oxalica-rust.overlays.default
          ];
        };

        inherit (pkgs) lib enableDebugging;

        llvmPackages = pkgs.llvmPackages_13;
        rustNightly = pkgs.rust-bin.nightly.latest;
        rustStable = pkgs.rust-bin.beta.latest;
        rust = rustStable.minimal;
      in rec {
        devShell = with pkgs; (mkShell.override { inherit (llvmPackages) stdenv; }) {
          buildInputs = [
            # For building.
            llvmPackages.clang llvmPackages.libclang.lib
            llvmPackages.bintools

            (rust.override {
              extensions = [ "rust-src" "clippy" ];
            })
            rustNightly.rustfmt
            rustNightly.rust-analyzer-preview

            cargo-deny
            cargo-outdated
            cargo-watch

            pkg-config openssl libsodium
          ];

          LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
          RUST_BACKTRACE = "full";
          RUSTC_BOOTSTRAP = 1;
          RUST_LOG = "";
          RUSTFLAGS = lib.concatStringsSep " " [
            # "-C prefer-dynamic"
            "-C target-cpu=native"
            "-C linker=clang"
            "-C link-arg=-fuse-ld=lld"
            "-Z threads=8"
            "-Z share-generics=y"
            "-Z macro-backtrace"
          ];

          RUST_SRC = rustStable.rust-src;
        };
    });
}
