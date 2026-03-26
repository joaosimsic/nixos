{ config, pkgs, inputs, hostname, ... }:

{
  imports = [
    ./hardware-configuration.nix
  ];

  networking.hostName = hostname;

  boot.kernelPackages = pkgs.linuxPackages_latest;

  services.qemuGuest.enable = true;
  services.spice-vdagentd.enable = true;

  boot.kernelParams = [
    "video=Virtual-1:1920x1080@144e"
    "video=Virtual-2:1920x1080@60e"
  ];

  services.xserver.videoDrivers = [ "modesetting" ];

  hardware.graphics.enable = true;

  environment.sessionVariables = {
    WLR_NO_HARDWARE_CURSORS = "1";
  };

  zramSwap.enable = true;

  system.stateVersion = "24.11";
}
