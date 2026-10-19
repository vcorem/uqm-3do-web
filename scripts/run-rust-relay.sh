#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
relay="$(bash "$root/scripts/build-rust-relay.sh")"
cd "$root"
exec "$relay" "$@"
