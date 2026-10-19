#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../.." && pwd)"
source "$root/scripts/rust-env.sh"
python3 "$root/scripts/rust-engine/verify-provenance.py"
: "${LLVM_CONFIG_PATH:?Set LLVM_CONFIG_PATH to the isolated LLVM18 llvm-config executable.}"
: "${CLANG_PATH:?Set CLANG_PATH to the matching LLVM18 clang executable.}"
: "${LIBCLANG_PATH:?Set LIBCLANG_PATH to the matching LLVM18 library directory.}"
command -v cmake >/dev/null || { echo 'CMake is required; install it in an isolated environment.' >&2; exit 1; }
[[ "$("$LLVM_CONFIG_PATH" --version)" == 18.1.8 ]] || { echo 'LLVM18.1.8 is required for this translation snapshot.' >&2; exit 1; }
destination="${C2RUST_SOURCE_DIR:-$root/.tools/c2rust-pinned}"
revision=105afacc8d84e64df860bed470453edf6e0865fd
if [[ ! -d "$destination/.git" ]]; then
  [[ ! -e "$destination" ]] || { echo 'Translator destination exists without Git metadata.' >&2; exit 1; }
  git init "$destination"
  git -C "$destination" remote add origin https://github.com/immunant/c2rust.git
  git -C "$destination" fetch --depth 1 origin "$revision"
  git -C "$destination" checkout --detach FETCH_HEAD
fi
[[ "$(git -C "$destination" rev-parse HEAD)" == "$revision" ]] || { echo 'Unexpected translator revision.' >&2; exit 1; }
patch="$root/scripts/rust-engine/c2rust-wasm.patch"
if ! git -C "$destination" apply --reverse --check "$patch" 2>/dev/null; then
  git -C "$destination" apply --check "$patch"
  git -C "$destination" apply "$patch"
fi
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$root/.tools/c2rust-pinned-target}"
cargo +1.85.1 build --manifest-path "$destination/Cargo.toml" -p c2rust --release --locked
printf '%s\n' "$CARGO_TARGET_DIR/release/c2rust-transpile"
