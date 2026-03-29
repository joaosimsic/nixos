{ pkgs }:

let
  amber-cli = pkgs.rustPlatform.buildRustPackage {
    pname = "amber-cli";
    version = "0.1.0";
    src = ../tools;
    cargoLock.lockFile = ../tools/Cargo.lock;
  };
in
pkgs.writeShellScriptBin "amber" ''
  AMBER_ROOT="/home/joao/.config/amber"
  HOSTNAME=$(hostname)
  
  get_target_name() {
    case "$1" in
      hyprland) echo "hypr" ;;
      *)        echo "$1" ;;
    esac
  }

  needs_special_sync() {
    case "$1" in
      hyprland|waybar) return 0 ;;
      *) return 1 ;;
    esac
  }

  get_monitor() {
    local monitor=$1  
    local field=$2    
    local file="$AMBER_ROOT/hosts/$HOSTNAME/monitors.nix"
    
    if [ ! -f "$file" ]; then
      return 1
    fi
    
    case "$field" in
      name)
        ${pkgs.gnugrep}/bin/grep -A5 "$monitor = {" "$file" | ${pkgs.gnugrep}/bin/grep "name = " | head -1 | ${pkgs.gnused}/bin/sed 's/.*"\(.*\)".*/\1/'
        ;;
      resolution)
        ${pkgs.gnugrep}/bin/grep -A5 "$monitor = {" "$file" | ${pkgs.gnugrep}/bin/grep "resolution = " | head -1 | ${pkgs.gnused}/bin/sed 's/.*"\(.*\)".*/\1/'
        ;;
      refreshRate)
        ${pkgs.gnugrep}/bin/grep -A5 "$monitor = {" "$file" | ${pkgs.gnugrep}/bin/grep "refreshRate = " | head -1 | ${pkgs.gnused}/bin/sed 's/.*= \([0-9]*\).*/\1/'
        ;;
    esac
  }

  sync_hyprland() {
    local config_dir="$AMBER_ROOT/domains/wm/hyprland/config"
    local target="/home/joao/.config/hypr"

    if [ -L "$target" ]; then
      rm "$target"
    fi
    
    mkdir -p "$target"
    
    for file in "$config_dir"/*; do
      local basename=$(basename "$file")
      if [ "$basename" != "monitors.conf" ]; then
        ln -sfn "$file" "$target/$basename"
      fi
    done
    
    local primary_name=$(get_monitor primary name)
    local primary_res=$(get_monitor primary resolution)
    local primary_rate=$(get_monitor primary refreshRate)
    local secondary_name=$(get_monitor secondary name)
    local secondary_res=$(get_monitor secondary resolution)
    local secondary_rate=$(get_monitor secondary refreshRate)
    
    if [ -n "$primary_name" ]; then
      cat > "$target/monitors.conf" << EOF
monitor = $primary_name, $primary_res@$primary_rate, 0x0, 1
monitor = $secondary_name, $secondary_res@$secondary_rate, 1920x0, 1

workspace = 1, monitor:$primary_name
workspace = 2, monitor:$primary_name
workspace = 3, monitor:$primary_name
workspace = 4, monitor:$primary_name
workspace = 5, monitor:$primary_name
workspace = 6, monitor:$secondary_name
workspace = 7, monitor:$secondary_name
workspace = 8, monitor:$secondary_name
workspace = 9, monitor:$secondary_name
workspace = 10, monitor:$secondary_name
EOF
      echo " -> hypr (with generated monitors.conf)"
    else
      echo " ! hypr: Could not read monitors for host '$HOSTNAME'"
    fi
  }

  sync_waybar() {
    local config_dir="$AMBER_ROOT/domains/bar/waybar/config"
    local target="/home/joao/.config/waybar"
    
    if [ -L "$target" ]; then
      rm "$target"
    fi

    mkdir -p "$target"
    
    ln -sfn "$config_dir/style.css" "$target/style.css"
    ln -sfn "$config_dir/colors.css" "$target/colors.css"
    
    local primary_name=$(get_monitor primary name)
    local secondary_name=$(get_monitor secondary name)
    
    if [ -n "$primary_name" ] && [ -f "$config_dir/config" ]; then
      ${pkgs.gnused}/bin/sed \
        -e "s/\"output\": \"[^\"]*\"/\"output\": \"$primary_name\"/1" \
        "$config_dir/config" | \
      ${pkgs.python3}/bin/python3 -c "
import sys, json
data = json.load(sys.stdin)
data[0]['output'] = '$primary_name'
data[0]['hyprland/workspaces']['persistent-workspaces'] = {'$primary_name': [1,2,3,4,5]}
if len(data) > 1:
    data[1]['output'] = '$secondary_name'
    data[1]['hyprland/workspaces']['persistent-workspaces'] = {'$secondary_name': [6,7,8,9,10]}
print(json.dumps(data, indent=2))
" > "$target/config"
      echo " -> waybar (with generated config)"
    else
      echo " ! waybar: Could not read monitors for host '$HOSTNAME'"
    fi
  }

  sync_configs() {
    echo "Syncing configurations for host '$HOSTNAME'..."
    echo ""
    
    for config_dir in "$AMBER_ROOT"/domains/*/*/config "$AMBER_ROOT"/capabilities/*/config; do
      if [ -d "$config_dir" ]; then
        app_name=$(basename "$(dirname "$config_dir")")
        target_name=$(get_target_name "$app_name")
        target="/home/joao/.config/$target_name"
        
        if needs_special_sync "$app_name"; then
          case "$app_name" in
            hyprland) sync_hyprland ;;
            waybar) sync_waybar ;;
          esac
          continue
        fi
        
        if [ -L "$target" ]; then
          rm "$target"
        elif [ -d "$target" ]; then
          echo " ! $target_name exists as directory, skipping"
          continue
        fi
        
        ln -sfn "$config_dir" "$target"
        echo " -> $target_name"
      fi
    done
    
    if [ -f "$AMBER_ROOT/domains/shell/nushell/config/starship.toml" ]; then
      ln -sfn "$AMBER_ROOT/domains/shell/nushell/config/starship.toml" "/home/joao/.config/starship.toml"
      echo " -> starship.toml"
    fi
    
    echo ""
    echo "Done. Configs synced."
  }

  status_configs() {
    echo "Amber Config Status (host: $HOSTNAME)"
    echo "======================================="
    echo ""
    
    for config_dir in "$AMBER_ROOT"/domains/*/*/config "$AMBER_ROOT"/capabilities/*/config; do
      if [ -d "$config_dir" ]; then
        app_name=$(basename "$(dirname "$config_dir")")
        target_name=$(get_target_name "$app_name")
        target="/home/joao/.config/$target_name"
        
        if needs_special_sync "$app_name"; then
          if [ -d "$target" ]; then
            echo " [ok] $target_name (mixed: symlinks + generated)"
          else
            echo " [--] $target_name (not synced)"
          fi
        elif [ -L "$target" ]; then
          link_target=$(readlink "$target")
          if [ "$link_target" = "$config_dir" ]; then
            echo " [ok] $target_name"
          else
            echo " [!!] $target_name (wrong target)"
          fi
        elif [ -d "$target" ]; then
          echo " [!!] $target_name (directory, not synced)"
        else
          echo " [--] $target_name (not linked)"
        fi
      fi
    done
  }

  COMMAND=$1

  case "$COMMAND" in
    theme)
      exec ${amber-cli}/bin/amber "$@"
      ;;
    sync)
      sync_configs
      ;;
    status)
      status_configs
      ;;
    *)
      echo "Amber - Config Management"
      echo ""
      echo "Usage: amber <command>"
      echo ""
      echo "Commands:"
      echo "  theme  - Set color theme"
      echo "  sync   - Link repo configs + generate host-specific files"
      echo "  status - Show sync status of all configs"
      echo ""
      echo "Versioning: Use git directly in ~/.config/amber/"
      echo "  git diff           - See uncommitted changes"
      echo "  git commit -am 'message'  - Save a checkpoint"
      echo "  git log --oneline  - List checkpoints"
      echo "  git checkout <id> -- domains/<app>/config  - Restore"
      ;;
  esac
''
