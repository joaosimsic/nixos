{ config, pkgs, ... }:

{
  home.packages = with pkgs; [
    opencode
  ];

  xdg.configFile."opencode".source = ./config;
}
