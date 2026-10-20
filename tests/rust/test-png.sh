#!/usr/bin/env bash
# Compile unchanged Original + HD C beside the production Rust PNG module.
set -euo pipefail
root="$(cd "$(dirname "$0")/../.." && pwd)"
source "$root/scripts/rust-env.sh"
mode="${1:---wasm}"
if [[ "$#" -gt 1 || ( "$mode" != --native && "$mode" != --wasm && "$mode" != --all ) ]]; then
  echo "Usage: $0 [--native|--wasm|--all]" >&2; exit 2
fi
source_hd="${HD_SOURCE_DIR:-$root/.tools/hd-source}"
if [[ ! -f "$source_hd/src/libs/graphics/sdl/png2sdl.c" ]]; then
  echo 'The pinned HD source checkout is required; run scripts/build-hd-web.sh first.' >&2; exit 1
fi
artifacts="$root/.tools/rust-png-oracle"
export CARGO_TARGET_DIR="$artifacts/cargo"
manifest="$root/tests/rust/png-abi/Cargo.toml"

run_case() {
  local target="$1" profile="$2" optimization="$3"
  local compiler="${CC:-cc}" extension="" libtarget=""
  local output="$artifacts/$target-$profile"
  local args=(build --manifest-path "$manifest" --locked)
  local flags=(-fsanitize=address,undefined -fno-omit-frame-pointer)
  local includes=("-I$root/rust/uqm-png")
  local libs
  if [[ "$target" == native ]]; then
    local sdl_config="${SDL2_CONFIG:-$root/.tools/rust-png-sdl/sdl-native/bin/sdl2-config}"
    local png_config="${LIBPNG_CONFIG:-/opt/homebrew/bin/libpng16-config}"
    local extra
    if [[ ! -x "$sdl_config" || ! -x "$png_config" ]]; then
      echo 'Native tests require real SDL2 and libpng; see tests/rust/PNG.md.' >&2; exit 1
    fi
    read -r -a extra <<< "$("$sdl_config" --cflags) $("$png_config" --cflags)"
    includes+=("${extra[@]}")
    read -r -a extra <<< "$("$sdl_config" --static-libs) $("$png_config" --ldflags)"
    libs=("${extra[@]}")
  else
    compiler=emcc; extension=.js; libtarget=wasm32-unknown-emscripten/
    args=("+$(cat "$root/.rust-wasm-toolchain")" build -Zbuild-std=core,compiler_builtins \
      --target wasm32-unknown-emscripten --manifest-path "$manifest" --locked)
    flags=(-pthread -sUSE_SDL=2 -sUSE_LIBPNG=1)
    libs=(-sSINGLE_FILE=1 -sENVIRONMENT=node -sEXIT_RUNTIME=1 -sASSERTIONS=2 -sSTACK_SIZE=4194304)
  fi
  if [[ "$profile" == release ]]; then args+=(--release); fi
  if [[ "$target" == wasm ]]; then
    RUSTFLAGS="-Ctarget-feature=+atomics,+bulk-memory,+mutable-globals -Cpanic=abort" cargo "${args[@]}"
  else cargo "${args[@]}"; fi
  mkdir -p "$output"
  "$compiler" -std=c11 -Wall -Wextra -Werror "$optimization" "${flags[@]}" "${includes[@]}" \
    -DTFB_png_to_sdl=oracle_original_decode -c "$root/sc2/src/libs/graphics/sdl/png2sdl.c" -o "$output/original.o"
  "$compiler" -std=c11 -Wall -Wextra -Werror "$optimization" "${flags[@]}" "${includes[@]}" \
    -DTFB_png_to_sdl=oracle_hd_decode -DTFB_sdl_to_png=oracle_hd_write -DSDL_PNGFormatAlpha=oracle_hd_format \
    -c "$source_hd/src/libs/graphics/sdl/png2sdl.c" -o "$output/hd.o"
  "$compiler" -std=c11 -Wall -Wextra -Werror "$optimization" "${flags[@]}" "${includes[@]}" \
    "$root/tests/rust/png-oracle.c" "$root/rust/uqm-png/png-guard.c" "$output/original.o" "$output/hd.o" \
    "$CARGO_TARGET_DIR/$libtarget$profile/libpng_adapter_oracle.a" "${libs[@]}" -o "$output/oracle$extension"
  echo "Running PNG $target $profile with unchanged Original and HD C"
  if [[ "$target" == wasm ]]; then node "$output/oracle.js" 2>"$output/errors.log"
  else "$output/oracle" 2>"$output/errors.log"; fi
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
    echo "Expected Emscripten $required; found $version" >&2; exit 1
  fi
  run_case wasm debug -O0
  run_case wasm release -O2
fi
