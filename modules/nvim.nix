{ pkgs, config, userConfig, ... }:

let
  dotfilesPath = "${userConfig.homeDirectory}/nixos/dotfiles";
in
{
  programs.neovim = {
    enable = true;
    defaultEditor = true;
    viAlias = true;
    vimAlias = true;
  };

  xdg.configFile."nvim" = {
    source = config.lib.file.mkOutOfStoreSymlink "${dotfilesPath}/.config/nvim";
  };
}
