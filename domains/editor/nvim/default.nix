{ config, pkgs, amberLib, devMode, amberPath, ... }:

{
  programs.neovim = {
    enable = true;
    defaultEditor = true;
    viAlias = true;
    vimAlias = true;
  };

  xdg.configFile."nvim".source = amberLib.mkConfig {
    inherit config devMode amberPath;
    configPath = "domains/editor/nvim/config";
    sourcePath = ./config;
  };
}
