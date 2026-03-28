{ config, pkgs, ... }:

{
  home.packages = with pkgs; [
    mako
    libnotify
  ];

  xdg.configFile."mako".source = ./config;
}
