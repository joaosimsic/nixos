#!/usr/bin/env bash

HOSTNAME=$(hostname)
HOST_DIR="hosts/$HOSTNAME"

if [ -d "$HOST_DIR" ]; then
  echo "Host '$HOSTNAME' already exists at $HOST_DIR"
  echo "If you want to regenerate hardware config, run:"
  echo "  nixos-generate-config --show-hardware-config > $HOST_DIR/hardware-configuration.nix"
  exit 1
fi

echo "Initializing new host: $HOSTNAME"

mkdir -p "$HOST_DIR"

nixos-generate-config --show-hardware-config > "$HOST_DIR"/hardware-configuration.nix

cp hosts/nixos/configuration.nix "$HOST_DIR"/configuration.nix

git add "$HOST_DIR"

echo ""
echo "Created $HOST_DIR with:"
echo "  - hardware-configuration.nix (auto-generated)"
echo "  - configuration.nix (copied from nixos template)"
echo ""
echo "Next steps:"
echo "  1. Edit $HOST_DIR/configuration.nix for this host"
echo "  2. Add '$HOSTNAME' to the 'hosts' attrset in flake.nix"
echo "  3. Run ./apply.sh to build and switch"
