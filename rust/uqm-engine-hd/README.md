# UQM hd Rust engine

This standalone Wasm crate contains generated unsafe Rust translations and
the shared, independently tested `uqm-core` implementation.

Build from the repository root with `scripts/rust-engine/build.sh hd`.
See `scripts/rust-engine/README.md` for pins, regeneration, external library
boundaries and validation. `SOURCE-MAP.json` identifies original source files
and hashes; upstream copyright and license notices are preserved in source
headers, `COPYING`, and `notices/`.

The combined engine is distributed under GPL version 2: the bundled UIO
library permits GPL-2.0-only. Individual UQM, Rust core and PNG components
retain their GPL-2.0-or-later notices; the aggregate crate metadata does not
relicense those components. See `notices/src/libs/uio/COPYING`.
