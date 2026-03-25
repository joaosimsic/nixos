{ lib, ... }:

{
  options.monitors = {
    primary = lib.mkOption {
      type = lib.types.str;
      default = "DP-1";
      description = "Primary monitor name";
    };
    secondary = lib.mkOption {
      type = lib.types.str;
      default = "HDMI-A-2";
      description = "Secondary monitor name";
    };
  };
}
