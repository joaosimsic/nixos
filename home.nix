{ config, pkgs, userConfig, ... }:

{
  imports = [
    ./core/home.nix
    ./profiles/default.nix
  ];
}
