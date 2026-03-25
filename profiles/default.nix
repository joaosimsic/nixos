{ pkgs, ... }:

{
  imports = [
    ../capabilities/fuzzy.nix
    ../capabilities/clipboard.nix
    ../capabilities/archive.nix
    ../capabilities/networking.nix
    ../capabilities/files.nix
    ../capabilities/git
    ../capabilities/containers

    ../toolchains/node.nix
    ../toolchains/python.nix
    ../toolchains/rust.nix
    ../toolchains/go.nix
    ../toolchains/c.nix
    ../toolchains/lua.nix
    ../toolchains/nix.nix
    ../toolchains/java.nix
    ../toolchains/php.nix
    ../toolchains/web.nix
    ../toolchains/docker.nix
    ../toolchains/bash.nix
    ../toolchains/terraform.nix
    ../toolchains/xml.nix
    ../toolchains/csharp.nix

    ../domains/editor/nvim
    ../domains/wm/hyprland
    ../domains/terminal/ghostty
    ../domains/terminal/zellij
    ../domains/shell/nushell
    ../domains/bar/waybar
    ../domains/launcher/rofi
    ../domains/notifications/mako
    ../domains/assistant
  ];

  programs.firefox.enable = true;

  home.packages = with pkgs; [
    thunar
  ];
}
