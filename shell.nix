let
  fenix = import (fetchTarball https://github.com/nix-community/fenix/archive/main.tar.gz);
in

with (import <nixpkgs> {
  overlays = [fenix];
});

mkShell {
  name = "mailersend";

  buildInputs = [
    # For building.
    clang rust-nightly.latest.toolchain pkg-config openssl libsodium
  ];

  LIBCLANG_PATH = "${llvmPackages.libclang}/lib";
  RUST_BACKTRACE = 1;
  RUST_LOG = "info";
}
