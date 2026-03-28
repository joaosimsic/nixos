{ config, pkgs, ... }:

{
  home.packages = with pkgs; [
    nushell
    starship
  ];

  xdg.configFile."nushell/config.nu".source = ./config/config.nu;
  xdg.configFile."nushell/env.nu".source = ./config/env.nu;
  xdg.configFile."nushell/scripts".source = ./config/scripts;
  xdg.configFile."starship.toml".source = ./config/starship.toml;
}
