#!/usr/bin/env python3
"""Run the actual pinned translator on argument/type-sensitive macro expansions."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import shutil

ROOT = Path(__file__).resolve().parents[2]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--c2rust', default=os.environ.get('C2RUST_BIN', str(ROOT / '.tools/c2rust-pinned-target/release/c2rust-transpile')))
parser.add_argument('--clang', default=os.environ.get('CLANG_PATH', str(ROOT / '.tools/c2rust-deps/llvm@18/18.1.8/bin/clang-18')))
parser.add_argument('--sysroot', default=str(ROOT / '.tools/emsdk/upstream/emscripten/cache/sysroot'))
parser.add_argument('--target', choices=['native', 'wasm', 'all'], default='all')
args = parser.parse_args()
for file in [args.c2rust, args.clang]:
    if not Path(file).is_file(): parser.error(f'Not found: {file}; see scripts/rust-engine/setup-translator.sh')
output = ROOT / '.tools/rust-macro-oracle'
output.mkdir(exist_ok=True)
source = ROOT / 'tests/rust/macro-fixture.c'
database = output / 'compile_commands.json'
database.write_text(json.dumps([{'directory': str(ROOT), 'file': str(source), 'arguments': [
    str(Path(args.clang).resolve()), '-target', 'wasm32-unknown-emscripten',
    '--sysroot=' + str(Path(args.sysroot).resolve()), '-std=c11', '-pthread', '-c', str(source)
]}], indent=2))
env = dict(os.environ, C2RUST_NO_HOST_HEADERS='1', C2RUST_TARGET_WASM='1')
llvm_lib = str(Path(args.clang).resolve().parent.parent / 'lib')
for key in ['DYLD_LIBRARY_PATH', 'LD_LIBRARY_PATH']:
    env[key] = llvm_lib + (os.pathsep + env[key] if key in env else '')
results = []
for policy in ['expanded', 'default-negative-control']:
    destination = output / policy
    destination.mkdir(exist_ok=True)
    generated = destination / 'generated'
    # Never silently reuse an earlier translation after the fixture changes.
    if generated.exists(): shutil.rmtree(generated)
    flags = ['--translate-const-macros', 'none', '--translate-fn-macros', 'none'] if policy == 'expanded' else []
    command = [str(Path(args.c2rust).resolve()), '--emit-no-std', '--fail-on-error',
               '--disable-refactoring', '--disable-rustfmt', '--edition', '2021', *flags,
               '--output-dir', str(generated), str(database)]
    subprocess.run(command, env=env, check=True, cwd=ROOT)
    files = list(generated.rglob('macro_fixture.rs'))
    if len(files) != 1: raise RuntimeError(f'Expected one generated source, found {files}')
    # Add only the no_std panic binding; every translated expression is untouched.
    body = files[0].read_text()
    body += '\n#[panic_handler]\nfn panic(_: &core::panic::PanicInfo) -> ! { unsafe { extern "C" { fn abort() -> !; } abort() } }\n'
    (destination / 'lib.rs').write_text(body)
    (destination / 'Cargo.toml').write_text('''[workspace]
[package]
name = "uqm-macro-oracle"
version = "0.1.0"
edition = "2021"
[lib]
name = "macro_oracle"
path = "lib.rs"
crate-type = ["staticlib"]
[profile.dev]
panic = "abort"
overflow-checks = false
[profile.release]
panic = "abort"
''')
    # This fixture contains only defined unsigned arithmetic; C2Rust's debug
    # integer checks still need the production wrapping setting for u32 +/*.
    test_env = dict(os.environ, MACRO_ORACLE_DIR=str(destination), MACRO_ORACLE_EXPECT_FAILURE='1' if policy != 'expanded' else '0')
    completed = subprocess.run(['bash', str(ROOT / 'tests/rust/test-macros-build.sh'), args.target],
                               cwd=ROOT, env=test_env, text=True)
    results.append({'policy': policy, 'exit': completed.returncode, 'flags': flags})
    if completed.returncode: raise SystemExit(completed.returncode)
(output / 'results.json').write_text(json.dumps(results, indent=2) + '\n')
print('Actual translator regression passed; unsafe default reconstruction was rejected.')
