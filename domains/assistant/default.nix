{ config, pkgs, amberLib, devMode, amberPath, ... }:

{
  home.packages = with pkgs; [
    opencode
    claude-code
  ];

  xdg.configFile."opencode".source = amberLib.mkConfig {
    inherit config devMode amberPath;
    configPath = "domains/assistant/config";
    sourcePath = ./config;
  };
}
