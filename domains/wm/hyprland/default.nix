{ config, pkgs, monitors, ... }:

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
    monitor = ${monitors.primary.name}, ${monitors.primary.resolution}@${toString monitors.primary.refreshRate}, 0x0, 1
    monitor = ${monitors.secondary.name}, ${monitors.secondary.resolution}@${toString monitors.secondary.refreshRate}, 1920x0, 1

    workspace = 1, monitor:${monitors.primary.name}
    workspace = 2, monitor:${monitors.primary.name}
    workspace = 3, monitor:${monitors.primary.name}
    workspace = 4, monitor:${monitors.primary.name}
    workspace = 5, monitor:${monitors.primary.name}
    workspace = 6, monitor:${monitors.secondary.name}
    workspace = 7, monitor:${monitors.secondary.name}
    workspace = 8, monitor:${monitors.secondary.name}
    workspace = 9, monitor:${monitors.secondary.name}
    workspace = 10, monitor:${monitors.secondary.name}
  '';
}
