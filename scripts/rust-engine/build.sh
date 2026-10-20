#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../.." && pwd)"
source "$root/scripts/rust-env.sh"
edition="${1:-original}"
case "$edition" in original|hd) ;; *) echo 'Expected original or hd.' >&2; exit 1 ;; esac
python3 "$root/scripts/rust-engine/asm-const-fix.py" "$root/rust/uqm-engine-$edition"
if [[ "$edition" = hd ]]; then
  python3 "$root/scripts/rust-engine/lua-fix.py" "$root/rust/uqm-engine-hd" --audit
fi
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$root/.tools/rust-engine-$edition}"
export RUSTFLAGS="${RUSTFLAGS:-} -C target-feature=+atomics,+bulk-memory,+mutable-globals -C panic=abort"
cargo "+$(cat "$root/.rust-wasm-toolchain")" build --locked \
  -Zbuild-std=core,alloc,compiler_builtins \
  --manifest-path "$root/rust/uqm-engine-$edition/Cargo.toml" \
  --target wasm32-unknown-emscripten --release
printf '%s\n' "$CARGO_TARGET_DIR/wasm32-unknown-emscripten/release/libuqm_engine_$edition.a"
