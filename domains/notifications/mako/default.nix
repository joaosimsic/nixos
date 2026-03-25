{ config, pkgs, amberLib, devMode, amberPath, ... }:

{
  home.packages = with pkgs; [
    mako
    libnotify
  ];

  xdg.configFile."mako".source = amberLib.mkConfig {
    inherit config devMode amberPath;
    configPath = "domains/notifications/mako/config";
    sourcePath = ./config;
  };
}
