#!/usr/bin/env bash
# Link the real C implementations and the replacement Rust static library into
# one differential executable. No copied/reimplemented C algorithm is an oracle.
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
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
cargo_features=(--no-default-features)
environment_flags=(-DUQM_TEST_ORIGINAL=1)
if [[ "$edition" == hd ]]; then
  source_root="${HD_SOURCE_DIR:-$root/.tools/hd-source}/src"
  if [[ ! -f "$source_root/libs/math/random.c" ]]; then
    echo 'The pinned HD source checkout is required for --hd. See scripts/build-hd-web.sh.' >&2
    exit 1
  fi
  cargo_features+=(--features hd)
  environment_flags=(-DUQM_TEST_HD=1 -include "$root/tests/rust/hd-environment.h")
fi

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$root/rust/target}"
if [[ "$CARGO_TARGET_DIR" != /* ]]; then
  export CARGO_TARGET_DIR="$root/$CARGO_TARGET_DIR"
fi
artifacts="$root/.tools/rust-core-oracle"
mkdir -p "$artifacts"
sources=(
  "$source_root/libs/math/random.c"
  "$source_root/libs/math/random2.c"
  "$source_root/libs/math/sqrt.c"
  "$source_root/uqm/supermelee/netplay/crc.c"
)
includes=("-I$root/tests/rust/include" "-I$source_root")

compile_oracle() {
  local compiler="$1" output="$2"
  shift 2
  local source
  mkdir -p "$output"
  for source in "${sources[@]}"; do
    "$compiler" -std=gnu11 -Wall -Wextra -Werror "${includes[@]}" \
      "${environment_flags[@]}" "$@" \
      -include "$root/tests/rust/oracle-symbols.h" -c "$source" \
      -o "$output/$(basename "${source%.c}").o"
  done
}

run_native() {
  local profile="$1" optimization="$2"
  local output="$artifacts/native-$edition-$profile"
  local compiler="${CC:-cc}"
  local cargo_args=(--profile dev)
  local checks=(-fsanitize=address,undefined -fno-omit-frame-pointer)
  if [[ "$profile" == release ]]; then
    cargo_args=(--release)
  fi
  cargo build --manifest-path "$root/rust/Cargo.toml" -p uqm-core \
    "${cargo_features[@]}" "${cargo_args[@]}"
  compile_oracle "$compiler" "$output" "$optimization" "${checks[@]}"
  "$compiler" -std=gnu11 -Wall -Wextra -Werror "${includes[@]}" "${environment_flags[@]}" \
    "$optimization" "${checks[@]}" "$root/tests/rust/core-oracle.c" \
    "$root/tests/rust/unused-engine-imports.c" \
    "$output/random.o" "$output/random2.o" "$output/sqrt.o" "$output/crc.o" \
    "$CARGO_TARGET_DIR/$profile/libuqm_core.a" -o "$output/core-oracle"
  echo "Running native $edition $profile C/Rust oracle (C harness: ASan + UBSan)"
  "$output/core-oracle"
}

prepare_wasm() {
  if [[ -f "$root/.tools/emsdk/emsdk_env.sh" ]]; then
    set +u
    EMSDK_QUIET=1 source "$root/.tools/emsdk/emsdk_env.sh" >/dev/null
    set -u
  fi
  local required version
  required="$(cat "$root/.emscripten-version")"
  if ! command -v emcc >/dev/null; then
    echo "Emscripten $required is required for --wasm. See README.md." >&2
    exit 1
  fi
  version="$(emcc --version | head -n 1)"
  if [[ "$version" != *" $required "* ]]; then
    echo "Expected Emscripten $required; found: $version" >&2
    exit 1
  fi
  if ! command -v node >/dev/null; then
    echo 'Node.js is required to execute the WebAssembly oracle.' >&2
    exit 1
  fi
}

run_wasm() {
  local profile="$1" optimization="$2"
  local output="$artifacts/wasm-$edition-$profile"
  local cargo_args=(--profile dev)
  local toolchain
  toolchain="$(cat "$root/.rust-wasm-toolchain")"
  if [[ "$profile" == release ]]; then
    cargo_args=(--release)
  fi
  RUSTFLAGS="${RUSTFLAGS:-} -Ctarget-feature=+atomics,+bulk-memory,+mutable-globals -Cpanic=abort" \
    cargo "+$toolchain" build -Zbuild-std=core,compiler_builtins \
      --manifest-path "$root/rust/Cargo.toml" -p uqm-core \
      --target wasm32-unknown-emscripten "${cargo_features[@]}" "${cargo_args[@]}"
  # Rebuild core with the production shared-memory target features in both
  # profiles: the stable precompiled core is not compatible with -pthread.
  compile_oracle emcc "$output" "$optimization" -pthread
  emcc -std=gnu11 -Wall -Wextra -Werror "${includes[@]}" "${environment_flags[@]}" \
    "$optimization" -pthread \
    "$root/tests/rust/core-oracle.c" "$output/random.o" "$output/random2.o" \
    "$root/tests/rust/unused-engine-imports.c" \
    "$output/sqrt.o" "$output/crc.o" \
    "$CARGO_TARGET_DIR/wasm32-unknown-emscripten/$profile/libuqm_core.a" \
    -sENVIRONMENT=node -sSINGLE_FILE=1 -sEXIT_RUNTIME=1 -sASSERTIONS=2 \
    -sINITIAL_MEMORY=33554432 -sSTACK_SIZE=1048576 -o "$output/core-oracle.js"
  echo "Running WebAssembly $edition $profile C/Rust oracle (Emscripten $(cat "$root/.emscripten-version"))"
  node "$output/core-oracle.js"
}

if [[ "$mode" == --native || "$mode" == --all ]]; then
  run_native debug -O0
  run_native release -O2
fi
if [[ "$mode" == --wasm || "$mode" == --all ]]; then
  prepare_wasm
  run_wasm debug -O0
  run_wasm release -O2
fi
