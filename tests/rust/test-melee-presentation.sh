#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../.." && pwd)"
source "$root/scripts/rust-env.sh"
output="$root/.tools/melee-presentation-tests"
mkdir -p "$output"
rustc +1.85.1 --edition=2021 --test "$root/tests/rust/melee-presentation.rs" -o "$output/registry"
"$output/registry" --test-threads=1
