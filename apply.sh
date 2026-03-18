#!/usr/bin/env bash

nixos-generate-config --show-hardware-config > hardware-configuration.nix

git add hardware-configuration.nix

sudo nixos-rebuild boot --flake .#nixos

echo "Done! Reboot to test the new hardware config."
