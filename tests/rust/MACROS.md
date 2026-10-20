# Actual-translator macro regression

`test-macros.py` invokes the same patched, pinned C2Rust translator used by the
whole-engine regeneration. It does not compare against a handwritten Rust
version of the fixture. Install the translator using
`scripts/rust-engine/setup-translator.sh`, with its required LLVM 18.1.8 tools,
then run:

```sh
python3 tests/rust/test-macros.py --clang "$CLANG_PATH"
```

The default translator is the setup script's
`.tools/c2rust-pinned-target/release/c2rust-transpile`. `--c2rust` or `C2RUST_BIN`
can select the same pinned build at another path. `--clang`, `CLANG_PATH` and
`--sysroot` allow an explicitly configured LLVM/Emscripten installation.
`--target native` or `--target wasm` limits the run; the default runs both.

The small C fixture uses the actual `MakeLinks` object-macro alias pattern from
UQM's graphics headers. Its constant invocations have different arguments,
while a separate alias exercises variable arguments. Keeping constant and
variable aliases distinct is significant: C2Rust's conservative policy skips
reconstruction when it sees a nonconstant invocation of the same alias, which
would hide this regression. The fixture also covers nested expansions,
statement/member updates, type-sensitive sizes and casts, token concatenation,
different struct member types, and side-effect arguments without undefined C
evaluation order.

The test generates and compiles two independent translator outputs:

1. Production policy: `--translate-const-macros none --translate-fn-macros none`.
   Every Clang-expanded expression must agree with the actual compiled C.
2. Negative control: the pinned translator's default conservative policy.
   This must demonstrably disagree, reproducing the bug that left ambient star
   display lists empty. One observed collapse makes both `MakeLinks` calls
   return `329`, including the call whose correct result is `0xffffffff`.

Both outputs run in native and shared-memory Wasm, debug and optimized. The C
oracle has native ASan/UBSan; the Rust archive is not sanitizer instrumented.
All arithmetic in the fixture is defined unsigned arithmetic, and debug Rust
uses the production wrapping configuration. Only a no-std panic binding is
appended to the generated Rust; no generated algorithm is repaired. Fresh
translation output is created on every run, so C2Rust cannot silently skip a
stale generated file.

Each configuration compares **162,704 results**. The production policy passes
with zero differences; the negative control produces **10,169 differences**.
The ordinary browser CI does not need to install C2Rust: it runs the frozen
real-image regression in `tests/browser/scene-pixels.test.mjs` and the stronger
ambient-star assertion against the freshly built HD game in
`tests/browser/hd-campaign.mjs`. Run this translator oracle whenever changing
translation settings, the translator patch/pin, or regenerating engine bodies.
