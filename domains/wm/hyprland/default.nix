{ config, pkgs, ... }:

{
  home.packages = with pkgs; [
    hyprland
    hyprpaper
    hypridle
    hyprlock
    grim
    slurp
    brightnessctl
  ];
}
