#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
BUILD_OUTPUT="$root/.tools/original-dist" "$root/scripts/build-web.sh"
HD_BUILD_OUTPUT="$root/.tools/hd-dist" BUILD_OUTPUT="$root/.tools/hd-dist" "$root/scripts/build-hd-web.sh"
python3 "$root/scripts/package-editions.py" \
  --original "$root/.tools/original-dist" \
  --hd "$root/.tools/hd-dist" \
  --output "${BUILD_OUTPUT:-$root/dist}"
