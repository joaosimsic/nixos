{ config, pkgs, amberLib, devMode, amberPath, ... }:

{
  home.packages = with pkgs; [
    nushell
    starship
  ];

  xdg.configFile."nushell/config.nu".source = amberLib.mkConfigFile {
    inherit config devMode amberPath;
    configPath = "domains/shell/nushell/config/config.nu";
    sourcePath = ./config/config.nu;
  };

  xdg.configFile."nushell/env.nu".source = amberLib.mkConfigFile {
    inherit config devMode amberPath;
    configPath = "domains/shell/nushell/config/env.nu";
    sourcePath = ./config/env.nu;
  };

  xdg.configFile."nushell/scripts".source = amberLib.mkConfig {
    inherit config devMode amberPath;
    configPath = "domains/shell/nushell/config/scripts";
    sourcePath = ./config/scripts;
  };

  xdg.configFile."starship.toml".source = amberLib.mkConfigFile {
    inherit config devMode amberPath;
    configPath = "domains/shell/nushell/config/starship.toml";
    sourcePath = ./config/starship.toml;
  };
}
