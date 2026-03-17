{ config, pkgs, ... }:

{
  imports = [
    ./modules/nvim.nix
    ./modules/hyprland.nix
  ];

  home.username = "joao";
  home.homeDirectory = "/home/joao";
  home.stateVersion = "24.05";

  programs.home-manager.enable = true;
  programs.firefox.enable = true;
}
