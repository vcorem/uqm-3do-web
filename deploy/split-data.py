#!/usr/bin/env python3
"""Copy a packaged edition site and split each Emscripten data file into parts.

Cloudflare static assets accept files up to 25 MiB. The Worker joins the parts
back into one response, so the game still fetches a single uqm-debug.data.
"""
import argparse
import hashlib
import json
from pathlib import Path
import shutil

PART_SIZE = 24 * 1024 * 1024
ASSET_LIMIT = 25 * 1024 * 1024
DATA_NAME = 'uqm-debug.data'


def split(source, output, manifest_path):
    source, output = source.resolve(), output.resolve()
    if output == source or source in output.parents or output in source.parents:
        raise ValueError('The output must be separate from the packaged site.')
    if output.exists():
        shutil.rmtree(output)
    shutil.copytree(source, output)
    manifest = {}
    by_hash = {}
    for data in sorted(output.glob('*/' + DATA_NAME)):
        url = '/' + data.relative_to(output).as_posix()
        size = data.stat().st_size
        digest = hashlib.sha256()
        with data.open('rb') as handle:
            while chunk := handle.read(PART_SIZE):
                digest.update(chunk)
        sha = digest.hexdigest()
        if sha in by_hash:
            # The C and Rust builds of an edition package the same content, and the Melee
            # Remastered pages package the HD builds again: share one set of parts.
            manifest[url] = dict(manifest[by_hash[sha]])
            data.unlink()
            print('%s: same bytes as %s, parts shared' % (url, by_hash[sha]))
            continue
        parts = []
        with data.open('rb') as handle:
            while chunk := handle.read(PART_SIZE):
                part = data.with_name('%s.p%03d' % (DATA_NAME, len(parts)))
                part.write_bytes(chunk)
                parts.append('/' + part.relative_to(output).as_posix())
        data.unlink()
        by_hash[sha] = url
        manifest[url] = {'size': size, 'sha256': sha, 'parts': parts}
        print('%s: %d bytes in %d parts' % (url, size, len(parts)))
    required = {'/original/' + DATA_NAME, '/hd/' + DATA_NAME, '/melee/' + DATA_NAME}
    allowed = required | {'/original-rust/' + DATA_NAME, '/hd-rust/' + DATA_NAME, '/melee-rust/' + DATA_NAME}
    if not required <= set(manifest) <= allowed:
        raise ValueError('Expected the Original, HD and Melee data files (and optional Rust copies), found: ' + ', '.join(sorted(manifest)))
    for path in output.rglob('*'):
        if path.is_file() and path.stat().st_size > ASSET_LIMIT:
            raise ValueError('Asset larger than 25 MiB: ' + str(path))
    manifest_path.write_text(json.dumps(manifest, indent=2) + '\n')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--site', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--manifest', type=Path, required=True)
    args = parser.parse_args()
    split(args.site, args.output, args.manifest)
