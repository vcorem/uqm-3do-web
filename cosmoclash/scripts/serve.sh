#!/usr/bin/env bash
set -euo pipefail
cosmo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ ! -f "$cosmo_root/dist/index.html" ]]; then
  echo 'Build Cosmo Clash first with npm run build.' >&2
  exit 1
fi
cd "$cosmo_root/.."
exec bash scripts/run-rust-relay.sh --dist "$cosmo_root/dist" "$@"
