{ config, pkgs, ... }:

{
  home.packages = with pkgs; [
    lazydocker
  ];

  xdg.configFile."lazydocker".source = ./config;
}
