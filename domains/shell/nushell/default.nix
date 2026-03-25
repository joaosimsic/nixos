{ config, pkgs, amberLib, devMode, amberPath, ... }:

{
  home.packages = with pkgs; [
    nushell
    starship
  ];

  xdg.configFile."nushell".source = amberLib.mkConfig {
    inherit config devMode amberPath;
    configPath = "domains/shell/nushell/config";
    sourcePath = ./config;
  };

  xdg.configFile."starship.toml".source = amberLib.mkConfigFile {
    inherit config devMode amberPath;
    configPath = "domains/shell/nushell/config/starship.toml";
    sourcePath = ./config/starship.toml;
  };
}
