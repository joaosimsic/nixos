{ config, lib, pkgs, userConfig, ... }:

let
  amber = pkgs.writeShellScriptBin "amber" ''
    COMMAND=$1

    if [ "$COMMAND" = "dev" ]; then
      echo "Unlocking Dev Mode: Auto-linking configurations..."

      for config_dir in /home/joao/.config/amber/domains/*/*/config /home/joao/.config/amber/capabilities/*/config; do
        if [ -d "$config_dir" ]; then
          
          app_name=$(basename $(dirname "$config_dir"))
          target_name="$app_name"

          case "$app_name" in
            hyprland)    target_name="hypr" ;;
            claude-code) target_name="claude" ;;
          esac

          TARGET="/home/joao/.config/$target_name"

          ln -sfn "$config_dir" "$TARGET"
          echo " -> Linked $target_name"
        fi
      done

      if [ -f "/home/joao/.config/amber/domains/shell/nushell/config/starship.toml" ]; then
        ln -sfn "/home/joao/.config/amber/domains/shell/nushell/config/starship.toml" "/home/joao/.config/starship.toml"
        echo " -> Linked starship.toml"
      fi

      echo ""
      echo "Done. You are now live-editing your repository."

    elif [ "$COMMAND" = "lock" ]; then
      echo "Locking system: Restoring Nix immutable configurations..."
      
      HOSTNAME=$(hostname)
      home-manager switch --flake "/home/joao/.config/amber#joao@$HOSTNAME" -b backup
      
      echo ""
      echo "Done. System state is secure."

    elif [ "$COMMAND" = "clean" ]; then
      echo "Cleaning up legacy home-manager artifacts..."
      
      if [ -d "/home/joao/.local/bin" ]; then
        rm -rf /home/joao/.local/bin
        echo " -> Removed ~/.local/bin"
      fi
      
      if [ -d "/home/joao/.local/state/home-manager" ]; then
        rm -rf /home/joao/.local/state/home-manager
        echo " -> Removed ~/.local/state/home-manager"
      fi
      
      if nix-env -q 2>/dev/null | grep -q .; then
        echo " -> Removing nix-env packages..."
        nix-env -e '.*'
      fi
      
      find /home/joao/.config -name "*.backup" -type f -delete 2>/dev/null
      echo " -> Removed .backup files"
      
      echo ""
      echo "Done. Run 'amber lock' to restore clean state."

    else
      echo "Amber CLI"
      echo "Usage: amber <command>"
      echo ""
      echo "Commands:"
      echo "  dev   - Auto-link all repo configs to ~/.config (Live 0ms iteration)"
      echo "  lock  - Run home-manager switch to restore immutable Nix safety"
      echo "  clean - Remove legacy artifacts from old home-manager setup"
    fi
  '';
in
{
  imports = [
    ./audio.nix
    ./monitors.nix
  ];

  console.keyMap = lib.mkDefault "br-abnt2";
  services.xserver.xkb = {
    layout = lib.mkDefault "br";
    variant = lib.mkDefault "abnt2";
  };

  time.timeZone = lib.mkDefault "America/Sao_Paulo";
  i18n.defaultLocale = lib.mkDefault "en_US.UTF-8";

  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  networking.networkmanager.enable = true;

  nix.settings.experimental-features = [ "nix-command" "flakes" ];
  nixpkgs.config.allowUnfree = true;
  nixpkgs.config.nvidia.acceptLicense = true;

  users.users.${userConfig.username} = {
    isNormalUser = true;
    description = userConfig.username;
    extraGroups = [ "wheel" "networkmanager" "video" "audio" ];
    initialPassword = lib.mkDefault "changeme";
    shell = pkgs.nushell;
  };

  environment.shells = [ pkgs.bash pkgs.nushell ];

  users.users.root.initialPassword = "changeme";

  environment.systemPackages = with pkgs; [
    git
    wget
    nushell
    home-manager
    starship
    amber
  ];

  programs.nix-ld.enable = true;
}
