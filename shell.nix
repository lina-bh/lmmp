{ pkgs ? import <nixpkgs> { } }:
let inherit (pkgs) stdenv lib;
in pkgs.mkShell {
  buildInputs = with pkgs; [
    xorg.libX11
    binutils
    ffmpeg
    pkgconfig
    rustc
    cargo
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath [ pkgs.libGL ];
  LIBCLANG_PATH = lib.makeLibraryPath [ pkgs.rustc.llvmPackages.libclang ];
  BINDGEN_EXTRA_CLANG_ARGS = ''
    -I${stdenv.glibc.dev}/include
      -I${stdenv.cc.cc}/lib/gcc/${stdenv.hostPlatform.config}/${
        lib.getVersion stdenv.cc.cc
      }/include
      -I${stdenv.cc.cc}/lib/gcc/${stdenv.hostPlatform.config}/${
        lib.getVersion stdenv.cc.cc
      }/include-fixed'';
}
