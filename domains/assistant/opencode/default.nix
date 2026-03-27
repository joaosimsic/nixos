{ config, pkgs, amberLib, devMode, amberPath, ... }:

{
  home.packages = with pkgs; [
    opencode
  ];

  xdg.configFile."opencode".source = amberLib.mkConfig {
    inherit config devMode amberPath;
    configPath = "domains/assistant/opencode/config";
    sourcePath = ./config;
  };
}
