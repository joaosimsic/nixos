{ pkgs, ... }:

{
  home.packages = with pkgs; [
    fzf
    ripgrep
    fd
  ];
}
