{ config, pkgs, inputs, hostname, nixosConfigPath, userConfig, ... }:

{
  imports = [
    ./hardware-configuration.nix
    ../../modules/system/common.nix
    ../../modules/system/desktop-hyprland-sddm.nix
  ];

  networking.hostName = hostname;

  boot.kernelPackages = pkgs.linuxPackages_latest;
  boot.kernelParams = [ "amd_pstate=active" ];

  boot.initrd.kernelModules = [ "amdgpu" ];
  services.xserver.videoDrivers = [ "amdgpu" ];

  hardware.graphics = {
    enable = true;
    enable32Bit = true;
    extraPackages = with pkgs; [ rocmPackages.clr.icd ];
  };

  hardware.cpu.amd.updateMicrocode = true;

  zramSwap.enable = true;

  services.fstrim.enable = true;

  system.stateVersion = "24.11";
}
