#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
source_dir="${HD_SOURCE_DIR:-$root/.tools/hd-source}"
output="${HD_BUILD_OUTPUT:-${BUILD_OUTPUT:-$root/.tools/hd-dist}}"
source_dir="$(python3 -c 'import os,sys; print(os.path.abspath(sys.argv[1]))' "$source_dir")"
output="$(python3 -c 'import os,sys; print(os.path.abspath(sys.argv[1]))' "$output")"
revision='1eece12ff9a89d029a5d1f65ead7c75f9ab78acb'
repository='https://github.com/JHGuitarFreak/UQM-MegaMod.git'

if [[ -f "$root/.tools/emsdk/emsdk_env.sh" ]]; then
  set +u
  source "$root/.tools/emsdk/emsdk_env.sh" >/dev/null
  set -u
fi
required="$(cat "$root/.emscripten-version")"
if ! command -v emcc >/dev/null || [[ "$(emcc --version | head -n 1)" != *" $required "* ]]; then
  echo "Emscripten $required is required. See README.md for setup." >&2
  exit 1
fi
python3 - "$root/hd/content-lock.json" "$revision" <<'PY'
import json, sys
with open(sys.argv[1]) as source:
    lock = json.load(source)
if lock['upstream']['source_commit'] != sys.argv[2]:
    raise SystemExit('HD build source revision does not match the content lock.')
PY
if [[ ! -d "$source_dir/.git" ]]; then
  if [[ -e "$source_dir" ]]; then
    echo "HD_SOURCE_DIR exists but is not a Git checkout: $source_dir" >&2
    exit 1
  fi
  mkdir -p "$(dirname "$source_dir")"
  git init "$source_dir"
  git -C "$source_dir" remote add origin "$repository"
fi
if ! git -C "$source_dir" rev-parse --verify HEAD >/dev/null 2>&1; then
  if [[ "${HD_OFFLINE:-0}" = 1 ]]; then
    echo "The pinned HD source is missing; run once without HD_OFFLINE=1." >&2
    exit 1
  fi
  git -C "$source_dir" fetch --depth 1 "$repository" "$revision"
  git -C "$source_dir" checkout --detach FETCH_HEAD
fi
if [[ "$(git -C "$source_dir" rev-parse HEAD)" != "$revision" ]]; then
  echo "HD source must be pinned to $revision; refusing to reset the existing checkout." >&2
  exit 1
fi
python3 "$root/scripts/apply-hd-patches.py" "$source_dir"
mkdir -p "$source_dir/wasm"
cp "$root/hd/wasm/shell.html" "$source_dir/wasm/shell.html"
cp "$root/hd/wasm/pre.js" "$source_dir/wasm/pre.js"
case "${UQM_CORE:-rust}" in
  rust)
    export UQM_RUST_CORE=0
    unset UQM_RUST_LIB
    ;;
  hybrid)
    "$root/scripts/build-rust-core.sh" hd
    export UQM_RUST_CORE=1
    export UQM_RUST_LIB="$root/.tools/rust-hd/wasm32-unknown-emscripten/release/libuqm_core.a"
    cp "$root/hd/rust/rust_config.c" "$source_dir/src/libs/math/rust_config.c"
    ;;
  c) export UQM_RUST_CORE=0; unset UQM_RUST_LIB ;;
  *) echo 'UQM_CORE must be rust, hybrid or c.' >&2; exit 1 ;;
esac
fetch_args=(--output "$source_dir/content")
if [[ "${HD_OFFLINE:-0}" = 1 ]]; then fetch_args+=(--offline); fi
python3 "$root/scripts/fetch-hd-content.py" "${fetch_args[@]}"

cd "$source_dir"
python3 "$root/scripts/prepare-emscripten-proxy.py" wasm/compat
emcc -c -E - -s USE_ZLIB=1 -s USE_LIBPNG=1 -s USE_VORBIS=1 -s USE_SDL=2 -pthread </dev/null >/dev/null
cp wasm/config.state.expected config.state
emconfigure ./build.sh uqm config </dev/null
diff -u wasm/config.state.expected <(grep '^CHOICE_' config.state)
if [[ "${UQM_CORE:-rust}" = rust ]]; then
  mkdir -p "$root/.tools"
  "$root/scripts/rust-engine/build.sh" hd >"$root/.tools/rust-engine-hd-build.log" 2>&1 || {
    tail -n 100 "$root/.tools/rust-engine-hd-build.log" >&2; exit 1;
  }
  python3 "$root/scripts/rust-engine/link.py" hd --source "$source_dir" \
    --generated "$root/rust/uqm-engine-hd" \
    --library "${CARGO_TARGET_DIR:-$root/.tools/rust-engine-hd}/wasm32-unknown-emscripten/release/libuqm_engine_hd.a" \
    --output "$output"
  python3 - "$root/hd/content-lock.json" "$output/uqm-debug.data" <<'PY'
import json, os, sys
with open(sys.argv[1]) as source:
    expected = sum(package['bytes'] for package in json.load(source)['packages'])
if os.path.getsize(sys.argv[2]) < expected:
    raise SystemExit('HD preload is smaller than its required media packages.')
PY
  exit 0
fi
./build.sh uqm depend
# Upstream does not track shell, pre-JS, or media package changes as link inputs.
# Always recreate the preload metadata and runtime together.
rm -f uqm-debug.html uqm-debug.data
./build.sh uqm -j"${JOBS:-4}"
python3 - "$root/hd/content-lock.json" uqm-debug.data <<'PY'
import json, os, sys
with open(sys.argv[1]) as source:
    lock = json.load(source)
expected = sum(package['bytes'] for package in lock['packages'])
if os.path.getsize(sys.argv[2]) < expected:
    raise SystemExit('HD preload is smaller than its required media packages.')
PY
mkdir -p "$output"
rm -f "$output/link-inputs.json"
cp uqm-debug.html "$output/index.html"
cp wasm/compat/toolchain-fixes.json "$output/toolchain-fixes.json"
for artifact in uqm-debug.js uqm-debug.wasm uqm-debug.data uqm-debug.worker.js; do
  cp "$artifact" "$output/$artifact"
done
cp content/notices/upstream/licenses/COPYING-UQM "$output/COPYING.txt"
cp content/content-licenses.txt "$output/content-licenses.txt"
cp "$root/hd/content-lock.json" "$output/content-lock.json"
cp "$root/web/game-help/game-help.js" "$output/game-help.js"
cp "$root/sc2/wasm/netplay-client.js" "$output/netplay-client.js"
python3 - "$output/engine-language.json" "${UQM_CORE:-rust}" <<'PY'
import json, sys
with open(sys.argv[1], 'w') as stream:
    json.dump({'core': sys.argv[2], 'edition': 'hd',
               'rust_modules': ['rng', 'context_rng', 'integer_sqrt', 'string_crc', 'velocity', 'trigonometry', 'queues', 'box_geometry', 'line_clipping', 'dirty_regions']
               if sys.argv[2] == 'hybrid' else [], 'complete_rust_port': False}, stream, indent=2)
    stream.write('\n')
PY
echo "Built native 1280×960 HD browser runtime: $output/index.html"
