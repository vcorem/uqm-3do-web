#!/usr/bin/env python3
"""Stage presentation resources without replacing any native collision images.

The renderer reads a side tree. Music uses an explicit optional resource addon.
This script can also publish the two small page controls after either C or Rust
linking; the heavy renderer images and audio live inside the native preload.
"""
import argparse
import json
from pathlib import Path
import shutil

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / 'hd' / 'remastered'
PAGE_FILES = ('melee-presentation.js', 'melee-presentation.css')
MUSIC = {'music.meleemenu': 'fleet-menu.ogg', 'music.battle': 'battle.ogg'}


def stage_content(destination):
    render = SOURCE / 'render'
    if not render.is_dir() or not any(render.rglob('*.png')):
        raise ValueError('No compiled Melee presentation images: ' + str(render))
    for name in MUSIC.values():
        path = SOURCE / 'music' / name
        if not path.is_file() or path.read_bytes()[:4] != b'OggS':
            raise ValueError('Missing or invalid Melee soundtrack: ' + str(path))
    # Only directories owned by this feature are synchronized. Original sprites,
    # ANI hotspots, fonts, sound effects and media packages remain unchanged.
    target = destination / 'remastered' / 'render'
    if target.exists():
        shutil.rmtree(target)
    shutil.copytree(render, target, ignore=shutil.ignore_patterns('.*'))
    addon = destination / 'addons' / 'remastered'
    addon.mkdir(parents=True, exist_ok=True)
    music = addon / 'music'
    music.mkdir(exist_ok=True)
    for name in MUSIC.values():
        shutil.copy2(SOURCE / 'music' / name, music / name)
    (addon / 'remastered.rmp').write_text(''.join(
        f'{key} = MUSICRES:addons/remastered/music/{name}\n' for key, name in MUSIC.items()))
    # The manifest travels with the native preload for reproducibility.
    for name in ('manifest.json', 'credits.md', 'music/manifest.json'):
        source = SOURCE / name
        if source.is_file():
            target = destination / 'remastered' / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(source, target)
    print('Staged Melee render side images and optional native soundtrack:', destination)


def publish_page(destination):
    destination.mkdir(parents=True, exist_ok=True)
    for name in PAGE_FILES:
        shutil.copy2(ROOT / 'hd' / 'wasm' / name, destination / name)
    # Reuse the authored arena art in the launcher without another engine load.
    nebula = SOURCE / 'art' / 'nebula.png'
    if nebula.is_file():
        shutil.copy2(nebula, destination / 'melee-nebula.png')
        manifest = destination / 'online-art.json'
        if manifest.is_file():
            art = json.loads(manifest.read_text())
            art['images']['remastered'] = 'melee-nebula.png'
            manifest.write_text(json.dumps(art, separators=(',', ':')) + '\n')
    print('Published Melee presentation controls:', destination)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--content', type=Path)
    parser.add_argument('--output', type=Path)
    args = parser.parse_args()
    if not args.content and not args.output:
        parser.error('provide --content or --output')
    if args.content:
        stage_content(args.content)
    if args.output:
        publish_page(args.output)


if __name__ == '__main__':
    main()
