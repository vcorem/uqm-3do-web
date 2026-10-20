# Lua boundary oracle

The HD engine contains Rust game callbacks but retains the pinned external Lua
5.2 runtime. Lua errors use non-local C control transfer; they must never skip a
live Rust frame. This oracle executes the production
`rust/uqm-lua/lua-guard.c` and `rust/uqm-lua/guard.rs`, not substitutes for either
boundary. It links the unchanged Lua sources listed by the pinned HD
`src/libs/lua/Makeinfo` and uses the real public and internal headers.

Run after the pinned HD checkout has been prepared:

```sh
npm run test:rust:lua                  # shared Wasm debug and optimized
bash tests/rust/test-lua.sh --native   # native debug and optimized
bash tests/rust/test-lua.sh --all      # all four configurations
```

The Wasm build uses the repository's pinned Emscripten and Rust nightly,
rebuilding `core` and `compiler_builtins` with the same atomics, bulk memory and
mutable globals as the production shared-memory build. The native harness,
guard and external Lua runtime use ASan/UBSan; the Rust archive is not sanitizer
instrumented. Lua's API assertions are enabled in all configurations. Results
and build products are isolated under `.tools/rust-lua-oracle/`.

`lua-boundary-oracle.c` implements reference callbacks that use Lua's original
C API directly. `lua-boundary.rs` supplies equivalent real Rust callbacks using
all 38 imported guard functions and the production checked-return macro. It
also includes a Rust helper beneath a callback and nested Rust callbacks. A
Rust `Drop` marker records every function entry and normal return: all markers
must run before the outer Lua protected call catches an error. The test checks
that the thread's callback frame and failure flag have been restored, then
reuses the same Lua state for a successful call after each error.

The corpus checks:

- Signed integer arguments and coercions, strings including embedded NUL,
  booleans, nil, invalid argument types, options and custom formatted errors.
- All imported stack, conversion, table, raw-index and global operations;
  throwing read/write metamethods; native and Rust nested calls; protected
  calls that deliberately catch another callback's failure. `xpcall` handlers
  must run exactly once, may reenter Rust, and may fail once before recovering;
  non-string error objects survive unchanged. This includes a regression that
  failed before inherited error-handler preservation was fixed.
- Reader callbacks that return valid or malformed chunks or raise an argument
  error, plus ordinary malformed-source errors returned by Lua's loader.
- Original user upvalues beside the hidden Rust callback pointer, registration
  through `luaL_setfuncs` with and without user upvalues, and all five native
  library openers that must remain native callbacks.
- Persistent allocation failures in table creation, string creation, numeric
  string conversion, argument error formatting, explicit error formatting and
  Rust callback registration. Four 80-step allocation-budget sweeps exercise
  progressively later failures in table, closure and parser operations.
- Rust finalizer callbacks that fail during `lua_close`. The pinned Lua close
  path suppresses finalizer errors internally, after the Rust callback returns.

The fault-injection allocator honors Lua's contract that shrinking an existing
allocation cannot fail. Callback registration uses an extra hidden userdata
allocation, so allocation counts are not asserted equal to the C reference.
The registration-only OOM case asserts a recoverable memory error and normal
Rust return. Status, stack types/values and observable callback completion are
compared on corresponding reference operations. Opaque function identities are
not compared; strings and numbers are compared directly.

The recorded four-configuration run passed **9,743 checks, 643 cases and 2,585
Rust frame returns per configuration**. This validates the error boundary and
shared propagation macro. It does not prove that every engine call site has
been instrumented: the packaging-time Lua call-graph audit and whole-engine HD
build/browser tests provide that separate check. The guard uses private Lua
5.2 internals and must be re-audited when the pinned runtime changes. Errors
outside a Rust Lua callback and a failed initial state allocation deliberately
abort instead of attempting a non-local jump across Rust; game callbacks do
not support yielding continuations.

An exploratory infinitely failing `xpcall` handler exhausted Node's host call
stack in the direct unchanged C reference before Rust was invoked. The shared
corpus therefore uses a handler that fails once and recovers, testing nested
handler-error semantics without depending on the host's recursion limit. This
is an external Lua/host limitation, not evidence of a Rust-only regression.
