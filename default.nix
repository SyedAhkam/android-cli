{ pkgs ? import <nixpkgs> { } }:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "android-cli";
  version = "0.2";
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;

  OPENSSL_NO_VENDOR = 1;
  nativeBuildInputs = with pkgs; [pkg-config];
  buildInputs = with pkgs; [ openssl ];
}