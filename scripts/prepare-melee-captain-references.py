#!/usr/bin/env python3
"""Read-only native archive inventory and reference packing. No artwork synthesis."""
from pathlib import Path
import hashlib
import io
import json
import math
import zipfile
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / '.tools/captain-reference'
ARCHIVE = ROOT / '.tools/hd-assets/mm-0.8.5-hd-content.uqm'
RACES = ['androsynth', 'arilou', 'chenjesu', 'chmmr', 'druuge', 'human',
         'ilwrath', 'kohrah', 'melnorme', 'mmrnmhrm', 'mycon', 'orz', 'pkunk',
         'shofixti', 'slylandro', 'spathi', 'supox', 'syreen', 'thraddash',
         'umgah', 'urquan', 'utwig', 'vux', 'yehat', 'zoqfotpik']
ROLES = ['background', 'right_held', 'right_transition', 'turn_neutral',
         'left_transition', 'left_held', 'thrust_rest', 'thrust_transition',
         'thrust_held', 'weapon_rest', 'weapon_transition', 'weapon_held',
         'special_rest', 'special_transition', 'special_held']

def sha(data):
    return hashlib.sha256(data).hexdigest()

def native_image(data, transparent_color):
    original = Image.open(io.BytesIO(data))
    image = original.convert('RGBA')
    # Native -1 disables the color key; it does not disable per-pixel alpha.
    # Every captain PNG is truecolor RGB/RGBA, with no palette or tRNS chunk.
    if transparent_color == 0:
        image.putdata([(r, g, b, 0 if (r, g, b) == (0, 0, 0) else a)
                       for r, g, b, a in image.getdata()])
    return original, image

def pack_race(z, race):
    candidates = [n for n in z.namelist() if n.startswith(f'mm-hd/ships/{race}/')
                  and n.endswith('-cap.ani') and not n.endswith('/oldscout-cap.ani')]
    assert len(candidates) == 1, (race, candidates)
    ani = candidates[0]
    ani_data = z.read(ani)
    rows = [line.split() for line in ani_data.decode().splitlines() if line.strip()]
    resources = z.read('mm-hd/mm-hd.rmp').decode().splitlines()
    resource = next(l.split(' = ')[0] for l in resources if l.endswith('addons/' + ani))
    directory = OUT / 'original' / race
    directory.mkdir(parents=True, exist_ok=True)
    (directory / Path(ani).name).write_bytes(ani_data)
    frames, images, unique = [], {}, []
    for index, fields in enumerate(rows):
        filename, transparent, cmap, hx, hy = fields
        source_path = str(Path(ani).parent / filename)
        data = z.read(source_path)
        (directory / filename).write_bytes(data)
        original, rendered = native_image(data, int(transparent))
        images[filename] = rendered
        if filename not in unique:
            unique.append(filename)
        role = ROLES[index] if index < len(ROLES) else 'special_extended'
        if race == 'shofixti' and index >= 12:
            role = ['glory_safe', 'glory_first_press_transition', 'glory_first_stage',
                    'glory_second_press_transition', 'glory_second_stage',
                    'glory_third_press_detonate', 'glory_extended_alias',
                    'glory_death_start'][index - 12]
        aliases = [i for i, row in enumerate(rows) if row[0] == filename]
        frames.append({
            'index': index, 'filename': filename, 'source_path': source_path,
            'source_sha256': sha(data), 'size': list(original.size),
            'hotspot': [int(hx), int(hy)], 'position_in_portrait': [-int(hx), -int(hy)],
            'transparent_color': int(transparent), 'colormap_index': int(cmap),
            'mode': original.mode, 'native_alpha_bbox': list(rendered.getchannel('A').getbbox() or []),
            'native_alpha_extrema': list(rendered.getchannel('A').getextrema()),
            'role': role, 'aliases': aliases, 'unique_cell': unique.index(filename)})
    canvas = images[frames[0]['filename']]
    assert canvas.size == (220, 120)
    atlas = Image.new('RGBA', (880, 360), (0, 0, 0, 0))
    masks = Image.new('L', atlas.size, 0)
    rects = Image.new('L', atlas.size, 0)
    annotated = Image.new('RGB', (928, 480), '#101723')
    draw = ImageDraw.Draw(annotated)
    unique_cells = []
    for cell, filename in enumerate(unique):
        frame = next(f for f in frames if f['filename'] == filename)
        panel = canvas.copy()
        position = tuple(frame['position_in_portrait'])
        panel.alpha_composite(images[filename], position)
        x, y = cell % 4 * 220, cell // 4 * 120
        atlas.alpha_composite(panel, (x, y))
        masks.paste(images[filename].getchannel('A'), (x + position[0], y + position[1]))
        ImageDraw.Draw(rects).rectangle((x + position[0], y + position[1],
            x + position[0] + frame['size'][0] - 1,
            y + position[1] + frame['size'][1] - 1), fill=255)
        ax, ay = cell % 4 * 232 + 6, cell // 4 * 156 + 28
        annotated.paste(panel.convert('RGB'), (ax, ay))
        draw.text((ax, ay - 23), f"{cell:02d} {filename}", fill='#ffffff')
        draw.text((ax, ay + 122), 'ANI ' + ','.join(str(i) for i in frame['aliases']), fill='#a9c1d5')
        unique_cells.append({'cell': cell, 'filename': filename, 'ani_indices': frame['aliases'],
            'atlas_rect': [x, y, x + 220, y + 120],
            'patch_rect_in_cell': [position[0], position[1], position[0]+frame['size'][0], position[1]+frame['size'][1]],
            'native_alpha_mask': f'{race}-unique-alpha.png'})
    sheets = OUT / 'sheets'
    sheets.mkdir(exist_ok=True)
    atlas.save(sheets / f'{race}-unique-composited.png')
    masks.save(sheets / f'{race}-unique-alpha.png')
    rects.save(sheets / f'{race}-unique-rectangles.png')
    annotated.save(sheets / f'{race}-unique-labeled.png')
    entry = {'race': race, 'melee_index': RACES.index(race), 'resource': resource,
             'ani_path': ani, 'ani_sha256': sha(ani_data), 'frame_count': len(frames),
             'unique_png_count': len(unique), 'canvas_size': list(canvas.size),
             'frames': frames, 'reference_sheet': {
                 'path': str((sheets/f'{race}-unique-composited.png').relative_to(ROOT)),
                 'columns': 4, 'rows': 3, 'cell_size': [220, 120],
                 'unused_cells': list(range(len(unique), 12)), 'cells': unique_cells}}
    (sheets / f'{race}-cells.json').write_text(json.dumps(entry, indent=2) + '\n')
    return entry

def main():
    with zipfile.ZipFile(ARCHIVE) as z:
        ships = {}
        for race in ['human', 'urquan'] + [r for r in RACES if r not in ('human', 'urquan')]:
            ships[race] = pack_race(z, race)
            print(race, ships[race]['frame_count'], ships[race]['unique_png_count'], flush=True)
    result = {
        'schema_version': 1,
        'source_archive': {'path': str(ARCHIVE.relative_to(ROOT)), 'sha256': sha(ARCHIVE.read_bytes())},
        'scope': 'All 25 playable Melee captain resources; excludes flagship, Sa-Matra, and campaign old Shofixti.',
        'coordinate_convention': 'ANI fields: filename transparent_color colormap_index hotspot_x hotspot_y. Overlay top-left relative to portrait is negative hotspot. Rectangles use exclusive right/bottom.',
        'shared_slots': {str(i): role for i, role in enumerate(ROLES)},
        'native_draw_sequences': {
            'left_rest': [2, 3], 'left_transition': [2, 3, 4], 'left_held': [2, 3, 4, 5],
            'right_rest': [4, 3], 'right_transition': [4, 3, 2], 'right_held': [4, 3, 2, 1],
            'simultaneous_action_order': ['turn', 'thrust', 'weapon', 'special']},
        'animation_source': {
            'rust_status': 'rust/uqm-engine-hd/src/uqm/status.rs',
            'rust_shofixti': 'rust/uqm-engine-hd/src/uqm/ships/shofixti/shofixti.rs',
            'c_status': '.tools/hd-source/src/uqm/status.c',
            'c_shofixti': '.tools/hd-source/src/uqm/ships/shofixti/shofixti.c'},
        'special_cases': {
            'shofixti': '20 ANI rows, 12 unique PNGs. Special base index12; release handling advances the latched lever. special_offset5 triggers self_destruct; PostProcessStatus draws DecFrameIndex(background), row19, at explosion start. Preserve rows15–19 and all aliases.',
            'chmmr': '12 unique PNGs: thrust and weapon have distinct rest, transition, held PNGs.',
            'vux': '9 unique PNGs: right_held index1 and left_held index5 share intruder-cap-001.png.',
            'all': 'Damage/death rectangles and flashes are procedural status drawing, not additional captain PNGs. No ambient idle loop is present in native captain control.'},
        'totals': {'ships': len(ships), 'ani_frames': sum(v['frame_count'] for v in ships.values()),
                   'unique_pngs': sum(v['unique_png_count'] for v in ships.values())},
        'ships': [ships[r] for r in RACES]}
    path = ROOT / 'hd/remastered/art/captains/native-inventory.json'
    path.write_text(json.dumps(result, indent=2) + '\n')
    (OUT / 'native-inventory.json').write_text(json.dumps(result, indent=2) + '\n')

if __name__ == '__main__':
    main()
