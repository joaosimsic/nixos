{ pkgs }:

pkgs.writeShellScriptBin "amber" ''
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

        if [ "$app_name" = "claude-code" ]; then
          mkdir -p "$TARGET"
          for file in "$config_dir"/*; do
            if [ -f "$file" ]; then
              ln -sfn "$file" "$TARGET/$(basename "$file")"
              echo " -> Linked claude/$(basename "$file")"
            fi
          done
        else
          ln -sfn "$config_dir" "$TARGET"
          echo " -> Linked $target_name"
        fi
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
''
