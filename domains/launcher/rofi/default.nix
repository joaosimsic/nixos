{ config, pkgs, amberLib, devMode, amberPath, ... }:

{
  home.packages = with pkgs; [
    rofi
  ];

  xdg.configFile."rofi".source = amberLib.mkConfig {
    inherit config devMode amberPath;
    configPath = "domains/launcher/rofi/config";
    sourcePath = ./config;
  };
}
