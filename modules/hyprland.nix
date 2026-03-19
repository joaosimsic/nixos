{ pkgs, config, nixosConfigPath, ... }:

{
  wayland.windowManager.hyprland.enable = false;

  xdg.configFile."hypr" = {
    source = config.lib.file.mkOutOfStoreSymlink "${nixosConfigPath}/dotfiles/.config/hypr";
  };

  xdg.configFile."waybar" = {
    source = config.lib.file.mkOutOfStoreSymlink "${nixosConfigPath}/dotfiles/.config/waybar";
  };

  xdg.configFile."wofi" = {
    source = config.lib.file.mkOutOfStoreSymlink "${nixosConfigPath}/dotfiles/.config/wofi";
  };

  xdg.configFile."mako" = {
    source = config.lib.file.mkOutOfStoreSymlink "${nixosConfigPath}/dotfiles/.config/mako";
  };

  xdg.configFile."ghostty" = {
    source = config.lib.file.mkOutOfStoreSymlink "${nixosConfigPath}/dotfiles/.config/ghostty";
  };

  xdg.configFile."starship.toml" = {
    source = config.lib.file.mkOutOfStoreSymlink "${nixosConfigPath}/dotfiles/.config/starship.toml";
  };

  home.file.".local/bin/toggle-crt" = {
    source = config.lib.file.mkOutOfStoreSymlink "${nixosConfigPath}/dotfiles/.local/bin/toggle-crt";
    executable = true;
  };

  home.packages = with pkgs; [
    hyprland
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
