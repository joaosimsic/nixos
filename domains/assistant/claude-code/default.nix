{ config, pkgs, amberLib, devMode, amberPath, ... }:

{
  home.packages = with pkgs; [
    claude-code
  ];

  xdg.configFile."claude".source = amberLib.mkConfig {
    inherit config devMode amberPath;
    configPath = "domains/assistant/claude/config";
    sourcePath = ./config;
  };
}
