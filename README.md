# Amber

Personal [NixOS](https://nixos.org/) and [Home Manager](https://github.com/nix-community/home-manager) configuration. This repository is the flake source for multiple machines (personal, work, VM) under a single layout: shared **core** modules, per-**host** hardware and system settings, and **profiles** / **domains** for user programs and dotfiles.

## Requirements

- Nix with flakes enabled (`nix.settings.experimental-features` includes `nix-command` and `flakes`, as set in `core/default.nix`)
- For full system rebuilds: NixOS on the target host
- For user-only Home Manager setups: Nix + Home Manager on non-NixOS is possible using the `homeConfigurations` outputs (adjust paths and expectations as needed)

## Repository layout

| Path | Role |
|------|------|
| `flake.nix` | Flake inputs (`nixpkgs`, `home-manager`), `nixosConfigurations`, `homeConfigurations` |
| `core/` | Shared system defaults (audio, locale, boot, user shell, `nix-ld`, etc.) |
| `hosts/<name>/` | Per-machine `configuration.nix`, `hardware-configuration.nix`, monitors, and `default.nix` (system + user metadata) |
| `profiles/default.nix` | Home Manager imports: **capabilities** (git, containers, …), **toolchains**, and **domains** (Hyprland, Ghostty, Zellij, Nushell, Neovim, Waybar, …) |
| `domains/` | Application-specific Nix modules and config templates |
| `capabilities/` | Cross-cutting CLI/tool bundles |
| `toolchains/` | Language/runtime tool profiles |
| `tools/` | Rust CLI `amber` (theme, sync, status) |

The flake passes `amberPath` (currently `/home/joao/.config/amber`) into modules. If you clone this elsewhere or for another user, update that path in `flake.nix`.

## Hosts

Defined in `flake.nix` as `personal`, `work`, and `vm`. Each host’s `hosts/<name>/default.nix` sets `system` (e.g. `x86_64-linux`) and `user` (`username`, `homeDirectory`).

## Usage

From the repository root (or with `--flake /path/to/amber`):

**NixOS system**

```bash
sudo nixos-rebuild switch --flake .#<host>
```

Example: `sudo nixos-rebuild switch --flake .#personal`

**Home Manager**

```bash
home-manager switch --flake .#joao@<host>
```

There is also a `joao` home configuration that tracks the `personal` host.

**Inspect outputs**

```bash
nix flake show
```

## `amber` CLI

The `tools/` package builds a small binary used for config workflows (see `tools/src/main.rs`):

- `amber theme` — set color theme
- `amber sync` — link repo configs and generate host-specific files
- `amber status` — show sync status

After changes under `tools/`, rebuild the system or home environment so the updated `amber` is on your `PATH`.

## Development

- **Rust (`amber`)**: `cd tools && cargo build` / `cargo test`
- **Docker sandbox** (optional): see `Makefile` (`make sandbox`)

## Notes

- Default passwords in `core/default.nix` are placeholders; change them on a real install.
- This README describes the intent of the repo; exact package lists live in the Nix modules under `profiles/`, `domains/`, and `hosts/`.
