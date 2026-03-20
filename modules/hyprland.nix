{ pkgs, config, nixosConfigPath, ... }:

let
  toggle-crt = pkgs.writeShellScriptBin "toggle-crt" ''
    SHADER_PATH="$HOME/.config/hypr/shaders/crt-amber.glsl"
    STATE_FILE="/tmp/hypr-crt-shader-state"

    if [ -f "$STATE_FILE" ]; then
        hyprctl keyword decoration:screen_shader ""
        rm "$STATE_FILE"
        notify-send -t 1500 "CRT MODE" "DISABLED" -h string:x-canonical-private-synchronous:crt
    else
        hyprctl keyword decoration:screen_shader "$SHADER_PATH"
        touch "$STATE_FILE"
        notify-send -t 1500 "CRT MODE" "ENABLED" -h string:x-canonical-private-synchronous:crt
    fi
  '';
in
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

  home.pointerCursor = {
    package = pkgs.bibata-cursors;
    name = "Bibata-Modern-Amber";
    size = 16;
    gtk.enable = true;
    x11.enable = true;
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
    libnotify
    toggle-crt
  ];
}
