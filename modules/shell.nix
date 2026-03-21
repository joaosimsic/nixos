# Shell configuration

{ pkgs, ... }:

{
  home.packages = with pkgs; [
    starship
    ripgrep
    fd
    fzf
    bat
    eza
  ];
}
