{ config, pkgs, inputs, hostname, nixosConfigPath, userConfig, ... }:

{
  imports = [
    ./hardware-configuration.nix
    ../../modules/system/common.nix
    ../../modules/system/desktop-hyprland-sddm.nix
  ];

  networking.hostName = hostname;

  boot.kernelPackages = pkgs.linuxPackages_latest;

  services.qemuGuest.enable = true;
  services.spice-vdagentd.enable = true;

  environment.sessionVariables = {
    WLR_RENDERER_ALLOW_SOFTWARE = "1";
    LIBGL_ALWAYS_SOFTWARE = "1";
    WLR_NO_HARDWARE_CURSORS = "1";
  };

  zramSwap.enable = true;

  system.stateVersion = "24.11";
}
