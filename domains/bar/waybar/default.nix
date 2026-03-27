{ config, pkgs, lib, userConfig, monitors, amberLib, devMode, amberPath, ... }:

let
  devModeModule = lib.optionalAttrs devMode {
    "custom/devmode" = {
      format = " DEV";
      tooltip = false;
    };
  };

  commonModules = {
    "hyprland/submap" = {
      format = " RESIZE";
      tooltip = false;
    };

    "custom/weather" = {
      format = "{}";
      exec = "curl -s 'wttr.in/?format=%c%t' 2>/dev/null || echo ' --'";
      interval = 900;
      tooltip = false;
    };

    "hyprland/language" = {
      format = " {}";
      format-en = "EN";
      format-pt = "PT";
      format-es = "ES";
      format-de = "DE";
      format-fr = "FR";
    };

    pulseaudio = {
      format = "{icon} {volume}%";
      format-muted = " MUTE";
      format-icons = {
        default = ["" "" ""];
      };
      on-click = "wpctl set-mute @DEFAULT_AUDIO_SINK@ toggle";
    };

    network = {
      format-wifi = " {essid}";
      format-ethernet = " {ifname}";
      format-disconnected = " OFF";
      tooltip-format = "{ifname}: {ipaddr}";
    };

    "custom/date" = {
      format = " {}";
      exec = "date '+%Y-%m-%d'";
      interval = 60;
    };

    clock = {
      format = " {:%H:%M}";
      tooltip-format = "{:%A, %B %d, %Y}";
    };
  };

  windowRewrite = {
    "class<firefox>" = " [Firefox]";
    "class<chromium>" = " [Chromium]";
    "class<google-chrome>" = " [Chrome]";
    "class<brave-browser>" = " [Brave]";
    "class<ghostty>" = " [Ghostty]";
    "class<foot>" = " [Foot]";
    "class<org.codeberg.dnkl.foot>" = " [Foot]";
    "class<com.mitchellh.ghostty>" = " [Ghostty]";
    "class<nautilus>" = " [Files]";
    "class<org.gnome.Nautilus>" = " [Files]";
    "class<thunar>" = " [Thunar]";
    "class<nemo>" = " [Nemo]";
    "class<dolphin>" = " [Dolphin]";
    "class<spotify>" = " [Spotify]";
    "class<Spotify>" = " [Spotify]";
    "class<discord>" = " [Discord]";
    "class<slack>" = " [Slack]";
    "class<telegram>" = " [Telegram]";
    "class<org.telegram.desktop>" = " [Telegram]";
    "class<steam>" = " [Steam]";
    "class<gimp>" = " [GIMP]";
    "class<obs>" = " [OBS]";
    "class<mpv>" = " [MPV]";
    "class<vlc>" = " [VLC]";
    "class<virt-manager>" = " [Virt-Manager]";
    "class<org.prismlauncher.PrismLauncher>" = " [Prism]";
  };

  primaryBar = {
    output = monitors.primary.name;
    layer = "top";
    position = "top";
    height = 20;
    spacing = 0;
    margin-top = 0;
    margin-left = 0;
    margin-right = 0;

    modules-left = ["hyprland/workspaces" "hyprland/submap"];
    modules-center = ["custom/date" "clock"];
    modules-right = ["custom/weather" "hyprland/language" "pulseaudio" "network"] ++ (lib.optional devMode "custom/devmode");

    "hyprland/workspaces" = {
      format = "{name}{windows}";
      format-window-separator = " ";
      window-rewrite-default = " [?]";
      window-rewrite = windowRewrite;
      on-click = "activate";
      persistent-workspaces = {
        "${monitors.primary.name}" = [1 2 3 4 5];
      };
    };
  } // commonModules // devModeModule;

  secondaryBar = {
    output = monitors.secondary.name;
    layer = "top";
    position = "top";
    height = 16;
    spacing = 0;
    margin-top = 0;
    margin-left = 0;
    margin-right = 0;

    modules-left = ["hyprland/workspaces" "hyprland/submap"];
    modules-center = ["custom/date" "clock"];
    modules-right = ["custom/weather" "hyprland/language" "pulseaudio" "network"] ++ (lib.optional devMode "custom/devmode");

    "hyprland/workspaces" = {
      format = "{name}{windows}";
      format-window-separator = " ";
      window-rewrite-default = " [?]";
      window-rewrite = windowRewrite;
      on-click = "activate";
      format-icons = {
        "6" = "1";
        "7" = "2";
        "8" = "3";
        "9" = "4";
        "10" = "5";
      };
      persistent-workspaces = {
        "${monitors.secondary.name}" = [6 7 8 9 10];
      };
    };
  } // commonModules // devModeModule;

  waybarConfigPath = "${amberPath}/domains/bar/waybar/config";

in
{
  home.packages = with pkgs; [
    waybar
  ];

  xdg.configFile."waybar/config".text = builtins.toJSON [ primaryBar secondaryBar ];

  home.activation.waybarCss = lib.hm.dag.entryAfter ["writeBoundary"] ''
    if [ -f "${waybarConfigPath}/style.css" ]; then
      cp "${waybarConfigPath}/style.css" "${userConfig.homeDirectory}/.config/waybar/style.css"
      cp "${waybarConfigPath}/colors.css" "${userConfig.homeDirectory}/.config/waybar/colors.css"
      ${lib.optionalString devMode ''
        ${pkgs.gnused}/bin/sed -i 's/background-color: @base;/background-color: @bright;/' "${userConfig.homeDirectory}/.config/waybar/style.css"
      ''}
    fi
  '';
}
