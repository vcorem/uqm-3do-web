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
python3 "$root/scripts/prepare-emscripten-proxy.py" wasm/compat
emcc -c -E - -s USE_ZLIB=1 -s USE_LIBPNG=1 -s USE_VORBIS=1 -s USE_SDL=2 -pthread </dev/null >/dev/null
cp wasm/config.state.expected config.state
emconfigure ./build.sh uqm config </dev/null
# Fail if the menu silently substitutes an unavailable build choice.
diff -u wasm/config.state.expected <(grep "^CHOICE_" config.state)
# Refresh the source list when configuration enables the netplay subtree.
./build.sh uqm depend
# The upstream makefile does not track the browser shell and pre-JS as link inputs.
# Relink on every package build so browser-only edits cannot produce stale output.
rm -f uqm-debug.html
./build.sh uqm -j"${JOBS:-4}"
output="${BUILD_OUTPUT:-$root/dist}"
mkdir -p "$output"
cp uqm-debug.html "$output/index.html"
cp wasm/compat/toolchain-fixes.json "$output/toolchain-fixes.json"
for artifact in uqm-debug.js uqm-debug.wasm uqm-debug.data uqm-debug.worker.js; do
  cp "$artifact" "$output/$artifact"
done
cp COPYING "$output/COPYING.txt"
cp wasm/netplay-client.js "$output/netplay-client.js"
cp "$root/web/game-help/game-help.js" "$output/game-help.js"
echo "Built $output/index.html"
echo "Preview: python3 scripts/serve.py"
