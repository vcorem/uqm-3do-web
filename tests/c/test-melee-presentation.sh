#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../.." && pwd)"
source_dir="${HD_SOURCE_DIR:-$root/.tools/hd-source}"
export EMSDK_QUIET=1
source "$root/.tools/emsdk/emsdk_env.sh"
output="$root/.tools/melee-presentation-tests"
mkdir -p "$output"
cd "$source_dir"
emcc -std=gnu99 -pthread -sUSE_SDL=2 -DGFXMODULE_SDL -DSDL_DIR=SDL2 \
  -DHAVE_OPENAL -DUSE_INTERNAL_MIKMOD -DUSE_INTERNAL_LUA -DNETPLAY=NETPLAY_FULL \
  -DHAVE_JOYSTICK -DHAVE_ZIP=1 -DUSE_PLATFORM_ACCEL -DTHREADLIB_PTHREAD -I. -Isrc \
  -sENVIRONMENT=node -sEXIT_RUNTIME=1 "$root/tests/c/melee-presentation.c" \
  src/libs/graphics/melee_presentation.c -o "$output/captain-registry.cjs"
node "$output/captain-registry.cjs"
