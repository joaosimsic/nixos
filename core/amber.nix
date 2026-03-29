{ pkgs }:

let
  amber-cli = pkgs.rustPlatform.buildRustPackage {
    pname = "amber-cli";
    version = "0.1.0";
    src = ../tools;
    cargoLock.lockFile = ../tools/Cargo.lock;
  };
in
pkgs.writeShellScriptBin "amber" ''
  exec ${amber-cli}/bin/amber "$@"
''
