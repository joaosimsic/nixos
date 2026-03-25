{ config, pkgs, amberLib, devMode, amberPath, ... }:

{
  home.packages = with pkgs; [
    hyprland
    hyprpaper
    grim
    slurp
    brightnessctl
  ];

  xdg.configFile."hypr".source = amberLib.mkConfig {
    inherit config devMode amberPath;
    configPath = "domains/wm/hyprland/config";
    sourcePath = ./config;
  };
}
