#!/usr/bin/env python3
"""Package independently built engines behind a lightweight edition picker.

Original and HD are required. The Rust builds of each edition are optional; when
given, they are published next to the C builds as /original-rust/ and /hd-rust/,
with their own browser save folders so C and Rust saves never mix.

Cosmo Clash, a separate game, is optional too: --cosmoclash takes its Vite build
(made with --base /cosmoclash/) and publishes it as /cosmoclash/ with a third
card on the picker. Without it the picker has no Cosmo Clash card.
"""
import argparse
import json
from pathlib import Path
import re
import shutil

ENGINE = ('index.html', 'uqm-debug.js', 'uqm-debug.wasm', 'uqm-debug.data', 'uqm-debug.worker.js', 'COPYING.txt', 'toolchain-fixes.json', 'game-help.js', 'engine-loader.js')
EXTRA = {'original': ('netplay-client.js',), 'hd': ('content-licenses.txt', 'netplay-client.js')}
# The in-frame start screen and online lobby (web/online) and its art.
ENGINE += ('online-ui.js', 'online-art.json', 'online-art.png', 'online-stars.png', 'online-title.png')
# Rust pages get these settings before any page script runs (see the shells' UQM_VARIANT).
RUST_VARIANT = {
    'original': {'engine': 'rust', 'label': 'Rust engine (port)', 'saveDir': '/home/web_user/.uqm-rust'},
    'hd': {'engine': 'rust', 'label': 'Rust engine (port)', 'saveDir': '/home/web_user/.uqm-hd-rust'},
}
COSMO_FOLDER = 'cosmoclash'
COSMO_BASE = '/' + COSMO_FOLDER + '/'
COSMO_REQUIRED = ('index.html', 'cosmoclash.wasm')
COSMO_CARD_IMAGE = 'cosmoclash-card.jpg'
# Cloudflare static assets accept files up to 25 MiB (deploy/split-data.py splits only UQM data files).
ASSET_LIMIT = 25 * 1024 * 1024
# The picker's Cosmo Clash parts: the card and the noscript link.
COSMO_BLOCK = re.compile(r'\s*<!-- cosmoclash(?::[^>]*)? -->.*?<!-- /cosmoclash -->', re.S)
PAGE_URL = re.compile(r'<(?:script|link|img|source|audio|video|iframe)\b[^>]*?\b(?:src|href)="([^"]*)"', re.I)


def check_cosmoclash(source):
    """Refuses a Cosmo Clash build that does not work at /cosmoclash/ on this site."""
    if not source.is_dir():
        raise ValueError('Missing Cosmo Clash build directory: ' + str(source))
    for name in COSMO_REQUIRED:
        path = source / name
        if not path.is_file() or not path.stat().st_size:
            raise ValueError('Missing Cosmo Clash build artifact: ' + str(path))
    html = (source / 'index.html').read_text()
    urls = PAGE_URL.findall(html)
    if not any(url.startswith(COSMO_BASE + 'assets/') for url in urls):
        raise ValueError('The Cosmo Clash page loads no script from ' + COSMO_BASE + 'assets/. '
                         'Build it with: npm run build --prefix cosmoclash -- --base ' + COSMO_BASE)
    for url in urls:
        if url.startswith('data:'):
            continue
        # The site sends Cross-Origin-Embedder-Policy: require-corp, which blocks other origins.
        if re.match(r'^(?:[a-z][a-z0-9+.-]*:|//)', url, re.I):
            raise ValueError('The Cosmo Clash page loads a file from another origin: ' + url)
        if not url.startswith(COSMO_BASE):
            raise ValueError('The Cosmo Clash page has a URL outside ' + COSMO_BASE + ': ' + url)
        if not (source / url[len(COSMO_BASE):].split('?')[0].split('#')[0]).is_file():
            raise ValueError('The Cosmo Clash page loads a missing file: ' + url)
    # The scripts may name files from the site root ('/models/gondi.glb'): the
    # game adds the base at run time (cosmoclash/src/base.ts). The browser test
    # tests/browser/cosmoclash-site.mjs plays a battle and checks the real requests.
    for path in sorted(source.rglob('*')):
        if path.is_file() and path.stat().st_size > ASSET_LIMIT:
            raise ValueError('Cosmo Clash file larger than 25 MiB: ' + str(path))


def package(builds, output, cosmoclash=None):
    """builds: list of (source directory, edition, published folder, Rust variant or None).
    cosmoclash: the Cosmo Clash build directory, or None."""
    output = output.resolve()
    for source in [source for source, _, _, _ in builds] + ([cosmoclash] if cosmoclash else []):
        if source == output or source in output.parents or output in source.parents:
            raise ValueError('The output must be separate from every engine build directory.')
    for source, _, _, _ in builds:
        if not source.is_dir():
            raise ValueError('Missing engine build directory: ' + str(source))
    if cosmoclash:
        check_cosmoclash(cosmoclash)
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
    picker = output / 'index.html'
    html = picker.read_text()
    if html.count('data-games="uqm,cosmoclash"') != 1 or len(COSMO_BLOCK.findall(html)) != 2:
        raise ValueError('The picker page has no Cosmo Clash markers.')
    # A new copy each time: Vite's file names change with their content.
    shutil.rmtree(output / COSMO_FOLDER, ignore_errors=True)
    if cosmoclash:
        shutil.copytree(cosmoclash, output / COSMO_FOLDER, ignore=shutil.ignore_patterns('.*'))
        shutil.copy2(root / 'web' / 'editions' / COSMO_CARD_IMAGE, output / COSMO_CARD_IMAGE)
    else:
        (output / COSMO_CARD_IMAGE).unlink(missing_ok=True)
        picker.write_text(COSMO_BLOCK.sub('', html).replace('data-games="uqm,cosmoclash"', 'data-games="uqm"'))
    if not any(variant for _, _, _, variant in builds):
        # Without Rust builds the picker must not offer the Rust engine.
        picker = output / 'index.html'
        html = picker.read_text()
        if html.count('data-engines="c,rust"') != 1:
            raise ValueError('The picker page has no engine marker.')
        picker.write_text(html.replace('data-engines="c,rust"', 'data-engines="c"'))
    folders = [folder for _, _, folder, _ in builds] + ([COSMO_FOLDER] if cosmoclash else [])
    print('Packaged ' + ', '.join(folders) + ': ' + str(output / 'index.html'))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--original', type=Path, help='Original C build (required)')
    parser.add_argument('--hd', type=Path, help='HD C build (required)')
    parser.add_argument('--original-rust', type=Path, help='Original Rust build')
    parser.add_argument('--hd-rust', type=Path, help='HD Rust build')
    parser.add_argument('--cosmoclash', type=Path, help='Cosmo Clash build (vite build --base /cosmoclash/)')
    parser.add_argument('--output', type=Path, help='Site folder (required)')
    parser.add_argument('--check-cosmoclash', type=Path, metavar='DIR',
                        help='Only check that a Cosmo Clash build works at /cosmoclash/, then stop')
    args = parser.parse_args()
    if args.check_cosmoclash:
        try:
            check_cosmoclash(args.check_cosmoclash.resolve())
        except ValueError as error:
            parser.exit(1, 'package-editions.py: ' + str(error) + '\n')
        print('Cosmo Clash build is ready for ' + COSMO_BASE + ': ' + str(args.check_cosmoclash))
        parser.exit(0)
    if not (args.original and args.hd and args.output):
        parser.error('--original, --hd and --output are required')
    builds = [(args.original.resolve(), 'original', 'original', None), (args.hd.resolve(), 'hd', 'hd', None)]
    if args.original_rust:
        builds.append((args.original_rust.resolve(), 'original', 'original-rust', RUST_VARIANT['original']))
    if args.hd_rust:
        builds.append((args.hd_rust.resolve(), 'hd', 'hd-rust', RUST_VARIANT['hd']))
    package(builds, args.output, args.cosmoclash.resolve() if args.cosmoclash else None)
