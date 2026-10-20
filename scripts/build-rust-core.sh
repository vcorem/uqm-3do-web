#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
source "$root/scripts/rust-env.sh"
cd "$root"
edition="${1:-original}"
args=("+$(cat "$root/.rust-wasm-toolchain")" build -Zbuild-std=core,compiler_builtins
  --manifest-path "$root/rust/Cargo.toml" -p uqm-core
  --target wasm32-unknown-emscripten --release)
case "$edition" in
  original) ;;
  hd) args+=(--features hd) ;;
  *) echo 'Expected edition original or hd.' >&2; exit 1 ;;
esac
# Do not mix feature variants in one archive consumed by the two link jobs.
export CARGO_TARGET_DIR="$root/.tools/rust-$edition"
# The browser engine shares memory between pthreads. Even prebuilt Rust core
# lacks the necessary shared-memory features, so rebuild core/compiler_builtins
# with the exact same target flags. The game still uses no_std, not Rust libc.
export RUSTFLAGS="${RUSTFLAGS:-} -C target-feature=+atomics,+bulk-memory,+mutable-globals -C panic=abort"
cargo "${args[@]}"
