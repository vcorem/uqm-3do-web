#!/usr/bin/env python3
"""Package two independently built engines behind a lightweight edition picker."""
import argparse
from pathlib import Path
import shutil

ENGINE = ('index.html', 'uqm-debug.js', 'uqm-debug.wasm', 'uqm-debug.data', 'uqm-debug.worker.js', 'COPYING.txt', 'toolchain-fixes.json', 'game-help.js')


def package(original, hd, output):
    original, hd, output = (p.resolve() for p in (original, hd, output))
    for source in (original, hd):
        if source == output or source in output.parents or output in source.parents:
            raise ValueError('The output must be separate from both engine build directories.')
    for source, extra in ((original, ('netplay-client.js',)), (hd, ('content-licenses.txt',))):
        for name in ENGINE + extra:
            path = source / name
            if not path.is_file() or not path.stat().st_size:
                raise ValueError('Missing build artifact: ' + str(path))
    root = Path(__file__).resolve().parent.parent
    output.mkdir(parents=True, exist_ok=True)
    for source, edition, extra in ((original, 'original', ('netplay-client.js',)), (hd, 'hd', ('content-licenses.txt',))):
        destination = output / edition
        destination.mkdir(exist_ok=True)
        for name in ENGINE + extra:
            shutil.copy2(source / name, destination / name)
        if edition == 'original':
            page = destination / 'index.html'
            html = page.read_text()
            html = html.replace('</header>', '<a id="choose-edition" href="../index.html">Choose edition</a></header>', 1)
            page.write_text(html)
    for name in ('index.html', 'editions.css', 'editions.js'):
        shutil.copy2(root / 'web' / 'editions' / name, output / name)
    print('Packaged Original 3DO and Redrawn HD: ' + str(output / 'index.html'))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--original', type=Path, required=True)
    parser.add_argument('--hd', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    package(args.original, args.hd, args.output)
