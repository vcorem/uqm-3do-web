# Lua error boundary for the HD Rust engine

The HD game and its Lua-facing game functions are Rust. The pinned Lua 5.2
runtime remains an external C dependency. Its errors use `longjmp`, which
must never skip a live Rust frame.

`lua-guard.c` contains the external-runtime binding. Each potentially throwing
operation runs under its own Lua `luaD_pcall` protection, which restores Lua's
call frame, stack, hooks and upvalues on failure. It preserves the inherited
Lua error handler, so `xpcall` transforms an error exactly once, including
reentrant Rust handlers. A thread-local stack records
failure in the current Rust callback. All later operations in that callback
become no-ops until Rust has returned. Only then does the C trampoline throw
the saved Lua status. This preserves out-of-memory status as well as ordinary
script errors. Reader callbacks have the same boundary.

Rust callbacks receive a final hidden userdata upvalue containing a typed
function pointer. Their original upvalue indices remain unchanged. The five
native library openers (`base`, `table`, `string`, `bit32`, `math`) remain native
C callbacks; wrapping a throwing native callback in a Rust frame record would
leave a dangling thread-local frame. Non-null continuation callbacks are
rejected because the engine uses none and their suspension would need a
separate lifetime design. `lua_close` uses Lua's own protected finalizer path;
`luaL_newstate` already contains allocation errors internally and a null result
fails immediately. Errors outside a Rust callback also abort, matching Lua's
unprotected-error contract without jumping over Rust.

The binding uses private headers from the exact Lua version pinned in
`scripts/rust-engine/provenance.json`. Build it with the same target/thread
flags as the engine and that source tree's `src/libs/lua` include directory.
Do not substitute an unrelated system Lua library.

`guard.rs` contains the small shared propagation macro. The syntax-aware
`scripts/rust-engine/lua-propagate` tool redirects all 38 imported Lua APIs to
the typed C guards, computes transitive callers and conservatively includes
indirect function-pointer calls. Each affected call receives an immediate
failure check. Returning through every Rust caller before the C throw also
runs any Rust destructors. Existing signatures and successful-path expression
order remain intact. Failure values are explicit, type-checked unit, scalar,
boolean, null pointer or C aggregate constructors. The tool rejects unsupported
return types; it never creates references or Rust enums by zeroing bytes.

The checked-in HD engine has 13,209 checked calls across 264 files. Its single
`lua-propagation-report.json` records affected functions and failure returns.
`--audit` independently recomputes reachability from the actual parsed source,
including calls inside propagation macros, and rejects unguarded imports or
missing checks. Editing the report count cannot hide a new unchecked caller.

```sh
python3 scripts/rust-engine/lua-fix.py rust/uqm-engine-hd --audit
```

Packaging invokes `--apply` on freshly generated HD sources. The helper uses a
locked host-tool dependency graph, stable Rust 1.85.1 and the isolated toolchain
when installed. Source tests cover nested propagation, indirect callbacks,
explicit aggregate returns, Unicode positions, rejected reference/enum types,
duplicate application and a stale-report bypass attempt. They run with
`scripts/rust-engine/test_lua_propagate.py` and the built helper executable.

The independent runtime oracle uses this exact macro, the production C guard
and the pinned Lua runtime. It verifies invalid arguments, explicit errors,
throwing metamethods, nested protected calls, reader errors, original upvalue
indices, finalizers and allocation failures with Rust return/Drop counters.
Native and shared-memory Wasm debug/optimized profiles each pass 9,743 checks
across 643 cases, confirming 2,585 normal Rust returns per run. See
`tests/rust/test-lua.sh` for the reproducible check. This error boundary
provides normal return through Rust, not rollback of gameplay changes already
completed before an error.
