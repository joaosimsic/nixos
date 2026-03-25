{ config, pkgs, amberLib, devMode, amberPath, ... }:

{
  home.packages = with pkgs; [
    lazydocker
  ];

  xdg.configFile."lazydocker".source = amberLib.mkConfig {
    inherit config devMode amberPath;
    configPath = "capabilities/containers/config";
    sourcePath = ./config;
  };
}
