{ pkgs, config, ... }:

{
  wayland.windowManager.hyprland = {
    enable = true;
    systemd.enable = true;
    xwayland.enable = true;
  };

  xdg.configFile."hypr" = {
    source = config.lib.file.mkOutOfStoreSymlink "/home/joao/proj/nixos/home/dotfiles/.config/hypr";
  };

  xdg.configFile."waybar" = {
    source = config.lib.file.mkOutOfStoreSymlink "/home/joao/proj/nixos/home/dotfiles/.config/waybar";
    };

  xdg.configFile."wofi" = {
    source = config.lib.file.mkOutOfStoreSymlink "/home/joao/proj/nixos/home/dotfiles/.config/wofi";
  };

  xdg.configFile."mako" = {
    source = config.lib.file.mkOutOfStoreSymlink "/home/joao/proj/nixos/home/dotfiles/.config/mako";
  };

  xdg.configFile."ghostty" = {
    source = config.lib.file.mkOutOfStoreSymlink "/home/joao/proj/nixos/home/dotfiles/.config/ghostty";
  };

  home.packages = with pkgs; [
    waybar
    hyprpaper
    mako
    wofi
    grim
    slurp
    wl-clipboard
    brightnessctl
    ghostty
    thunar
  ];
}
