{ config, pkgs, ... }:

{
  home.packages = with pkgs; [
    claude-code
  ];

  xdg.configFile."claude/CLAUDE.md".source = ./config/CLAUDE.md;
  xdg.configFile."claude/settings.json".source = ./config/settings.json;
  xdg.configFile."claude/statusbar.sh".source = ./config/statusbar.sh;
}
