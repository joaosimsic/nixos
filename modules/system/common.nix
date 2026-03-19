{ config, lib, pkgs, userConfig, ... }:
{
  # Keyboard layout (can be overridden in host modules via mkForce/mkDefault).
  console.keyMap = lib.mkDefault "br-abnt2";
  services.xserver.xkb = {
    layout = lib.mkDefault "br";
    variant = lib.mkDefault "abnt2";
  };

  # Locale/time defaults (can be overridden per-host).
  time.timeZone = lib.mkDefault "America/Sao_Paulo";
  i18n.defaultLocale = lib.mkDefault "pt_BR.UTF-8";

  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  networking.networkmanager.enable = true;

  nix.settings.experimental-features = [ "nix-command" "flakes" ];
  nixpkgs.config.allowUnfree = true;

  users.users.${userConfig.username} = {
    isNormalUser = true;
    description = userConfig.username;
    extraGroups = [ "wheel" "networkmanager" "video" "audio" ];
    initialPassword = lib.mkDefault "changeme";
  };

  security.rtkit.enable = true;

  services.pipewire = {
    enable = true;
    alsa.enable = true;
    alsa.support32Bit = true;
    pulse.enable = true;
  };

  environment.systemPackages = with pkgs; [
    git
    wget
    ghostty
  ];
}

