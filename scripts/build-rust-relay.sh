#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
source "$root/scripts/rust-env.sh"
cd "$root"
target_dir="${CARGO_TARGET_DIR:-$root/rust/target}"
mkdir -p "$target_dir"
target_dir="$(cd "$target_dir" && pwd)"
cargo build --manifest-path rust/Cargo.toml -p uqm-relay --release --locked --target-dir "$target_dir"
printf '%s\n' "$target_dir/release/uqm-relay"
