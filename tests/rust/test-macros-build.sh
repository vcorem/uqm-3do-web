#!/usr/bin/env bash
# Internal runner used by test-macros.py; archives contain actual translator output.
set -euo pipefail
root="$(cd "$(dirname "$0")/../.." && pwd)"
source "$root/scripts/rust-env.sh"
directory="${MACRO_ORACLE_DIR:?test-macros.py supplies generated source}"
expected_failure="${MACRO_ORACLE_EXPECT_FAILURE:?}"
mode="${1:?}"
export CARGO_TARGET_DIR="$directory/target"
pin="$(cat "$root/.rust-wasm-toolchain")"
run_case() {
  local target="$1" profile="$2" opt="$3"
  local compiler="${CC:-cc}" suffix="" libtarget=""
  local flags=(-fsanitize=address,undefined -fno-omit-frame-pointer)
  local libs=(-lm)
  local build=("+$pin" build --manifest-path "$directory/Cargo.toml")
  local output="$directory/$target-$profile"
  if [[ "$target" == wasm ]]; then
    compiler=emcc; suffix=.js; libtarget=wasm32-unknown-emscripten/
    flags=(-pthread)
    libs=(-sSINGLE_FILE=1 -sENVIRONMENT=node -sEXIT_RUNTIME=1 -sASSERTIONS=2)
    build+=(--target wasm32-unknown-emscripten -Zbuild-std=core,compiler_builtins)
  fi
  if [[ "$profile" == release ]]; then build+=(--release); fi
  if [[ "$target" == wasm ]]; then
    RUSTFLAGS="-Ctarget-feature=+atomics,+bulk-memory,+mutable-globals -Cpanic=abort" cargo "${build[@]}"
  else cargo "${build[@]}"; fi
  mkdir -p "$output"
  "$compiler" -std=c11 -Wall -Wextra -Werror "$opt" "${flags[@]}" \
    -Dmacro_fixture=oracle_macro_fixture -c "$root/tests/rust/macro-fixture.c" -o "$output/reference.o"
  "$compiler" -std=c11 -Wall -Wextra -Werror "$opt" "${flags[@]}" "$root/tests/rust/macro-oracle.c" \
    "$output/reference.o" "$CARGO_TARGET_DIR/$libtarget$profile/libmacro_oracle.a" "${libs[@]}" -o "$output/oracle$suffix"
  echo "Macro policy $(basename "$directory"), $target $profile"
  local status=0
  if [[ "$target" == wasm ]]; then node "$output/oracle.js" || status=$?
  else "$output/oracle" || status=$?; fi
  if [[ "$status" != "$expected_failure" ]]; then
    echo "Expected differential status $expected_failure; got $status" >&2; exit 1
  fi
}
if [[ "$mode" == native || "$mode" == all ]]; then run_case native debug -O0;run_case native release -O2;fi
if [[ "$mode" == wasm || "$mode" == all ]]; then
  set +u
  EMSDK_QUIET=1 source "$root/.tools/emsdk/emsdk_env.sh" >/dev/null
  set -u
  run_case wasm debug -O0;run_case wasm release -O2
fi
