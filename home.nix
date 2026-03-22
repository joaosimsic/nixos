{ config, pkgs, userConfig, ... }:

{
  imports = [
    ./modules/dotfiles.nix
    ./modules/desktop.nix
    ./modules/shell.nix
    ./modules/nvim.nix
    ./modules/waybar.nix
  ];

  home.username = userConfig.username;
  home.homeDirectory = userConfig.homeDirectory;
  home.stateVersion = "24.05";

  programs.home-manager.enable = true;
  programs.firefox.enable = true;
}
