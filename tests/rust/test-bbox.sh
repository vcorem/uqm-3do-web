#!/usr/bin/env bash
# Real bbox.c oracle for the screen-thread dirty-region implementation.
set -euo pipefail
root="$(cd "$(dirname "$0")/../.." && pwd)"
source "$root/scripts/rust-env.sh"
cd "$root"
mode="${1:---native}"
edition="${2:---original}"
if [[ "$#" -gt 2 || ( "$mode" != --native && "$mode" != --wasm && "$mode" != --all ) ||
      ( "$edition" != --original && "$edition" != --hd ) ]]; then
  echo "Usage: $0 [--native|--wasm|--all] [--original|--hd]" >&2
  exit 2
fi
edition="${edition#--}"
source_root="$root/sc2/src"
features=(--no-default-features)
environment=(-DUQM_TEST_ORIGINAL=1)
if [[ "$edition" == hd ]]; then
  source_root="${HD_SOURCE_DIR:-$root/.tools/hd-source}/src"
  features+=(--features hd)
  environment=(-DUQM_TEST_HD=1)
fi
environment+=(-include "$root/tests/rust/velocity-environment.h")
# build.sh writes config_unix.h in the source checkout root, beside src/.
includes=("-I$root/tests/rust/include" "-I$source_root" "-I$(dirname "$source_root")")
# Keep this oracle's cargo artifacts independent from simultaneous oracle runs.
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$root/.tools/rust-bbox-cargo/$edition}"
if [[ "$CARGO_TARGET_DIR" != /* ]]; then export CARGO_TARGET_DIR="$root/$CARGO_TARGET_DIR"; fi

run_case() {
  local target="$1" profile="$2" optimization="$3"
  local output="$root/.tools/rust-bbox-oracle/$edition-$target-$profile"
  local compiler="${CC:-cc}" extension="" library_target=""
  local cargo_args=(build --manifest-path "$root/rust/Cargo.toml" -p uqm-core)
  local flags=(-fsanitize=address,undefined -fno-omit-frame-pointer)
  local link_flags=(-fwrapv)
  if [[ "$target" == wasm ]]; then
    compiler=emcc
    extension=.js
    library_target=wasm32-unknown-emscripten/
    cargo_args=("+$(cat "$root/.rust-wasm-toolchain")" build \
      -Zbuild-std=core,compiler_builtins --target wasm32-unknown-emscripten \
      --manifest-path "$root/rust/Cargo.toml" -p uqm-core)
    flags=(-pthread)
    link_flags=(-sENVIRONMENT=node -sSINGLE_FILE=1 -sEXIT_RUNTIME=1 -sASSERTIONS=2 \
      -sINITIAL_MEMORY=33554432 -sSTACK_SIZE=1048576)
  fi
  if [[ "$profile" == release ]]; then cargo_args+=(--release); fi
  if [[ "$target" == wasm ]]; then
    RUSTFLAGS="${RUSTFLAGS:-} -Ctarget-feature=+atomics,+bulk-memory,+mutable-globals -Cpanic=abort" \
      cargo "${cargo_args[@]}" "${features[@]}"
  else
    cargo "${cargo_args[@]}" "${features[@]}"
  fi
  mkdir -p "$output"
  "$compiler" -std=gnu11 -Wall -Wextra -Werror -fwrapv "$optimization" \
    "${flags[@]}" "${includes[@]}" "${environment[@]}" \
    -include "$root/tests/rust/bbox-symbols.h" \
    -c "$source_root/libs/graphics/bbox.c" -o "$output/bbox.o"
  "$compiler" -std=gnu11 -Wall -Wextra -Werror -fwrapv "$optimization" \
    "${flags[@]}" "${includes[@]}" "${environment[@]}" \
    "$root/tests/rust/bbox-oracle.c" "$output/bbox.o" \
    "$CARGO_TARGET_DIR/$library_target$profile/libuqm_core.a" \
    "${link_flags[@]}" -o "$output/bbox-oracle$extension"
  echo "Running $edition $target $profile bounding-box oracle"
  if [[ "$target" == wasm ]]; then node "$output/bbox-oracle.js"; else "$output/bbox-oracle"; fi
}

if [[ "$mode" == --native || "$mode" == --all ]]; then
  run_case native debug -O0
  run_case native release -O2
fi
if [[ "$mode" == --wasm || "$mode" == --all ]]; then
  if [[ -f "$root/.tools/emsdk/emsdk_env.sh" ]]; then
    set +u
    EMSDK_QUIET=1 source "$root/.tools/emsdk/emsdk_env.sh" >/dev/null
    set -u
  fi
  required="$(cat "$root/.emscripten-version")"
  version="$(emcc --version | head -n 1)"
  if [[ "$version" != *" $required "* ]]; then
    echo "Expected Emscripten $required; found: $version" >&2
    exit 1
  fi
  run_case wasm debug -O0
  run_case wasm release -O2
fi
