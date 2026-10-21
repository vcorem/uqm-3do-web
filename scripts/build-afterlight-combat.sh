#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
export UQM_CORE=rust UQM_CAMPAIGN_COMBAT=1
export BUILD_OUTPUT="${BUILD_OUTPUT:-$root/.tools/afterlight-combat}"
if [[ "${AFTERLIGHT_COMBAT_TEST_DRIVER:-0}" = 1 ]]; then
  export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$root/.tools/rust-afterlight-test}"
fi
bash "$root/scripts/build-web.sh"
echo "Afterlight native UQM combat: $BUILD_OUTPUT"
