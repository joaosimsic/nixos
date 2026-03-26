{ config, lib, pkgs, modulesPath, ... }:
{
  imports = [
    (modulesPath + "/profiles/qemu-guest.nix")
  ];

  boot.initrd.availableKernelModules = [
    "virtio_pci" "virtio_blk" "virtio_scsi" "virtio_net" "xhci_pci"
  ];

  monitors = {
    primary = {
      name = "Virtual-1";
      resolution = "1920x1080";
      refreshRate = 144;
    };
    secondary = {
      name = "Virtual-2";
      resolution = "1920x1080";
      refreshRate = 60;
    };
  };

  fileSystems."/" = {
    device = "/dev/disk/by-label/root";
    fsType = "ext4";
  };

  fileSystems."/boot" = {
    device = "/dev/vda1";
    fsType = "vfat";
  };
}

