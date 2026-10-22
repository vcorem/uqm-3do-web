#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../.." && pwd)"
source_dir="${HD_SOURCE_DIR:-$root/.tools/hd-source}"
headers="${HD_HEADER_DIR:-$root/.tools/hd-source}"
export EMSDK_QUIET=1
source "$root/.tools/emsdk/emsdk_env.sh"
output="$(mktemp -d "${TMPDIR:-/tmp}/uqm-melee-only-test.XXXXXX")"
trap 'rm -rf "$output"' EXIT
emcc -std=gnu99 -pthread -sUSE_SDL=2 -DGFXMODULE_SDL -DSDL_DIR=SDL2 \
  -DHAVE_OPENAL -DUSE_INTERNAL_MIKMOD -DUSE_INTERNAL_LUA -DNETPLAY=NETPLAY_FULL \
  -DHAVE_JOYSTICK -DHAVE_ZIP=1 -DUSE_PLATFORM_ACCEL -DTHREADLIB_PTHREAD \
  -I"$source_dir/src" -I"$headers" -I"$headers/src" -I"$headers/src/uqm" \
  -sENVIRONMENT=node -sEXIT_RUNTIME=1 "$root/tests/c/melee-only-profile.c" \
  -o "$output/profile.cjs"
node "$output/profile.cjs" classic
node "$output/profile.cjs"
node "$output/profile.cjs" network
