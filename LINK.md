# Amber — Package Config Management

## Overview

Amber uses a **two-layer architecture**:

1. **Nix (build-time)** — NixOS system configs + home-manager modules
2. **Amber CLI (runtime sync)** — custom Rust tool that deploys per-app config files

Result: Nix handles **package installation + system config**. Amber handles **app config file deployment** (symlinks, templates, theme injection).

---

## Directory Structure

```
flake.nix                   # Entry point: NixOS + home-manager builds
home.nix                    # home-manager root
theme                       # Single hex color (e.g. "base = #EEEEEE")
apply.sh                    # nixos-rebuild + amber sync
core/                       # NixOS-level system config
  default.nix               #   boot, network, users, base packages
  home.nix                  #   home-manager base (cursor, fonts)
  amber.nix                 #   builds amber CLI from tools/
  audio.nix                 #   pipewire
  monitors.nix              #   NixOS option types for monitor config
hosts/                      # Per-host config
  <hostname>/
    default.nix             #   system, username, homeDirectory, monitors
    configuration.nix       #   hardware-specific NixOS config
    hardware-configuration.nix
    monitors.nix            #   { primary, secondary } monitor definitions
capabilities/               # General-purpose tooling
  fuzzy.nix, clipboard.nix, files.nix, networking.nix, archive.nix
  git/                      #   lazygit (has config/ subdir)
  containers/               #   lazydocker (has config/ subdir)
toolchains/                 # Language runtimes/build tools
  node.nix, python.nix, rust.nix, go.nix, c.nix, lua.nix, nix.nix,
  java.nix, php.nix, javascript.nix, docker.nix, bash.nix,
  terraform.nix, xml.nix, csharp.nix
profiles/
  default.nix               # Master import: all capabilities + all domains
domains/                    # Organized by domain
  wm/hyprland/
    config/                 #   Config files (symlinked to ~/.config/hypr/)
    default.nix             #   Adds hyprland/hyprpaper/hypridle/etc packages
    system.nix              #   NixOS module: enables hyprland
  terminal/ghostty/
    config/config           #   Ghostty config
    default.nix             #   Adds ghostty package
  terminal/zellij/
    config/config.kdl       #   Zellij config (with layouts/)
    default.nix             #   Adds zellij package
  shell/nushell/
    config/config.nu        #   Nushell config
    config/env.nu
    config/colors.nu
    config/starship.toml    #   Starship config (special case — symlinked separately)
    default.nix             #   Adds nushell + starship packages
  bar/waybar/
    config/config           #   Waybar bar config (JSON, injected w/ monitor names)
    config/style.css
    config/colors.css       #   Generated from theme template
    default.nix             #   Adds waybar package
  launcher/rofi/
    config/config.rasi
    config/theme.rasi        #   Generated from theme template
    default.nix             #   Adds rofi package
  notifications/mako/
    config/config
    default.nix             #   Adds mako + libnotify packages
  editor/nvim/
    config/init.lua         #   Full neovim config tree
    default.nix             #   programs.neovim.enable
  assistant/claude/
    config/*                #   Claude Code configs
    default.nix             #   Adds claude-code package
  assistant/opencode/
    config/*                #   Opencode configs
    default.nix             #   (imported via profiles)
tools/                      # Amber CLI source (Rust)
  src/main.rs               #   Commands: theme, sync, status, grave
  src/commands/sync.rs      #   Walks tree, symlinks config/ dirs
  src/commands/theme/       #   Palette generation, template rendering, reload
  src/commands/monitors.rs  #   Reads monitors.nix, generates hyprland/waybar config
  src/commands/grave/       #   Zellij session manager
  src/amber_dir.rs          #   Resolves amber root (~/.config/amber or $AMBER_DIR)
```

---

## Layer 1: Nix — Build-time Package Management

### How Packages Are Added

Every domain has a `default.nix` that is a **home-manager module**. These install packages:

```nix
# domains/terminal/zellij/default.nix
{ config, pkgs, ... }: {
  home.packages = with pkgs; [ zellij ];
}
```

Some domains also have a **NixOS module** (e.g. `system.nix`) for system-level enablement:

```nix
# domains/wm/hyprland/system.nix
{ config, lib, pkgs, ... }: {
  programs.hyprland.enable = true;
  services.displayManager.sddm.enable = true;
}
```

### The Import Chain

```
flake.nix
  -> core/default.nix               (system: boot, users, keyboard, network)
  -> core/home.nix                  (home-manager base)
  -> home.nix
       -> profiles/default.nix      (master import list)
            -> capabilities/*       (fuzzy, clipboard, containers, git…)
            -> toolchains/*         (node, rust, python, go…)
            -> domains/*            (hyprland, zellij, nvim, waybar…)
```

### Host Configs

Each host in `hosts/<name>/` defines:
- `default.nix` — `system`, `user` (username, homeDirectory), `monitors`
- `configuration.nix` — imports `hardware-configuration.nix`, sets hostname, GPU drivers, kernel params
- `monitors.nix` — primary/secondary monitor definitions used by NixOS option type in `core/monitors.nix`

The NixOS option type (`core/monitors.nix`) defines a typed interface:

```nix
options.monitors.primary = { name, resolution, refreshRate };
options.monitors.secondary = { name, resolution, refreshRate };
```

This is consumed at runtime by Amber CLI, not by Nix directly.

### `./apply.sh`

```bash
sudo nixos-rebuild switch --flake .#"$HOSTNAME"  # build NixOS
amber sync                                        # deploy app configs
```

For user-only changes: `home-manager switch` then `amber sync`.

---

## Layer 2: Amber CLI — Runtime Config File Deployment

### Config File Discovery

The amber `sync` command (`tools/src/commands/sync.rs`) walks the entire project tree looking for directories named `config/`. When found:

1. Determines target name from parent directory name
2. Removes existing `~/.config/<target>/` (symlink or dir)
3. Creates symlink: `~/.config/<target>/` → `<project>/domains/<app>/config/`

**Name mapping:**
| Source dir | Target |
|---|---|
| `hyprland/` | `~/.config/hypr/` |
| `git/` | `~/.config/lazygit/` |
| everything else | `~/.config/<name>/` |

**Special handling for `hyprland`:**
- Removes old symlink, creates directory
- Symlinks each file individually (skips `monitors.conf` and `*.template` files)
- **Generates** `monitors.conf` from `hosts/<hostname>/monitors.nix` (parsed at runtime)

**Special handling for `waybar`:**
- Creates directory, symlinks `style.css` and `colors.css` individually
- **Injects** monitor names into the JSON `config` file (sets `output` and `persistent-workspaces`)

**Special standalone star:** `starship.toml` is symlinked from `domains/shell/nushell/config/starship.toml` to `~/.config/starship.toml` even though nushell's config dir is `config/`.

### Configs Deployed via Symlinks

| ~/.config/ target | Source |
|---|---|
| `hypr/` | `domains/wm/hyprland/config/` (files symlinked individually, monitors.conf generated) |
| `ghostty/` | `domains/terminal/ghostty/config/` |
| `zellij/` | `domains/terminal/zellij/config/` |
| `nushell/` | `domains/shell/nushell/config/` |
| `waybar/` | `domains/bar/waybar/config/` (config injected w/ monitors, style.css + colors.css symlinked) |
| `rofi/` | `domains/launcher/rofi/config/` |
| `mako/` | `domains/notifications/mako/config/` |
| `lazygit/` | `capabilities/git/config/` |
| `lazydocker/` | `capabilities/containers/config/` |
| `nvim/` | `domains/editor/nvim/config/` |
| `claude/` | `domains/assistant/claude/config/` |
| `opencode/` | `domains/assistant/opencode/config/` |
| `starship.toml` | `domains/shell/nushell/config/starship.toml` (direct file symlink) |

---

## Theme System

### How It Works

1. Root `theme` file stores one hex color: `base = #EEEEEE`
2. `amber theme <color>` writes to `theme`, then:
   - Reads the hex, generates a full **palette** (base, bright, dim, surface, black, red, green, yellow, blue, magenta, cyan + bright variants)
   - Palette is derived algorithmically using HLS color space transformations from the base color
   - Writes `palette.json` (all hex values + RGB decimal variants) — consumed by both template rendering and tools like `amber grave`
   - Finds every `*.template` file in the tree, renders `{{KEY}}` → palette value
   - Removes `.template` extension on output (e.g. `colors.conf.template` → `colors.conf`)
3. After render, calls `reload_all()` to hot-reload running apps

### Template Placeholders

| Placeholder Family | Examples | Format |
|---|---|---|
| Named colors | `{{BASE}}`, `{{BRIGHT}}`, `{{DIM}}`, `{{SURFACE}}`, `{{BG}}`, `{{BLACK}}`, `{{RED}}` | `eeeeee` (hex, no `#`) |
| RGB decimal variants | `{{BASE_RGB}}`, `{{BRIGHT_RGB}}`, `{{DIM_RGB}}` | `238 238 238` (space-separated decimal) |
| P0-P15 | `{{P0}}`-`{{P15}}` | Standard terminal palette indices |

### Template Files

| Template | Output |
|---|---|
| `config/colors.conf.template` | Hyprland color variables |
| `config/theme.rasi.template` | Rofi theme colors |
| `config/colors.nu.template` | Nushell color definitions |
| `config/colors.css.template` | Waybar CSS color vars |
| `config/config.kdl.template` | Zellij KDL theme |
| `config/starship.toml.template` | Starship prompt colors |
| `config/theme.lua.template` | Neovim Lua theme |
| `config/config.yml.template` | Lazygit config theme |
| `config/config.template` | Mako config theme |

### Hot Reload (on `amber theme`)

After theme change, running apps are notified:

| App | Method |
|---|---|
| Hyprland | `hyprctl keyword` (borders, background) + `hyprctl reload` |
| Waybar | `pkill waybar` → restart |
| Mako | `makoctl reload` |
| Ghostty | `pkill -USR2 ghostty` |
| Zellij | cache clear + sessions killed (or current session quit-triggers-reattach) |
| Neovim | `nvr -c "luafile theme.lua"` |
| Foot | `pkill -USR1 foot` |
| Kitty | `pkill -USR1 kitty` |
| Wezterm | `wezterm cli reload` |
| Dunst | `dunstctl reload` |
| Swaync | `swaync-client -R` |
| Eww | `eww reload` |

---

## Workflow Summary

### Initial Setup (new host)

```bash
./apply.sh   # detects unknown host, runs init-host.sh
             # generates hardware-config + copies template
             # edit configuration.nix, add to flake.nix
./apply.sh   # builds + switches + amber sync
```

### Add a new package

1. Create `domains/<app>/default.nix` with `home.packages = [ pkgs.<app> ];`
2. Add config files to `domains/<app>/config/`
3. If it needs theme support, create a `.template` file
4. Import the domain in `profiles/default.nix`
5. `home-manager switch && amber sync`

### Change theme

```bash
amber theme #ff6600     # writes theme file, regenerates palette,
                        # renders all templates, hot-reloads apps
amber theme             # read existing theme and re-render templates
amber theme #ff6600 --dry-run  # preview without writing
```

### Check sync status

```bash
amber status   # shows which configs are linked correctly
```

### Versioning

Config files live in this git repo alongside the Nix config. The amber CLI `--help` notes to use git directly for versioning. They are deployed as symlinks, so edits in `~/.config/<app>/` are edits in the repo.

---

## Key Design Decisions

- **Nix installs packages, Amber deploys configs** — keeps Nix modules pure (just package + program enablement), config management in Rust where it's easier to parse monitors.nix and run shell commands
- **Symlinks, not copies** — edits in `~/.config/` are edits in the repo; no separate sync step needed for config changes
- **Generated configs (monitors.nix → hyprland.conf, waybar.json)** — avoids per-host config duplication for monitor-dependent settings
- **Template system separate from Nix** — avoids Nix string interpolation for config files that need runtime theme values; templates work on any file format
- **Theme is a single hex color** — full palette is algorithmically derived, no need to manage 20+ color vars manually
