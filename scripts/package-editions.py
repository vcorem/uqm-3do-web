#!/usr/bin/env python3
"""Package independently built engines behind a lightweight edition picker.

Original and HD are required. The Rust builds of each edition are optional; when
given, they are published next to the C builds as /original-rust/ and /hd-rust/,
with their own browser save folders so C and Rust saves never mix.
"""
import argparse
import json
from pathlib import Path
import shutil

ENGINE = ('index.html', 'uqm-debug.js', 'uqm-debug.wasm', 'uqm-debug.data', 'uqm-debug.worker.js', 'COPYING.txt', 'toolchain-fixes.json', 'game-help.js')
EXTRA = {'original': ('netplay-client.js',), 'hd': ('content-licenses.txt', 'netplay-client.js')}
# The in-frame start screen and online lobby (web/online) and its art.
ENGINE += ('online-ui.js', 'online-art.json', 'online-art.png', 'online-stars.png', 'online-title.png')
# Rust pages get these settings before any page script runs (see the shells' UQM_VARIANT).
RUST_VARIANT = {
    'original': {'engine': 'rust', 'label': 'Rust engine (port)', 'saveDir': '/home/web_user/.uqm-rust'},
    'hd': {'engine': 'rust', 'label': 'Rust engine (port)', 'saveDir': '/home/web_user/.uqm-hd-rust'},
}


def package(builds, output):
    """builds: list of (source directory, edition, published folder, Rust variant or None)."""
    output = output.resolve()
    for source, _, _, _ in builds:
        if source == output or source in output.parents or output in source.parents:
            raise ValueError('The output must be separate from every engine build directory.')
        if not source.is_dir():
            raise ValueError('Missing engine build directory: ' + str(source))
    for source, edition, _, _ in builds:
        for name in ENGINE + EXTRA[edition]:
            path = source / name
            if not path.is_file() or not path.stat().st_size:
                raise ValueError('Missing build artifact: ' + str(path))
    root = Path(__file__).resolve().parent.parent
    output.mkdir(parents=True, exist_ok=True)
    for source, edition, folder, variant in builds:
        destination = output / folder
        destination.mkdir(exist_ok=True)
        for name in ENGINE + EXTRA[edition]:
            shutil.copy2(source / name, destination / name)
        for name in ('engine-language.json', 'link-inputs.json'):
            metadata = source / name
            if metadata.is_file():
                shutil.copy2(metadata, destination / name)
            else:
                (destination / name).unlink(missing_ok=True)
        page = destination / 'index.html'
        html = page.read_text()
        if edition == 'original':
            html = html.replace('</header>', '<a id="choose-edition" href="../index.html">Choose edition</a></header>', 1)
        if variant:
            if html.count('<head>') != 1:
                raise ValueError('Cannot mark the Rust page: expected one <head> in ' + str(page))
            html = html.replace('<head>', '<head>\n<script>window.UQM_VARIANT = ' + json.dumps(variant) + ';</script>', 1)
        page.write_text(html)
    for name in ('index.html', 'editions.css', 'editions.js'):
        shutil.copy2(root / 'web' / 'editions' / name, output / name)
    if not any(variant for _, _, _, variant in builds):
        # Without Rust builds the picker must not offer the Rust engine.
        picker = output / 'index.html'
        html = picker.read_text()
        if html.count('data-engines="c,rust"') != 1:
            raise ValueError('The picker page has no engine marker.')
        picker.write_text(html.replace('data-engines="c,rust"', 'data-engines="c"'))
    print('Packaged ' + ', '.join(folder for _, _, folder, _ in builds) + ': ' + str(output / 'index.html'))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--original', type=Path, required=True, help='Original C build')
    parser.add_argument('--hd', type=Path, required=True, help='HD C build')
    parser.add_argument('--original-rust', type=Path, help='Original Rust build')
    parser.add_argument('--hd-rust', type=Path, help='HD Rust build')
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    builds = [(args.original.resolve(), 'original', 'original', None), (args.hd.resolve(), 'hd', 'hd', None)]
    if args.original_rust:
        builds.append((args.original_rust.resolve(), 'original', 'original-rust', RUST_VARIANT['original']))
    if args.hd_rust:
        builds.append((args.hd_rust.resolve(), 'hd', 'hd-rust', RUST_VARIANT['hd']))
    package(builds, args.output)
