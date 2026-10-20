#!/usr/bin/env bash
set -euo pipefail
cosmo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$cosmo_root/../scripts/rust-env.sh"
cargo +1.85.1 fmt --manifest-path "$cosmo_root/rust/Cargo.toml" --all -- --check
cargo +1.85.1 test --manifest-path "$cosmo_root/rust/Cargo.toml" --locked
cargo +1.85.1 clippy --manifest-path "$cosmo_root/rust/Cargo.toml" --all-targets --locked -- -D warnings
