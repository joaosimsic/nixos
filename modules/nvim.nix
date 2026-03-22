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

  home.packages = with pkgs; [
    lua-language-server
    nil
    rust-analyzer
    pyright
    nodePackages.typescript-language-server
    nodePackages.vscode-langservers-extracted
    gopls
    clang-tools

    lua51Packages.luarocks
  ];
}
