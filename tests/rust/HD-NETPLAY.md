# HD network transport and packet oracle

Run after configuring and building the HD C source tree:

```sh
bash tests/rust/test-hd-netplay.sh --all
# For a separate source cache:
bash tests/rust/test-hd-netplay.sh --all --source .tools/hd-netplay-source
```

The script verifies every HD C source/header hash against the checked-in input
lock. It imports the production Rust `netrcv.rs`, `netsend.rs` and
`packethandlers.rs` files without editing them. Their real C counterparts are
compiled independently with renamed public symbols, allowing the same Wasm
driver to compare both implementations in one process.

The fixture checks C/Rust structure sizes and field offsets, malformed and
truncated fleet/team-name packets, invalid sides, ship types and positions,
reset and confirmation flags, every byte split across packet boundaries,
coalesced packets, callback-triggered disconnect, exhausted receive buffers,
invalid incoming JavaScript byte queues, send backpressure, send exceptions
and closed connections. It compares return values, errors, callback traces,
connection flags, protocol statistics and buffered bytes. Independent
assertions also check successful complete-packet handling and rejection of
invalid ship selections.

`--all` exercises debug and release Rust/C builds, each with a direct main
thread and a real pthread. Both use shared Wasm memory; pthread runs exercise
the actual Emscripten main-thread JavaScript byte bridge and the pinned proxy
compatibility adapter. Omitting `--all` runs the two release modes.

Only external game callbacks and the browser room are deterministic fixtures.
No Lua scripts execute in this isolated test; its Lua pending-error callback
returns zero, while the production Rust propagation macros remain intact.
Full Lua protection is covered by `test-lua.sh`. The network fixture is not a
substitute for two-browser full-fleet, disconnect, persistence and battle
checksum tests against the independently built C engine.

Machine-readable results and exact hashes of the three tested Rust/C source
modules are written to `.tools/rust-hd-netplay-tests/results.json`. Build and
runtime logs remain alongside that report.

All four modes passed after the HD netplay migration: 12,210 comparison cases
and 13 ABI checks per mode, for 48,840 matching cases overall. These are
focused boundary comparisons; the full-game browser results are recorded in
[the Rust validation record](../../docs/rust-validation.md).
