{ config, pkgs, amberLib, devMode, amberPath, ... }:

{
  home.packages = with pkgs; [
    ghostty
  ];

  xdg.configFile."ghostty".source = amberLib.mkConfig {
    inherit config devMode amberPath;
    configPath = "domains/terminal/ghostty/config";
    sourcePath = ./config;
  };
}
