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

  monitors = {
    primary = "Virtual-1";
    secondary = "Virtual-2";
  };

  services.xserver.videoDrivers = [ "modesetting" ];

  hardware.graphics.enable = true;

  environment.sessionVariables = {
    WLR_NO_HARDWARE_CURSORS = "1";
  };

  zramSwap.enable = true;

  system.stateVersion = "24.11";
}
