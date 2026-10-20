#!/usr/bin/env bash
# Source this file. Prefer the isolated toolchain when it has been installed.
rust_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ -x "$rust_root/.tools/cargo/bin/cargo" ]]; then
  export CARGO_HOME="${CARGO_HOME:-$rust_root/.tools/cargo}"
  export RUSTUP_HOME="${RUSTUP_HOME:-$rust_root/.tools/rustup}"
  export PATH="$rust_root/.tools/cargo/bin:$PATH"
fi
if ! command -v cargo >/dev/null; then
  echo 'Install the Rust toolchain specified in rust-toolchain.toml before building.' >&2
  return 1
fi
