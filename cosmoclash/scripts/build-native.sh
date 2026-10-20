#!/usr/bin/env bash
set -euo pipefail
cosmo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$cosmo_root/../scripts/rust-env.sh"
cargo +1.85.1 build --manifest-path "$cosmo_root/rust/Cargo.toml" --bin cosmo-jsonl --locked
