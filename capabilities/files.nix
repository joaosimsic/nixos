{ pkgs, ... }:

{
  home.packages = with pkgs; [
    bat
    eza
    tree-sitter
    lsof
    repomix
  ];
}
