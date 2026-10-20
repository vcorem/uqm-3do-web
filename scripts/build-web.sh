#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
if [[ -f "$root/.tools/emsdk/emsdk_env.sh" ]]; then
  set +u
  source "$root/.tools/emsdk/emsdk_env.sh" >/dev/null
  set -u
fi
required="$(cat "$root/.emscripten-version")"
if ! command -v emcc >/dev/null; then
  echo "Emscripten $required is required. See README.md for setup." >&2
  exit 1
fi
version="$(emcc --version | head -n 1)"
if [[ "$version" != *" $required "* ]]; then
  echo "Expected Emscripten $required; found: $version" >&2
  exit 1
fi
cd "$root/sc2"
case "${UQM_CORE:-rust}" in
  rust)
    export UQM_RUST_CORE=0
    unset UQM_RUST_LIB
    ;;
  hybrid)
    "$root/scripts/build-rust-core.sh" original
    export UQM_RUST_CORE=1
    export UQM_RUST_LIB="$root/.tools/rust-original/wasm32-unknown-emscripten/release/libuqm_core.a"
    ;;
  c) export UQM_RUST_CORE=0; unset UQM_RUST_LIB ;;
  *) echo 'UQM_CORE must be rust, hybrid or c.' >&2; exit 1 ;;
esac
python3 "$root/scripts/prepare-emscripten-proxy.py" wasm/compat
emcc -c -E - -s USE_ZLIB=1 -s USE_LIBPNG=1 -s USE_VORBIS=1 -s USE_SDL=2 -pthread </dev/null >/dev/null
cp wasm/config.state.expected config.state
emconfigure ./build.sh uqm config </dev/null
# Fail if the menu silently substitutes an unavailable build choice.
diff -u wasm/config.state.expected <(grep "^CHOICE_" config.state)
output="${BUILD_OUTPUT:-$root/dist}"
if [[ "${UQM_CORE:-rust}" = rust ]]; then
  mkdir -p "$root/.tools"
  "$root/scripts/rust-engine/build.sh" original >"$root/.tools/rust-engine-original-build.log" 2>&1 || {
    tail -n 100 "$root/.tools/rust-engine-original-build.log" >&2; exit 1;
  }
  python3 "$root/scripts/rust-engine/link.py" original --source "$root/sc2" \
    --generated "$root/rust/uqm-engine-original" \
    --library "${CARGO_TARGET_DIR:-$root/.tools/rust-engine-original}/wasm32-unknown-emscripten/release/libuqm_engine_original.a" \
    --output "$output"
  exit 0
fi
# Refresh the source list when configuration enables the netplay subtree.
./build.sh uqm depend
# The upstream makefile does not track the browser shell and pre-JS as link inputs.
# Relink on every package build so browser-only edits cannot produce stale output.
rm -f uqm-debug.html
./build.sh uqm -j"${JOBS:-4}"
output="${BUILD_OUTPUT:-$root/dist}"
mkdir -p "$output"
rm -f "$output/link-inputs.json"
cp uqm-debug.html "$output/index.html"
cp wasm/compat/toolchain-fixes.json "$output/toolchain-fixes.json"
for artifact in uqm-debug.js uqm-debug.wasm uqm-debug.data uqm-debug.worker.js; do
  cp "$artifact" "$output/$artifact"
done
cp COPYING "$output/COPYING.txt"
cp wasm/netplay-client.js "$output/netplay-client.js"
cp "$root/web/game-help/game-help.js" "$output/game-help.js"
python3 - "$output/engine-language.json" "${UQM_CORE:-rust}" <<'PY'
import json, sys
with open(sys.argv[1], 'w') as stream:
    json.dump({'core': sys.argv[2], 'edition': 'original',
               'rust_modules': ['rng', 'context_rng', 'integer_sqrt', 'netplay_crc', 'velocity', 'trigonometry', 'queues', 'box_geometry', 'line_clipping', 'dirty_regions']
               if sys.argv[2] == 'hybrid' else [], 'complete_rust_port': False}, stream, indent=2)
    stream.write('\n')
PY
echo "Built $output/index.html"
echo "Preview: python3 scripts/serve.py"
