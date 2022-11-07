let
  # TODO: pin nixpkgs to a specific version?
  # nixpkgs = import (fetchTarball("https://github.com/NixOS/nixpkgs/archive/52f9de5b39a22c86899d6e35c68ac7fab42dbb98.tar.gz")) {};
  nixpkgs = import <nixpkgs> { };
  inherit (nixpkgs) pkgs llvmPackages;
  stdenv = pkgs.clangStdenv;
in
stdenv.mkDerivation {
  name = "c2rust";
  buildInputs = [
    # Rust stuff
    pkgs.rustc
    pkgs.cargo
    # for c2rust
    pkgs.libclang
    pkgs.clang
    pkgs.llvmPackages.libclang
    pkgs.cmake
    pkgs.llvm
    pkgs.openssl
    pkgs.pkgconfig
    pkgs.python3
    pkgs.zlib
    # for getting compile_commands.json from Makefile in transpile.sh
    pkgs.bear
    # for refactoring in transpile.sh
    pkgs.gnused
    pkgs.perl
  ];
  LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
}
