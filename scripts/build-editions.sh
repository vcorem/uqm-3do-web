#!/usr/bin/env bash
# Build the four published engines and package them behind the edition picker:
# Original C (/original/), Original Rust (/original-rust/), HD C (/hd/), HD Rust (/hd-rust/).
# UQM_EDITION_ENGINES=c builds only the two C engines (the previous two-version site).
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
engines="${UQM_EDITION_ENGINES:-c,rust}"
package_args=(--original "$root/.tools/original-dist" --hd "$root/.tools/hd-dist")
UQM_CORE=c BUILD_OUTPUT="$root/.tools/original-dist" "$root/scripts/build-web.sh"
UQM_CORE=c HD_BUILD_OUTPUT="$root/.tools/hd-dist" BUILD_OUTPUT="$root/.tools/hd-dist" "$root/scripts/build-hd-web.sh"
if [[ ",$engines," == *,rust,* ]]; then
  UQM_CORE=rust BUILD_OUTPUT="$root/.tools/original-rust-dist" "$root/scripts/build-web.sh"
  # The C build above fetched and verified the pinned HD media.
  UQM_CORE=rust HD_OFFLINE=1 HD_BUILD_OUTPUT="$root/.tools/hd-rust-dist" BUILD_OUTPUT="$root/.tools/hd-rust-dist" "$root/scripts/build-hd-web.sh"
  package_args+=(--original-rust "$root/.tools/original-rust-dist" --hd-rust "$root/.tools/hd-rust-dist")
fi
python3 "$root/scripts/package-editions.py" "${package_args[@]}" --output "${BUILD_OUTPUT:-$root/dist}"
