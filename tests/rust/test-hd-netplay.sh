#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../.." && pwd)"
source "$root/scripts/rust-env.sh"
if [[ -f "$root/.tools/emsdk/emsdk_env.sh" ]]; then
  set +u
  source "$root/.tools/emsdk/emsdk_env.sh" >/dev/null
  set -u
fi
exec python3 "$root/tests/rust/hd-netplay.py" "$@"
