#!/usr/bin/env bash
# Pinned Lua C runtime + production guard + real Rust callback/destructor oracle.
set -euo pipefail
root="$(cd "$(dirname "$0")/../.." && pwd)"
source "$root/scripts/rust-env.sh"
mode="${1:---wasm}"
if [[ "$#" -gt 1 || ( "$mode" != --native && "$mode" != --wasm && "$mode" != --all ) ]]; then
  echo "Usage: $0 [--native|--wasm|--all]" >&2; exit 2
fi
lua_source="${HD_SOURCE_DIR:-$root/.tools/hd-source}/src/libs/lua"
if [[ ! -f "$lua_source/Makeinfo" ]]; then echo 'Pinned HD source checkout required.' >&2; exit 1; fi
source "$lua_source/Makeinfo"
sources=()
for file in $uqm_CFILES; do sources+=("$lua_source/$file"); done
artifacts="$root/.tools/rust-lua-oracle"
export CARGO_TARGET_DIR="$artifacts/cargo"
manifest="$root/tests/rust/lua-abi/Cargo.toml"
run_case() {
  local target="$1" profile="$2" optimization="$3"
  local compiler="${CC:-cc}" extension="" libtarget=""
  local output="$artifacts/$target-$profile"
  local args=(build --manifest-path "$manifest" --locked)
  local flags=(-fsanitize=address,undefined -fno-omit-frame-pointer)
  local libs=(-lm)
  if [[ "$target" == wasm ]]; then
    compiler=emcc; extension=.js; libtarget=wasm32-unknown-emscripten/
    args=("+$(cat "$root/.rust-wasm-toolchain")" build -Zbuild-std=core,compiler_builtins \
      --target wasm32-unknown-emscripten --manifest-path "$manifest" --locked)
    flags=(-pthread)
    libs=(-sSINGLE_FILE=1 -sENVIRONMENT=node -sEXIT_RUNTIME=1 -sASSERTIONS=2 -sSTACK_SIZE=4194304)
  fi
  if [[ "$profile" == release ]]; then args+=(--release); fi
  if [[ "$target" == wasm ]]; then
    RUSTFLAGS="-Ctarget-feature=+atomics,+bulk-memory,+mutable-globals -Cpanic=abort" cargo "${args[@]}"
  else cargo "${args[@]}"; fi
  mkdir -p "$output"
  "$compiler" -std=c11 -Wall -Wextra -Wno-deprecated-declarations -DLUA_USE_APICHECK "$optimization" "${flags[@]}" -I"$root/rust/uqm-lua" -I"$lua_source" \
    "$root/tests/rust/lua-boundary-oracle.c" "$root/rust/uqm-lua/lua-guard.c" "${sources[@]}" \
    "$CARGO_TARGET_DIR/$libtarget$profile/liblua_boundary_oracle.a" "${libs[@]}" -o "$output/oracle$extension"
  echo "Running Lua boundary $target $profile with pinned unchanged Lua 5.2"
  if [[ "$target" == wasm ]]; then node "$output/oracle.js"; else "$output/oracle"; fi
}
if [[ "$mode" == --native || "$mode" == --all ]]; then run_case native debug -O0; run_case native release -O2; fi
if [[ "$mode" == --wasm || "$mode" == --all ]]; then
  if [[ -f "$root/.tools/emsdk/emsdk_env.sh" ]]; then
    set +u
    EMSDK_QUIET=1 source "$root/.tools/emsdk/emsdk_env.sh" >/dev/null
    set -u
  fi
  required="$(cat "$root/.emscripten-version")"
  version="$(emcc --version | head -n 1)"
  if [[ "$version" != *" $required "* ]]; then echo "Expected Emscripten $required; found $version" >&2; exit 1; fi
  run_case wasm debug -O0;run_case wasm release -O2
fi
