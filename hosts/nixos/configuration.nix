{ config, pkgs, hostname, userConfig, ... }:

{
  imports = [
    ./hardware-configuration.nix
    ../../modules/system/common.nix
    ../../modules/system/desktop-hyprland-sddm.nix
  ];

  networking.hostName = hostname;

  system.stateVersion = "24.05";
}
