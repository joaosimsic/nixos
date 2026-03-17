{ pkgs, config, ... }:

{
  wayland.windowManager.hyprland = {
    enable = true;
    systemd.enable = true;
    xwayland.enable = true;
  };

  # Symlink configs from dotfiles
  xdg.configFile."hypr" = {
    source = config.lib.file.mkOutOfStoreSymlink "/home/joao/proj/nixos/dotfiles/.config/hypr";
  };

  xdg.configFile."waybar" = {
    source = config.lib.file.mkOutOfStoreSymlink "/home/joao/proj/nixos/dotfiles/.config/waybar";
  };

  xdg.configFile."wofi" = {
    source = config.lib.file.mkOutOfStoreSymlink "/home/joao/proj/nixos/dotfiles/.config/wofi";
  };

  xdg.configFile."mako" = {
    source = config.lib.file.mkOutOfStoreSymlink "/home/joao/proj/nixos/dotfiles/.config/mako";
  };

  xdg.configFile."ghostty" = {
    source = config.lib.file.mkOutOfStoreSymlink "/home/joao/proj/nixos/dotfiles/.config/ghostty";
  };

  # Services
  services.mako.enable = true;

  home.packages = with pkgs; [
    # Wayland essentials
    waybar
    hyprpaper
    mako
    wofi
    
    # Screenshot
    grim
    slurp
    
    # Clipboard
    wl-clipboard
    cliphist
    
    # System control
    brightnessctl
    playerctl
    
    # Lock screen & idle
    hyprlock
    hypridle
    
    # Terminal & file manager
    ghostty
    thunar
    
    # Utilities
    pavucontrol
    networkmanagerapplet
    btop
    
    # Fonts
    jetbrains-mono
    
    # Polkit
    polkit_gnome
  ];
}
