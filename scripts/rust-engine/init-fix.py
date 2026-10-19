#!/usr/bin/env python3
"""Make C2Rust's generated static initializers explicit before UQM startup.

Run only after transpilation is finished. This owns generated initializer
attributes, the generated uqm.rs entry wrapper, uqm_engine_init.rs and the tiny
C host entry adapter. It deliberately does not change the generated manifest or
lib.rs: the crate owner must include `mod uqm_engine_init;` in lib.rs.

The generated C initializers contain only C static-initializer expressions;
their code must not call back into engine startup. Review the emitted report
when upgrading C2Rust or the source pins. Foreign code must call initialization
before accessing generated globals; the UQM entry does this before any use.
"""

import argparse
import hashlib
import json
from pathlib import Path
import re


INITIALIZER = 'unsafe extern "C" fn c2rust_run_static_initializers() {'
CONSTRUCTOR = re.compile(
    r'#\[used\]\n'
    r'(?:#\[cfg_attr\([^\n]+\)\]\n)+'
    r'static INIT_ARRAY: \[unsafe extern "C" fn\(\); 1\] = '
    r'\[c2rust_run_static_initializers\];\n?'
)
MARKER = '// Explicit engine initialization replaces platform constructor tables.'
MAIN = re.compile(
    r'unsafe fn main_0\(\s*'
    r'mut argc: ::core::ffi::c_int,\s*'
    r'mut argv: \*mut \*mut ::core::ffi::c_char,\s*'
    r'\) -> ::core::ffi::c_int \{'
)
PATCHED_MAIN = (
    '#[export_name = "uqm_rust_main"]\n'
    'pub unsafe extern "C" fn main_0(\n'
    '    mut argc: ::core::ffi::c_int,\n'
    '    mut argv: *mut *mut ::core::ffi::c_char,\n'
    ') -> ::core::ffi::c_int {\n'
    '    crate::uqm_engine_init::uqm_rust_engine_init();'
)


def final_wrapper(source: str, begin: int) -> int:
    """Find the wrapper's closing brace without treating string braces as code."""
    index = source.index('{', begin)
    depth = 0
    while index < len(source):
        char = source[index]
        if source.startswith('//', index):
            end = source.find('\n', index)
            index = len(source) if end < 0 else end + 1
            continue
        if source.startswith('/*', index):
            end = source.find('*/', index + 2)
            if end < 0:
                raise ValueError('unterminated comment in host wrapper')
            index = end + 2
            continue
        if char == '"':
            index += 1
            while index < len(source) and source[index] != '"':
                index += 2 if source[index] == '\\' else 1
            if index >= len(source):
                raise ValueError('unterminated string in host wrapper')
        elif char == '{':
            depth += 1
        elif char == '}':
            depth -= 1
            if depth == 0:
                if source[index + 1:].strip():
                    raise ValueError('host wrapper is not the final generated item')
                return index + 1
        index += 1
    raise ValueError('unterminated host wrapper')


def prepare(directory: Path) -> dict:
    directory = directory.resolve()
    changes = {}
    initializers = []
    for path in sorted((directory / 'src').rglob('*.rs')):
        source = path.read_text()
        count = source.count(INITIALIZER)
        if count == 0:
            if 'static INIT_ARRAY' in source:
                raise ValueError(f'unrecognized constructor table in {path}')
            continue
        if count != 1:
            raise ValueError(f'expected one initializer in {path}, found {count}')
        relative = path.relative_to(directory).as_posix()
        symbol = 'uqm_c2rust_init_' + hashlib.sha256(relative.encode()).hexdigest()[:16]
        attribute = f'#[export_name = "{symbol}"]\n'
        if attribute + INITIALIZER not in source:
            source = source.replace(INITIALIZER, attribute + INITIALIZER, 1)
        source, replaced = CONSTRUCTOR.subn(MARKER + '\n', source)
        if replaced != 1 and MARKER not in source:
            raise ValueError(f'expected known constructor registration in {path}')
        if 'static INIT_ARRAY' in source:
            raise ValueError(f'constructor registration remains in {path}')
        changes[path] = source
        initializers.append({'path': relative, 'symbol': symbol})

    entry = directory / 'src/uqm.rs'
    source = changes.get(entry, entry.read_text())
    wrapper = '\npub fn main() {'
    if wrapper in source:
        begin = source.index(wrapper)
        # C2Rust puts its host-only std argv/exit wrapper last. Fail rather than
        # deleting unrelated generated code if that format ever changes.
        trailer = source[begin:]
        if '::std::env::args()' not in trailer or '::std::process::exit(' not in trailer:
            raise ValueError('unexpected C2Rust host main wrapper')
        final_wrapper(source, begin)
        source = source[:begin] + '\n'
    if PATCHED_MAIN not in source:
        source, replaced = MAIN.subn(PATCHED_MAIN, source)
        if replaced != 1:
            raise ValueError(f'expected exactly one generated C main, found {replaced}')
    if '::std::' in source or 'pub fn main()' in source:
        raise ValueError('std-dependent startup code remains')
    changes[entry] = source

    declarations = '\n'.join(f'    fn {item["symbol"]}();' for item in initializers)
    calls = '\n'.join(f'            {item["symbol"]}();' for item in initializers)
    aggregator = '''// Generated by init-fix.py. Re-run after regenerating C2Rust output.
// Original C static initialization must finish before any game state is used.
// Implicit platform constructor tables are disabled to prevent duplicate runs.
use core::sync::atomic::{AtomicU8, Ordering};

static INITIALIZATION: AtomicU8 = AtomicU8::new(0);

extern "C" {
DECLARATIONS
}

/// Initialize translated C static data exactly once, before starting threads.
///
/// # Safety
/// Every generated global must remain unused until this returns. Initializer
/// functions may not recursively call this function or invoke engine callbacks.
/// Subsequent calls are no-ops; concurrent callers wait for the first caller.
#[no_mangle]
pub unsafe extern "C" fn uqm_rust_engine_init() {
    match INITIALIZATION.compare_exchange(0, 1, Ordering::Acquire, Ordering::Acquire) {
        Ok(_) => {
            // SAFETY: The once state gives this caller exclusive initialization
            // access; these functions only materialize C static expressions.
            unsafe {
CALLS
            }
            INITIALIZATION.store(2, Ordering::Release);
        }
        Err(2) => {}
        Err(_) => {
            while INITIALIZATION.load(Ordering::Acquire) != 2 {
                core::hint::spin_loop();
            }
        }
    }
}
'''.replace('DECLARATIONS', declarations).replace('CALLS', calls)
    changes[directory / 'uqm_engine_init.rs'] = aggregator
    changes[directory / 'uqm_rust_entry.c'] = '''/* Host startup adapter; all game startup logic is translated Rust.
 * Compile with the same Emscripten flags as the game so the compiler lowers
 * main to its required __main_argc_argv entry point. */
extern int uqm_rust_main(int argc, char **argv);
int main(int argc, char **argv) { return uqm_rust_main(argc, argv); }
'''
    report = {
        'initializer_count': len(initializers),
        'initializers': initializers,
        'entry_symbol': 'uqm_rust_main',
        'entry_adapter': 'uqm_rust_entry.c',
        'required_lib_item': 'mod uqm_engine_init;',
        'initialization_order': 'sorted source paths; only C static-initializer expressions',
    }
    # Validate every transformation before changing any generated source.
    for path, content in changes.items():
        path.write_text(content)
    (directory / 'init-fix-report.json').write_text(json.dumps(report, indent=2) + '\n')
    return report


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('directory', type=Path)
    args = parser.parse_args()
    print(json.dumps(prepare(args.directory), indent=2))
