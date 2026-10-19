#!/usr/bin/env python3
"""Build a local SDK3.1.50 proxy runtime with the upstream context-lifetime fix.

The SDK and its shared cache are never modified. Both web engines link this
object before Emscripten's system libraries, replacing their proxying object.
"""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess

SDK_VERSION = "3.1.50"
SOURCE_SHA256 = "a772941ebd474044ba1eeb3172c9e4e0135c8d407e4a5ce7a77561780c7e44c3"
UPSTREAM_FIX = "https://github.com/emscripten-core/emscripten/pull/26582"
UPSTREAM_COMMIT = "f3fef05bf5d63df33ed21eb067b86d827c546bfa"


def patched_source(source):
    if hashlib.sha256(source).hexdigest() != SOURCE_SHA256:
        raise ValueError("Emscripten proxying.c does not match the pinned 3.1.50 source")
    text = source.decode("utf-8")
    old = "    pthread_mutex_unlock(&ctx->sync.mutex);\n    pthread_cond_signal(&ctx->sync.cond);"
    new = ("    // Keep the stack-owned context alive until signaling is complete.\n"
           "    // Backport of " + UPSTREAM_FIX + "\n"
           "    pthread_cond_signal(&ctx->sync.cond);\n"
           "    pthread_mutex_unlock(&ctx->sync.mutex);")
    if text.count(old) != 2:
        raise ValueError("Expected exactly the finish and cancel context paths")
    return text.replace(old, new).encode("utf-8")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    compiler = shutil.which("emcc")
    if not compiler:
        raise SystemExit("Emscripten 3.1.50 is required")
    compiler = Path(compiler).resolve()
    version = subprocess.check_output([str(compiler), "--version"], text=True).splitlines()[0]
    if " " + SDK_VERSION + " " not in version:
        raise SystemExit("Expected Emscripten " + SDK_VERSION + "; found: " + version)
    sdk = compiler.parent
    source = sdk / "system/lib/pthread/proxying.c"
    patched = patched_source(source.read_bytes())
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    local_source = output / "emscripten-proxying.c"
    local_source.write_bytes(patched)
    obj = output / "emscripten-proxying.o"
    subprocess.run([str(compiler), "-c", "-pthread", "-O0", "-g",
                    '-Dhidden=__attribute__((visibility("hidden")))',
                    "-I", str(sdk / "system/lib/pthread"),
                    "-I", str(sdk / "system/lib/libc/musl/src/internal"),
                    str(local_source), "-o", str(obj)], check=True)
    (output / "toolchain-fixes.json").write_text(json.dumps({
        "emscripten": SDK_VERSION,
        "fixes": [{"upstream": UPSTREAM_FIX, "commit": UPSTREAM_COMMIT,
                   "source": "system/lib/pthread/proxying.c",
                   "sourceSha256": SOURCE_SHA256,
                   "patchedSha256": hashlib.sha256(patched).hexdigest(),
                   "description": "Signal synchronous proxy completion before releasing its context mutex"}]
    }, indent=2) + "\n")
    print("Prepared local Emscripten proxy lifetime fix: " + str(obj))


if __name__ == "__main__":
    main()
