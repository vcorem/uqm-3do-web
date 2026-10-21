#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ ! -s "$root/.tools/afterlight-dist/index.html" ]]; then
  echo 'Build the campaign first with npm run build:afterlight.' >&2
  exit 1
fi
exec bash "$root/scripts/run-rust-relay.sh" --dist "$root/.tools/afterlight-dist" --port 10002 "$@"
