let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  ruststable = nixpkgs.latest.rustChannels.stable.rust.override {
        extensions = [ "rust-src" "rustfmt-preview" "clippy-preview" ];
      };
in
  with nixpkgs;
clangStdenv.mkDerivation {
  name = "rust-env";
  buildInputs = [
    rust-analyzer
    ruststable
    pkg-config
    openssl
    python
    nix
    cmake
    llvmPackages.libclang
    bash
    rust-bindgen
    valgrind
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
  RUST_SRC_PATH="${ruststable}/lib/rustlib/src/rust/src";
  LIBCLANG_PATH="${llvmPackages.libclang}/lib";
  KISSFFT="${kissfft}";
}
