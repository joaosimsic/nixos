# Dotfiles symlinks
# Links ~/nixos/dotfiles/.config/* to ~/.config/*

{ config, userConfig, ... }:

let
  dotfilesPath = "${userConfig.homeDirectory}/nixos/dotfiles";
  mkSymlink = path: config.lib.file.mkOutOfStoreSymlink "${dotfilesPath}/${path}";
in
{
  xdg.configFile = {
    "hypr".source = mkSymlink ".config/hypr";
    "tofi".source = mkSymlink ".config/tofi";
    "mako".source = mkSymlink ".config/mako";
    "ghostty".source = mkSymlink ".config/ghostty";
    "nushell".source = mkSymlink ".config/nushell";
    "starship.toml".source = mkSymlink ".config/starship.toml";
    "zellij".source = mkSymlink ".config/zellij";
  };
}
