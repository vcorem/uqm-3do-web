#!/usr/bin/env python3
"""Pack authored captain atlases into the native masked animation layers.

Only asset cropping/resampling occurs here. The source ANI tables, offsets,
alpha support, frame aliases, timing, and battle simulation are never edited.
"""
from __future__ import annotations

import argparse
import hashlib
import io
import json
from pathlib import Path
import zipfile

from PIL import Image, ImageChops

ROOT = Path(__file__).resolve().parents[1]
ART = ROOT / 'hd/remastered/art/captains'
RENDER = ROOT / 'hd/remastered/render'


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read_png(archive, name):
    return Image.open(io.BytesIO(archive.read(name))).convert('RGBA')


def compile_captains(archive_path, races=None, qa_dir=None):
    inventory = json.loads((ART / 'native-inventory.json').read_text())
    if sha(archive_path.read_bytes()) != inventory['source_archive']['sha256']:
        raise ValueError('Captain source archive differs from the pinned inventory')
    records = []
    ship_records = []
    with zipfile.ZipFile(archive_path) as archive:
        for ship in inventory['ships']:
            race = ship['race']
            if races and race not in races:
                continue
            if sha(archive.read(ship['ani_path'])) != ship['ani_sha256']:
                raise ValueError(f'Changed ANI: {race}')
            config_path = ART / f'{race}.packing.json'
            config = json.loads(config_path.read_text()) if config_path.exists() else {}
            atlas_path = ART / config.get('atlas', f'{race}.png')
            atlas = Image.open(atlas_path).convert('RGBA')
            base_native = read_png(archive, ship['frames'][0]['source_path'])
            cells = ship['reference_sheet']['cells']
            packed = {}
            base_rendered = None
            for cell in cells:
                i = cell['cell']
                source_path = ART / config.get('cell_sources', {}).get(str(i), atlas_path.name)
                sheet = atlas if source_path == atlas_path else Image.open(source_path).convert('RGBA')
                x, y = i % 4, i // 4
                inset = config.get('inset', 0)
                grid = config.get('grid_bounds', [0, 0, sheet.width, sheet.height])
                gx, gy, gr, gb = grid
                bounds = tuple(config.get('cell_bounds', {}).get(str(i), (
                    gx + round(x * (gr - gx) / 4) + inset,
                    gy + round(y * (gb - gy) / 3) + inset,
                    gx + round((x + 1) * (gr - gx) / 4) - inset,
                    gy + round((y + 1) * (gb - gy) / 3) - inset)))
                scene = sheet.crop(bounds).resize(tuple(ship['canvas_size']), Image.Resampling.LANCZOS)
                # Select small authored correction regions without replacing
                # unrelated base pixels. This is source packing only: native
                # descriptor geometry and alpha are restored below as usual.
                base_regions = []
                if i == 0:
                    for region in config.get('base_regions', []):
                        region_path = ART / region['source']
                        region_image = Image.open(region_path).convert('RGBA')
                        source_bounds = tuple(region.get('source_bounds',
                            [0, 0, region_image.width, region_image.height]))
                        destination = tuple(region['destination'])
                        if (len(source_bounds) != 4 or len(destination) != 4
                            or not 0 <= source_bounds[0] < source_bounds[2] <= region_image.width
                            or not 0 <= source_bounds[1] < source_bounds[3] <= region_image.height
                            or not 0 <= destination[0] < destination[2] <= scene.width
                            or not 0 <= destination[1] < destination[3] <= scene.height):
                            raise ValueError(f'Invalid base region: {race}/{region["source"]}')
                        source_layout = region.get('source_layout', 'scene')
                        if source_layout == 'region':
                            selection = region_image.crop(source_bounds).resize(
                                (destination[2] - destination[0], destination[3] - destination[1]),
                                Image.Resampling.LANCZOS)
                        elif source_layout == 'scene':
                            region_scene = region_image.crop(source_bounds).resize(
                                tuple(ship['canvas_size']), Image.Resampling.LANCZOS)
                            selection = region_scene.crop(destination)
                        else:
                            raise ValueError(f'Invalid base region layout: {source_layout}')
                        feather = region.get('feather', 0)
                        if not isinstance(feather, int) or not 0 <= feather <= 3:
                            raise ValueError(f'Invalid base region feather: {feather}')
                        if feather:
                            # Blend only the selected authored RGB boundary.
                            # This temporary packing mask never replaces PNG
                            # alpha; the original native alpha is set below.
                            width, height = selection.size
                            mask = Image.new('L', selection.size)
                            mask.putdata([
                                round(255 * min(1, min(px, py, width - 1 - px,
                                    height - 1 - py) / feather))
                                for py in range(height) for px in range(width)
                            ])
                            scene.paste(selection, destination[:2], mask)
                        else:
                            scene.paste(selection, destination[:2])
                        base_regions.append({
                            'art_path': str(region_path.relative_to(ROOT)),
                            'art_sha256': sha(region_path.read_bytes()),
                            'source_bounds': list(source_bounds),
                            'source_layout': source_layout,
                            'feather': feather,
                            'destination': list(destination),
                        })
                row = ship['frames'][cell['ani_indices'][0]]
                original_bytes = archive.read(row['source_path'])
                if sha(original_bytes) != row['source_sha256']:
                    raise ValueError(f'Changed PNG: {row["source_path"]}')
                native = read_png(archive, row['source_path'])
                rect = tuple(cell['patch_rect_in_cell'])
                # Native neutral/restore patches sometimes exactly repeat base
                # pixels. Use the same authored base for those so releasing a
                # button cannot introduce a different drawing of a neutral face.
                composed = base_native.copy()
                composed.alpha_composite(native, rect[:2])
                neutral = i != 0 and (bool({3, 6, 9, 12}.intersection(cell['ani_indices']))
                                      or ImageChops.difference(composed, base_native).convert('RGB').getbbox() is None)
                if i == 0:
                    base_rendered = scene.copy()
                patch = (base_rendered if neutral else scene).crop(rect)
                # A native action descriptor may include a much larger opaque
                # restore area than the region which actually animates (e.g.
                # Shofixti's lever descriptor includes an unchanged face). Pack
                # only its native changed bounding box from the authored pose;
                # the surrounding region comes from the shared authored base.
                change = ImageChops.difference(composed, base_native).convert('RGB')
                change = ImageChops.lighter(ImageChops.lighter(*change.split()[:2]), change.split()[2])
                changed_bounds = change.point(lambda value: 255 if value > 8 else 0).getbbox()
                if i and not neutral and changed_bounds:
                    clipped = base_rendered.crop(rect)
                    left, top, right, bottom = changed_bounds
                    clipped.paste(scene.crop((left, top, right, bottom)), (left - rect[0], top - rect[1]))
                    patch = clipped
                # Partial-alpha restoration cannot fully erase recolored edge
                # pixels. Keep those pixels on the shared base, as the original
                # native art does. Alpha and control timing remain untouched.
                if i and not neutral:
                    index = cell['ani_indices'][0]
                    rest_rows = [2, 3, 4] if index < 6 else [6] if index < 9 else [9] if index < 12 else [12]
                    if not (race == 'shofixti' and index >= 12):
                        restoration = Image.new('L', tuple(ship['canvas_size']), 0)
                        for rest_index in rest_rows:
                            rest_row = ship['frames'][rest_index]
                            rest = read_png(archive, rest_row['source_path'])
                            layer = Image.new('L', restoration.size, 0)
                            layer.paste(rest.getchannel('A'), tuple(rest_row['position_in_portrait']))
                            restoration = ImageChops.lighter(restoration, layer)
                        fully_erasable = restoration.crop(rect).point(lambda alpha: 255 if alpha == 255 else 0)
                        patch = Image.composite(patch, base_rendered.crop(rect), fully_erasable)
                patch.putalpha(native.getchannel('A'))
                if patch.size != native.size:
                    raise ValueError(f'Invalid patch geometry: {race}/{cell["filename"]}')
                target = RENDER / 'ships' / race / cell['filename']
                target.parent.mkdir(parents=True, exist_ok=True)
                patch.save(target, optimize=True)
                packed[cell['filename']] = patch
                records.append({
                    'race': race, 'path': str(target.relative_to(RENDER)),
                    'sha256': sha(target.read_bytes()), 'size': list(patch.size),
                    'source_path': row['source_path'], 'source_sha256': row['source_sha256'],
                    'alpha_sha256': sha(native.getchannel('A').tobytes()),
                    'ani_indices': cell['ani_indices'], 'hotspot': row['hotspot'],
                    'art_path': str(source_path.relative_to(ROOT)),
                    'art_sha256': sha(source_path.read_bytes()), 'cell': i,
                    'cell_bounds': list(bounds), 'neutral_from_base': neutral,
                    'native_changed_bounds': list(changed_bounds) if changed_bounds else None,
                    **({'base_regions': base_regions} if base_regions else {}),
                })
            ship_records.append({'race': race, 'melee_index': ship['melee_index'],
                                 'ani_path': ship['ani_path'], 'ani_sha256': ship['ani_sha256'],
                                 'rows': [f['filename'] for f in ship['frames']]})
            if qa_dir:
                qa_dir.mkdir(parents=True, exist_ok=True)
                base = packed[ship['frames'][0]['filename']]
                contact = Image.new('RGB', (880, 360), 'black')
                for cell in cells:
                    i = cell['cell']; composed = base.copy()
                    composed.alpha_composite(packed[cell['filename']], tuple(cell['patch_rect_in_cell'][:2]))
                    contact.paste(composed, ((i % 4) * 220, (i // 4) * 120))
                contact.save(qa_dir / f'{race}-packed.png')
                # Same overlay order as DrawCaptainWindowAnimation. These
                # previews supplement, never replace, the live browser tests.
                frames = []
                for rows in ([], [2,3,4,5], [3], [4,3,2,1], [3], [6,7,8], [6],
                             [9,10,11], [9], [12,13,14], [12], [5,8,11,14], [],
                             *([[15,16,17,18,19]] if len(ship['frames']) == 20 else [])):
                    composed = base.copy()
                    for index in rows:
                        row = ship['frames'][index]
                        composed.alpha_composite(packed[row['filename']], tuple(row['position_in_portrait']))
                    frames.append(composed.resize((660,360), Image.Resampling.NEAREST).convert('RGB'))
                frames[0].save(qa_dir / f'{race}-reactions.gif', save_all=True, append_images=frames[1:], duration=450, loop=0)
    result = {'schema_version': 1, 'kind': 'melee-captain-presentation-only',
              'source_archive': inventory['source_archive'], 'ships': ship_records,
              'totals': {'ships': len(ship_records), 'ani_frames': sum(len(s['rows']) for s in ship_records), 'unique_pngs': len(records)},
              'frames': records}
    if not races:
        (RENDER / 'captains-manifest.json').write_text(json.dumps(result, indent=2) + '\n')
    elif qa_dir:
        (qa_dir / 'partial-manifest.json').write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps(result['totals']))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--archive', type=Path, default=ROOT / '.tools/hd-assets/mm-0.8.5-hd-content.uqm')
    parser.add_argument('--race', action='append', help='Only pack this race (does not replace final manifest)')
    parser.add_argument('--qa-dir', type=Path, default=ROOT / '.tools/captain-qa')
    args = parser.parse_args()
    compile_captains(args.archive, set(args.race) if args.race else None, args.qa_dir)
