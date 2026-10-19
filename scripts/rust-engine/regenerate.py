#!/usr/bin/env python3
"""Regenerate into an empty staging directory; never overwrite reviewed crates."""
import argparse
import importlib.util
import os
from pathlib import Path
import subprocess

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--edition', choices=['original', 'hd'], required=True)
parser.add_argument('--source', required=True)
parser.add_argument('--output', required=True)
parser.add_argument('--emcc', required=True)
parser.add_argument('--clang', required=True)
parser.add_argument('--c2rust', required=True, help='Patched c2rust-transpile binary; see setup-translator.sh.')
args = parser.parse_args()
tools = Path(__file__).resolve().parent
output = Path(args.output).resolve()
if output.exists() and any(output.iterdir()):
    raise SystemExit('Output directory must be empty; regenerate into a new staging directory.')
output.mkdir(parents=True, exist_ok=True)
spec = importlib.util.spec_from_file_location('compile_db', tools / 'compile-db.py')
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
database = module.generate(args.edition, args.source, output / 'database', args.emcc, args.clang)
generated = output / 'generated'
environment = dict(os.environ, C2RUST_NO_HOST_HEADERS='1', C2RUST_TARGET_WASM='1')
llvm_library = str(Path(args.clang).resolve().parent.parent / 'lib')
for key in ['DYLD_LIBRARY_PATH', 'LD_LIBRARY_PATH']:
    environment[key] = llvm_library + (os.pathsep + os.environ[key] if key in os.environ else '')
subprocess.run([str(Path(args.c2rust).resolve()), '--emit-build-files', '--emit-no-std',
                '--emit-c-decl-map', '--fail-on-error', '--disable-refactoring', '--disable-rustfmt',
                '--translate-const-macros', 'none', '--translate-fn-macros', 'none',
                '--edition', '2021', '--output-dir', str(generated), str(database)],
               env=environment, check=True)
if not (generated / 'Cargo.toml').exists():
    raise SystemExit('Translation did not emit a complete crate.')
subprocess.run(['python3', str(tools / 'init-fix.py'), str(generated)], check=True)
subprocess.run(['python3', str(tools / 'normalize.py'), str(generated)], check=True)
# Temporary explicit libpng boundary; never execute setjmp across Rust frames.
lib = generated / 'lib.rs'
lib.write_text(lib.read_text().replace('pub mod png2sdl;', '// External libpng adapter.\n// pub mod png2sdl;'))
print(f'Generated staging sources: {generated}')
print('Review differences, compile, and repeat symbol/browser tests before packaging.')
