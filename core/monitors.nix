{ lib, ... }:

let
  monitorSubmodule = lib.types.submodule {
    options = {
      name = lib.mkOption {
        type = lib.types.str;
        description = "Monitor output name";
      };
      resolution = lib.mkOption {
        type = lib.types.str;
        description = "Monitor resolution (e.g. 1920x1080)";
      };
      refreshRate = lib.mkOption {
        type = lib.types.int;
        description = "Monitor refresh rate in Hz";
      };
    };
  };
in
{
  options.monitors = {
    primary = lib.mkOption {
      type = monitorSubmodule;
      description = "Primary monitor configuration";
    };
    secondary = lib.mkOption {
      type = monitorSubmodule;
      description = "Secondary monitor configuration";
    };
  };
}
