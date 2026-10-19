#!/usr/bin/env bash
# Independent actual-source queue/geometry oracle against unchanged upstream C sources.
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
includes=("-I$root/tests/rust/include" "-I$source_root")
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$root/rust/target}"
if [[ "$CARGO_TARGET_DIR" != /* ]]; then
  export CARGO_TARGET_DIR="$root/$CARGO_TARGET_DIR"
fi

run_case() {
  local target="$1" profile="$2" optimization="$3"
  local output="$root/.tools/rust-queue-geometry-oracle/$edition-$target-$profile"
  local compiler="${CC:-cc}" extension="" library_target=""
  local cargo_args=(build --manifest-path "$root/rust/Cargo.toml" -p uqm-core)
  local flags=(-fsanitize=address,undefined -fno-sanitize=shift -fno-omit-frame-pointer)
  local link_flags=(-fno-sanitize=shift)
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
  if [[ "$profile" == release ]]; then
    cargo_args+=(--release)
  fi
  if [[ "$target" == wasm ]]; then
    RUSTFLAGS="${RUSTFLAGS:-} -Ctarget-feature=+atomics,+bulk-memory,+mutable-globals -Cpanic=abort" \
      cargo "${cargo_args[@]}" "${features[@]}"
  else
    cargo "${cargo_args[@]}" "${features[@]}"
  fi
  mkdir -p "$output"
  local source
  for source in uqm/displist libs/graphics/boxint libs/graphics/clipline; do
    "$compiler" -std=gnu11 -Wall -Wextra -Werror -fwrapv "$optimization" \
      "${flags[@]}" "${includes[@]}" "${environment[@]}" \
      -include "$root/tests/rust/queue-geometry-symbols.h" \
      -c "$source_root/$source.c" -o "$output/${source##*/}.o"
  done
  "$compiler" -std=gnu11 -Wall -Wextra -Werror -fwrapv "$optimization" \
    "${flags[@]}" "${includes[@]}" "${environment[@]}" \
    "$root/tests/rust/queue-geometry-oracle.c" "$output/displist.o" "$output/boxint.o" "$output/clipline.o" \
    "$CARGO_TARGET_DIR/$library_target$profile/libuqm_core.a" \
    "${link_flags[@]}" -o "$output/queue-geometry-oracle$extension"
  echo "Running $edition $target $profile queue/geometry oracle"
  if [[ "$target" == wasm ]]; then
    node "$output/queue-geometry-oracle.js"
  else
    "$output/queue-geometry-oracle"
  fi
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
