{ config, pkgs, amberLib, devMode, amberPath, ... }:

{
  home.packages = with pkgs; [
    lazygit
  ];

  xdg.configFile."lazygit".source = amberLib.mkConfig {
    inherit config devMode amberPath;
    configPath = "capabilities/git/config";
    sourcePath = ./config;
  };
}
