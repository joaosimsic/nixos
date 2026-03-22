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
    # LSP Servers
    lua-language-server
    nil
    rust-analyzer
    pyright
    nodePackages.typescript-language-server
    nodePackages.vscode-langservers-extracted # html, css, json, eslint
    gopls
    clang-tools # clangd + clang-format
    nodePackages.intelephense
    dockerfile-language-server-nodejs
    docker-compose-language-service
    nodePackages.bash-language-server
    prisma-language-server
    vue-language-server
    terraform-ls
    jdt-language-server
    lemminx
    roslyn-ls

    # Formatters
    stylua
    black
    prettierd
    nodePackages."@angular/language-server"
    nodePackages.blade-formatter
    google-java-format
    gotools # goimports
    gofumpt
    rustfmt

    # Linters
    eslint_d
    pylint
    golangci-lint

    # Other
    lua51Packages.luarocks
  ];
}
