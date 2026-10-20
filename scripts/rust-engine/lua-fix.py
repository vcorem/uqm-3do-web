#!/usr/bin/env python3
"""Build the pinned syntax-aware helper and propagate contained Lua failures."""
import argparse
import os
from pathlib import Path
import subprocess


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('engine', type=Path)
    action = parser.add_mutually_exclusive_group(required=True)
    for value in ('apply', 'check', 'audit'):
        action.add_argument('--' + value, dest='mode', action='store_const', const='--' + value)
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[2]
    environment = os.environ.copy()
    isolated = root / '.tools/cargo/bin/cargo'
    if isolated.is_file():
        environment.setdefault('CARGO_HOME', str(root / '.tools/cargo'))
        environment.setdefault('RUSTUP_HOME', str(root / '.tools/rustup'))
        environment['PATH'] = str(isolated.parent) + os.pathsep + environment.get('PATH', '')
    target = root / '.tools/rust-lua-transform-target'
    environment['CARGO_TARGET_DIR'] = str(target)
    subprocess.run([
        str(isolated) if isolated.is_file() else 'cargo', '+1.85.1', 'build',
        '--release', '--locked', '--manifest-path',
        str(root / 'scripts/rust-engine/lua-propagate/Cargo.toml'),
    ], cwd=root, env=environment, check=True)
    binary = target / 'release/uqm-lua-propagate'
    subprocess.run([str(binary), str(args.engine.resolve()), args.mode], cwd=root, env=environment, check=True)


if __name__ == '__main__':
    main()
