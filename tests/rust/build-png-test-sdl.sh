#!/usr/bin/env bash
# Build only an isolated test dependency; the browser uses pinned Emscripten SDL.
set -euo pipefail
root="$(cd "$(dirname "$0")/../.." && pwd)"
base="$root/.tools/rust-png-sdl"
source_dir="$root/.tools/emsdk/upstream/emscripten/cache/ports/sdl2/SDL-release-2.24.2"
if [[ ! -f "$source_dir/configure" ]]; then
  echo 'Run the pinned Emscripten SDL2 build first to populate its source cache.' >&2
  exit 1
fi
mkdir -p "$base/sdl-build"
cd "$base/sdl-build"
bash "$source_dir/configure" --prefix="$base/sdl-native" \
  --disable-shared --enable-static --disable-video-cocoa --disable-video-metal \
  --disable-render-metal --disable-audio --disable-joystick --disable-haptic \
  --disable-sensor --disable-power --disable-video-opengl --disable-video-opengles \
  > configure.log 2>&1
make -j"${JOBS:-8}" > build.log 2>&1
make install > install.log 2>&1
