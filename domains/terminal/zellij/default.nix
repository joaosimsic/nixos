{ config, pkgs, amberLib, devMode, amberPath, ... }:

{
  home.packages = with pkgs; [
    zellij
  ];

  xdg.configFile."zellij".source = amberLib.mkConfig {
    inherit config devMode amberPath;
    configPath = "domains/terminal/zellij/config";
    sourcePath = ./config;
  };
}
