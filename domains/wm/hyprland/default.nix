{ config, pkgs, amberLib, devMode, amberPath, monitors, ... }:

{
  home.packages = with pkgs; [
    hyprland
    hyprpaper
    hypridle
    hyprlock
    grim
    slurp
    brightnessctl
  ];

  xdg.configFile."hypr/hyprland.conf".source = ./config/hyprland.conf;
  xdg.configFile."hypr/input.conf".source = ./config/input.conf;
  xdg.configFile."hypr/looknfeel.conf".source = ./config/looknfeel.conf;
  xdg.configFile."hypr/bindings.conf".source = ./config/bindings.conf;
  xdg.configFile."hypr/windowrules.conf".source = ./config/windowrules.conf;
  xdg.configFile."hypr/autostart.conf".source = ./config/autostart.conf;
  xdg.configFile."hypr/envs.conf".source = ./config/envs.conf;
  xdg.configFile."hypr/colors.conf".source = ./config/colors.conf;
  xdg.configFile."hypr/hypridle.conf".source = ./config/hypridle.conf;
  xdg.configFile."hypr/hyprlock.conf".source = ./config/hyprlock.conf;
  xdg.configFile."hypr/shaders".source = ./config/shaders;

  xdg.configFile."hypr/monitors.conf".text = ''
    monitor = ${monitors.primary}, preferred, 0x0, 1
    monitor = ${monitors.secondary}, preferred, 1920x0, 1

    workspace = 1, monitor:${monitors.primary}
    workspace = 2, monitor:${monitors.primary}
    workspace = 3, monitor:${monitors.primary}
    workspace = 4, monitor:${monitors.primary}
    workspace = 5, monitor:${monitors.primary}
    workspace = 6, monitor:${monitors.secondary}
    workspace = 7, monitor:${monitors.secondary}
    workspace = 8, monitor:${monitors.secondary}
    workspace = 9, monitor:${monitors.secondary}
    workspace = 10, monitor:${monitors.secondary}
  '';
}
