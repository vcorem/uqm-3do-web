#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
if [[ "${1:-}" != --ui-only ]]; then
  bash scripts/build-afterlight-combat.sh
fi
python3 campaign/tools/package.py
