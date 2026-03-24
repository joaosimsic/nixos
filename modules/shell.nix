# Shell configuration

{ pkgs, ... }:

{
  home.packages = with pkgs; [
    repomix
    starship
    ripgrep
    fd
    fzf
    bat
    eza
    zellij
  ];
}
