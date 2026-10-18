#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../../../../../.." && pwd)"
work="$(mktemp -d "${TMPDIR:-/tmp}/uqm-transport-test.XXXXXX")"
trap 'rm -rf "$work"' EXIT
emcc="${EMCC:-$root/.tools/emsdk/upstream/emscripten/emcc}"
# Use the current generated platform configuration without changing it.
sed 's/^#undef NETPLAY/#define NETPLAY 2/' "$root/sc2/config_unix.h" > "$work/config_unix.h"
for mode in direct pthread; do
  flags=(-sASSERTIONS=1)
  if [[ "$mode" == pthread ]]; then
    flags+=(-pthread -sPROXY_TO_PTHREAD=1 -sPTHREAD_POOL_SIZE=1 -sEXIT_RUNTIME=1)
  fi
  "$emcc" "${flags[@]}" -DNETPLAY=2 -I"$work" -I"$root/sc2" -I"$root/sc2/src" \
  "$root/sc2/src/uqm/supermelee/netplay/tests/transport.c" \
  -sSINGLE_FILE=1 -sENVIRONMENT=node -sASSERTIONS=1 -O0 -o "$work/$mode.js"
  node "$work/$mode.js"
done
"$emcc" -DNETPLAY=2 -I"$work" -I"$root/sc2" -I"$root/sc2/src" \
  "$root/sc2/src/uqm/supermelee/netplay/tests/packet_bounds.c" \
  -sSINGLE_FILE=1 -sENVIRONMENT=node -sASSERTIONS=1 -O0 -o "$work/bounds.js"
node "$work/bounds.js"

"$emcc" -DNETPLAY=2 -I"$work" -I"$root/sc2" -I"$root/sc2/src" \
  "$root/sc2/src/uqm/supermelee/netplay/tests/disconnect.c" \
  -sSINGLE_FILE=1 -sENVIRONMENT=node -sASSERTIONS=1 -O0 -o "$work/disconnect.js"
node "$work/disconnect.js"
"$emcc" -DNETPLAY=2 -I"$work" -I"$root/sc2" -I"$root/sc2/src" \
  "$root/sc2/src/uqm/supermelee/netplay/tests/setup_disconnect.c" \
  -sSINGLE_FILE=1 -sENVIRONMENT=node -sASSERTIONS=1 -O0 -o "$work/setup-disconnect.js"
node "$work/setup-disconnect.js"
