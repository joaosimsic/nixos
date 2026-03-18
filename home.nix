{ config, pkgs, userConfig, ... }:

{
  imports = [
    ./modules/nvim.nix
    ./modules/hyprland.nix
  ];

  home.username = userConfig.username;
  home.homeDirectory = userConfig.homeDirectory;
  home.stateVersion = "24.05";

  programs.home-manager.enable = true;
  programs.firefox.enable = true;
}
