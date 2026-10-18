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
for patch_file in "$root"/hd/patches/*.patch; do
  if git -C "$source_dir" apply --reverse --check "$patch_file" 2>/dev/null; then
    continue
  fi
  git -C "$source_dir" apply --check "$patch_file"
  git -C "$source_dir" apply "$patch_file"
done
mkdir -p "$source_dir/wasm"
cp "$root/hd/wasm/shell.html" "$source_dir/wasm/shell.html"
cp "$root/hd/wasm/pre.js" "$source_dir/wasm/pre.js"
fetch_args=(--output "$source_dir/content")
if [[ "${HD_OFFLINE:-0}" = 1 ]]; then fetch_args+=(--offline); fi
python3 "$root/scripts/fetch-hd-content.py" "${fetch_args[@]}"

cd "$source_dir"
python3 "$root/scripts/prepare-emscripten-proxy.py" wasm/compat
emcc -c -E - -s USE_ZLIB=1 -s USE_LIBPNG=1 -s USE_VORBIS=1 -s USE_SDL=2 -pthread </dev/null >/dev/null
cp wasm/config.state.expected config.state
emconfigure ./build.sh uqm config </dev/null
diff -u wasm/config.state.expected <(grep '^CHOICE_' config.state)
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
cp uqm-debug.html "$output/index.html"
cp wasm/compat/toolchain-fixes.json "$output/toolchain-fixes.json"
for artifact in uqm-debug.js uqm-debug.wasm uqm-debug.data uqm-debug.worker.js; do
  cp "$artifact" "$output/$artifact"
done
cp content/notices/upstream/licenses/COPYING-UQM "$output/COPYING.txt"
cp content/content-licenses.txt "$output/content-licenses.txt"
cp "$root/hd/content-lock.json" "$output/content-lock.json"
echo "Built native 1280×960 HD browser runtime: $output/index.html"
