#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../.." && pwd)"
source "$root/scripts/rust-env.sh"
cd "$root"
if [[ -f "$root/.tools/emsdk/emsdk_env.sh" ]]; then
  set +u
  EMSDK_QUIET=1 source "$root/.tools/emsdk/emsdk_env.sh" >/dev/null
  set -u
fi
required="$(cat "$root/.emscripten-version")"
[[ "$(emcc --version | head -n 1)" == *" $required "* ]] || { echo 'Wrong Emscripten version.' >&2; exit 1; }
# Both editions must use the exact production allocator exercised below.
cmp rust/uqm-engine-original/uqm_allocator.rs rust/uqm-engine-hd/uqm_allocator.rs
output="$root/.tools/rust-allocator-oracle"
mkdir -p "$output"
cat > "$output/Cargo.toml" <<'TOML'
[workspace]
[package]
name = "uqm-allocator-oracle"
version = "0.1.0"
edition = "2021"
[lib]
path = "lib.rs"
crate-type = ["staticlib"]
[profile.dev]
panic = "abort"
[profile.release]
panic = "abort"
TOML
{
  echo '#![no_std]'
  cat rust/uqm-engine-original/uqm_allocator.rs tests/rust/allocator-oracle.rs
} > "$output/lib.rs"
export CARGO_TARGET_DIR="$output/target"
for profile in debug release; do
  args=(build -Zbuild-std=core,compiler_builtins --manifest-path "$output/Cargo.toml" --target wasm32-unknown-emscripten)
  optimization=-O0
  if [[ "$profile" == release ]]; then args+=(--release); optimization=-O2; fi
  RUSTFLAGS='-Ctarget-feature=+atomics,+bulk-memory,+mutable-globals -Cpanic=abort' \
    cargo "+$(cat .rust-wasm-toolchain)" "${args[@]}"
  emcc -pthread "$optimization" tests/rust/allocator-oracle.c \
    "$output/target/wasm32-unknown-emscripten/$profile/libuqm_allocator_oracle.a" \
    -sENVIRONMENT=node -sSINGLE_FILE=1 -sEXIT_RUNTIME=1 -sASSERTIONS=2 \
    -sSTACK_SIZE=1048576 -sINITIAL_MEMORY=33554432 -o "$output/$profile.js"
  node "$output/$profile.js"
done
