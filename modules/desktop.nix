
{ pkgs, ... }:

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
  home.pointerCursor = {
    package = pkgs.bibata-cursors;
    name = "Bibata-Modern-Amber";
    size = 16;
    gtk.enable = true;
    x11.enable = true;
  };

  fonts.fontconfig.enable = true;

  home.packages = with pkgs; [
    nerd-fonts.jetbrains-mono
    
    hyprland
    hyprpaper
    waybar
    walker
    wofi
    mako
    
    grim
    slurp
    wl-clipboard
    brightnessctl
    libnotify
    
    ghostty
    thunar
    
    toggle-crt
  ];
}
