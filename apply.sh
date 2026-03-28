#!/usr/bin/env bash

HOSTNAME=$(hostname)
HOST_DIR="hosts/$HOSTNAME"

if [ ! -d "$HOST_DIR" ]; then
  echo "Host '$HOSTNAME' not found. Initializing..."
  ./init-host.sh
  echo ""
  echo "Please edit $HOST_DIR/configuration.nix and add '$HOSTNAME' to flake.nix, then run ./apply.sh again."
  exit 0
fi

sudo nixos-rebuild switch --flake .#"$HOSTNAME"
