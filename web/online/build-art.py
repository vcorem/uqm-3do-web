#!/usr/bin/env python3
"""Build the art for the in-frame start screen and online lobby.

The art comes from the game's own content, so the screens look like part of
the game: its bitmap fonts, the Super-Melee title lettering and star field,
and the title picture. Original reads sc2/content; HD reads the pinned HD
content package (a .uqm zip, or a folder that contains mm-hd/).

Output (in --output):
  online-art.png    glyph atlas (white glyphs on alpha) and the title lettering
  online-art.json   font metrics, sprite rectangles and image names
  online-title.png  the title picture (start screen)
  online-stars.png  the Super-Melee star field (lobby)

Only the Python standard library is used: CI has no imaging packages.
The art keeps the licence of the content it comes from (see COPYING.txt
and, for HD, content-licenses.txt next to the page).
"""
import argparse
import io
import json
import struct
import zipfile
import zlib
from pathlib import Path

# ---------------------------------------------------------------- PNG codec


def _unfilter(raw, width, height, bpp, stride):
    out = bytearray(height * stride)
    prev = bytearray(stride)
    pos = 0
    for y in range(height):
        kind = raw[pos]
        line = bytearray(raw[pos + 1:pos + 1 + stride])
        pos += 1 + stride
        if kind == 1:
            for i in range(bpp, stride):
                line[i] = (line[i] + line[i - bpp]) & 255
        elif kind == 2:
            for i in range(stride):
                line[i] = (line[i] + prev[i]) & 255
        elif kind == 3:
            for i in range(stride):
                left = line[i - bpp] if i >= bpp else 0
                line[i] = (line[i] + ((left + prev[i]) >> 1)) & 255
        elif kind == 4:
            for i in range(stride):
                a = line[i - bpp] if i >= bpp else 0
                b = prev[i]
                c = prev[i - bpp] if i >= bpp else 0
                p = a + b - c
                pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
                pred = a if pa <= pb and pa <= pc else (b if pb <= pc else c)
                line[i] = (line[i] + pred) & 255
        elif kind != 0:
            raise ValueError('Unknown PNG filter ' + str(kind))
        out[y * stride:(y + 1) * stride] = line
        prev = line
    return out


class Image:
    """RGBA pixels, 8 bits per channel."""

    def __init__(self, width, height, pixels=None):
        self.width, self.height = width, height
        self.px = pixels if pixels is not None else bytearray(width * height * 4)

    def get(self, x, y):
        i = (y * self.width + x) * 4
        return tuple(self.px[i:i + 4])

    def put(self, x, y, rgba):
        i = (y * self.width + x) * 4
        self.px[i:i + 4] = bytes(rgba)

    def alpha(self, x, y):
        return self.px[(y * self.width + x) * 4 + 3]

    def crop(self, x0, y0, x1, y1):
        out = Image(x1 - x0, y1 - y0)
        for y in range(y0, y1):
            start = (y * self.width + x0) * 4
            out.px[(y - y0) * out.width * 4:(y - y0 + 1) * out.width * 4] = self.px[start:start + out.width * 4]
        return out

    def paste(self, other, x0, y0):
        for y in range(other.height):
            start = ((y0 + y) * self.width + x0) * 4
            self.px[start:start + other.width * 4] = other.px[y * other.width * 4:(y + 1) * other.width * 4]


def read_png(data):
    if data[:8] != b'\x89PNG\r\n\x1a\n':
        raise ValueError('Not a PNG file')
    pos, idat, palette, trns = 8, bytearray(), None, None
    while pos < len(data):
        length, kind = struct.unpack('>I4s', data[pos:pos + 8])
        body = data[pos + 8:pos + 8 + length]
        pos += 12 + length
        if kind == b'IHDR':
            width, height, depth, color, _, _, interlace = struct.unpack('>IIBBBBB', body)
        elif kind == b'PLTE':
            palette = body
        elif kind == b'tRNS':
            trns = body
        elif kind == b'IDAT':
            idat += body
        elif kind == b'IEND':
            break
    if interlace:
        raise ValueError('Interlaced PNG files are not supported')
    channels = {0: 1, 2: 3, 3: 1, 4: 2, 6: 4}[color]
    bits = depth * channels
    bpp = max(1, bits // 8)
    stride = (width * bits + 7) // 8
    raw = _unfilter(zlib.decompress(bytes(idat)), width, height, bpp, stride)
    image = Image(width, height)
    px = image.px
    for y in range(height):
        row = raw[y * stride:(y + 1) * stride]
        for x in range(width):
            if depth < 8:
                per = 8 // depth
                value = (row[x // per] >> (8 - depth * (x % per + 1))) & ((1 << depth) - 1)
                samples = (value,)
            else:
                step = depth // 8
                base = x * channels * step
                samples = tuple(row[base + c * step] for c in range(channels))
            if color == 3:
                index = samples[0]
                r, g, b = palette[index * 3:index * 3 + 3]
                a = trns[index] if trns and index < len(trns) else 255
            elif color in (0, 4):
                value = samples[0]
                if depth < 8:
                    value = value * 255 // ((1 << depth) - 1)
                r = g = b = value
                a = samples[1] if color == 4 else 255
            else:
                r, g, b = samples[:3]
                a = samples[3] if color == 6 else 255
            i = (y * width + x) * 4
            px[i], px[i + 1], px[i + 2], px[i + 3] = r, g, b, a
    image.has_alpha = color in (4, 6) or (color == 3 and trns is not None)
    return image


def write_png(path, image):
    rows = b''.join(b'\0' + bytes(image.px[y * image.width * 4:(y + 1) * image.width * 4])
                    for y in range(image.height))

    def chunk(kind, body):
        return struct.pack('>I', len(body)) + kind + body + struct.pack('>I', zlib.crc32(kind + body) & 0xffffffff)
    data = (b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('>IIBBBBB', image.width, image.height, 8, 6, 0, 0, 0))
            + chunk(b'IDAT', zlib.compress(rows, 9)) + chunk(b'IEND', b''))
    Path(path).write_bytes(data)

# ---------------------------------------------------------------- content


class Content:
    """Files of one edition's content, from a folder or from a .uqm zip."""

    def __init__(self, edition, path):
        self.edition = edition
        path = Path(path)
        self.zip = None
        if edition == 'original':
            self.root = path / 'base'
        else:
            packages = [path, path / 'addons' / 'mm-0.8.5-hd-content.uqm', path / 'mm-0.8.5-hd-content.uqm']
            folders = [path / 'mm-hd', path / 'addons' / 'mm-hd']
            for package in packages:
                if package.is_file() and zipfile.is_zipfile(package):
                    self.zip = zipfile.ZipFile(package)
                    self.root = 'mm-hd/'
                    break
            else:
                for folder in folders:
                    if (folder / 'fonts').is_dir():
                        self.root = folder
                        break
                else:
                    raise SystemExit('HD content not found under ' + str(path))

    def read(self, name):
        if self.zip:
            return self.zip.read(self.root + name)
        return (self.root / name).read_bytes()

    def list(self, folder):
        if self.zip:
            prefix = self.root + folder.rstrip('/') + '/'
            return sorted(n[len(prefix):] for n in self.zip.namelist() if n.startswith(prefix) and '/' not in n[len(prefix):] and n != prefix)
        return sorted(p.name for p in (self.root / folder).iterdir())

    def image(self, name):
        return read_png(self.read(name))

# ---------------------------------------------------------------- fonts


def glyph_mask(image):
    """The engine's font mask: alpha when the file has alpha, else intensity."""
    mask = Image(image.width, image.height)
    for y in range(image.height):
        for x in range(image.width):
            r, g, b, a = image.get(x, y)
            if not getattr(image, 'has_alpha', True):
                a = (r * 2 + g * 4 + b) // 7
            mask.put(x, y, (255, 255, 255, a))
    return mask


def load_font(content, folder, scale, only=None):
    """Glyphs plus the engine's text metrics (font.c and gfxload.c)."""
    glyphs, kern, meta = {}, {}, None
    for name in content.list(folder):
        if name == 'kerndat.fnt':
            lines = content.read(folder + '/' + name).decode('latin-1').splitlines()
            head = lines[0].split()
            meta = {'leading': int(head[1]), 'charSpace': int(head[2]), 'kernAmount': int(head[3]), 'vertAlign': int(head[4])}
            for line in lines[1:]:
                parts = line.split()
                if len(parts) == 3:
                    left, right = min(3, max(0, int(parts[1]))), min(3, max(0, int(parts[2])))
                    kern[int(parts[0], 16)] = (left << 2) | right
            continue
        if not name.endswith('.png'):
            continue
        try:
            code = int(name.split('.')[0], 16)
        except ValueError:
            continue
        if only is not None and chr(code) not in only:
            continue
        image = content.image(folder + '/' + name)
        if image.width and image.height:
            glyphs[code] = glyph_mask(image)
    if not glyphs:
        raise SystemExit('No glyphs in ' + folder)
    # Some fonts repeat one "N/A" picture for characters they do not have.
    seen = {}
    for code, glyph in glyphs.items():
        seen.setdefault(bytes(glyph.px), []).append(code)
    for codes in seen.values():
        if len(codes) > 2:
            for code in codes:
                del glyphs[code]
    height = max(g.height for g in glyphs.values())
    if scale == 1:
        # Original: advance = width + 1, hot spot tuned by glyph height.
        def hot(h): return h + (-1 if h == 8 else -2 if h == 9 else -3 if h > 9 else 0)
        char_space, kern_amount, kern = 1, 0, {}
    else:
        if meta is None:
            vert = scale if height <= 8 * scale else 2 * scale if height == 9 * scale else 3 * scale
            meta = {'charSpace': scale, 'kernAmount': 0, 'vertAlign': vert}
        def hot(h): return h - meta['vertAlign']
        char_space, kern_amount = meta['charSpace'], meta['kernAmount']
    ascent = max(hot(g.height) for g in glyphs.values())
    return {'glyphs': glyphs, 'hot': {c: hot(g.height) for c, g in glyphs.items()}, 'ascent': ascent,
            'charSpace': char_space, 'kernAmount': kern_amount, 'kern': kern if kern_amount else {}}

# ---------------------------------------------------------------- title lettering


def ink_columns(mask):
    return [x for x in range(mask.width) if any(mask.alpha(x, y) for y in range(mask.height))]


def split_letters(mask):
    """Split a row of lettering at empty columns."""
    cols = set(ink_columns(mask))
    letters, start = [], None
    for x in range(mask.width + 1):
        if x in cols and start is None:
            start = x
        elif x not in cols and start is not None:
            letters.append(mask.crop(start, 0, x, mask.height))
            start = None
    return letters


def letter_gap(mask):
    """The most common empty run between letters."""
    cols = ink_columns(mask)
    runs = [b - a - 1 for a, b in zip(cols, cols[1:]) if b - a > 1]
    return max(set(runs), key=runs.count)


def trim(mask):
    cols = ink_columns(mask)
    return mask.crop(cols[0], 0, cols[-1] + 1, mask.height) if cols else mask


def ink_rows(mask):
    rows = [y for y in range(mask.height) if any(mask.alpha(x, y) for x in range(mask.width))]
    return rows[0], rows[-1]


def make_o(u):
    """O = U with its rounded bottom mirrored to the top."""
    top, bottom = ink_rows(u)
    o = Image(u.width, u.height, bytearray(u.px))
    middle = (top + bottom) // 2
    for y in range(top, middle + 1):
        source = top + bottom - y
        for x in range(u.width):
            o.put(x, y, u.get(x, source))
    return o


def stem(l):
    """The vertical stroke of L: its columns at mid height."""
    top, bottom = ink_rows(l)
    y = (top + bottom) // 2 - (bottom - top) // 4
    cols = [x for x in range(l.width) if l.alpha(x, y) > 127]
    end = cols[0]
    while end + 1 < l.width and l.alpha(end + 1, y) > 0:
        end += 1
    return trim(l.crop(0, 0, end + 1, l.height))


def make_n(width, stroke_image, aa):
    """N: two stems of the lettering and a diagonal of the same weight."""
    height = stroke_image.height
    top, bottom = ink_rows(stroke_image)
    sw = stroke_image.width
    n = Image(width, height)
    for y in range(height):
        for x in range(sw):
            value = stroke_image.get(x, y)
            n.put(x, y, value)
            n.put(width - sw + x, y, value)
    # Diagonal from the inside of the top-left to the inside of the bottom-right.
    thickness = sw * 1.35
    x0, x1 = 0.0, float(width - thickness)
    samples = 4 if aa else 1
    for y in range(top, bottom + 1):
        for x in range(width):
            hits = 0
            for sy in range(samples):
                for sx in range(samples):
                    px = x + (sx + 0.5) / samples
                    py = y + (sy + 0.5) / samples
                    t = (py - top) / (bottom + 1 - top)
                    left = x0 + (x1 - x0) * t
                    if left <= px < left + thickness:
                        hits += 1
            a = hits * 255 // (samples * samples)
            if a > n.alpha(x, y):
                n.put(x, y, (255, 255, 255, a))
    return n


def compose(word, letters, gap, space):
    width = sum(letters[c].width if c != ' ' else space for c in word) + gap * (len(word) - 1)
    height = max(l.height for l in letters.values())
    out, x = Image(width, height), 0
    for c in word:
        if c == ' ':
            x += space + gap
            continue
        out.paste(letters[c], x, 0)
        x += letters[c].width + gap
    return out


def colorize(mask, rows):
    """Apply a vertical gradient: rows[y] is the colour of row y."""
    out = Image(mask.width, mask.height)
    for y in range(mask.height):
        r, g, b = rows[min(len(rows) - 1, y)]
        for x in range(mask.width):
            a = mask.alpha(x, y)
            if a:
                out.put(x, y, (r, g, b, a))
    return out


def original_title(content):
    """ONLINE MELEE in the lettering of the Super-Melee title (meleemenu-000)."""
    background = content.image('ui/meleemenu-000.png')
    colors = {(93, 0, 93), (124, 0, 124), (155, 0, 155), (186, 0, 186), (217, 0, 217),
              (248, 0, 248), (240, 56, 248), (240, 112, 248)}
    area = background.crop(40, 0, 262, 24)
    mask, rows = Image(area.width, area.height), {}
    for y in range(area.height):
        for x in range(area.width):
            rgb = area.get(x, y)[:3]
            if rgb in colors:
                mask.put(x, y, (255, 255, 255, 255))
                rows.setdefault(y, {}).setdefault(rgb, 0)
                rows[y][rgb] += 1
    top, bottom = ink_rows(mask)
    mask = mask.crop(0, top, mask.width, bottom + 1)
    gradient = [max(rows[y].items(), key=lambda item: item[1])[0] for y in range(top, bottom + 1)]
    letters = split_letters(mask)
    if len(letters) != 11:
        raise SystemExit('Unexpected Super-Melee title layout: %d letters' % len(letters))
    s, u, p, e, r, dash, m, _, l, _, _ = letters
    gap = letter_gap(mask)
    i = stem(l)
    font = {'S': s, 'U': u, 'P': p, 'E': e, 'R': r, '-': dash, 'M': m, 'L': l, 'I': i,
            'O': make_o(u), 'N': make_n(u.width, i, False)}
    title = compose('ONLINE MELEE', font, gap, 3 * gap + 2)
    return colorize(title, gradient)


def hd_title(content):
    """ONLINE MELEE in melee.title.fon with the engine's title gradient (frame 7)."""
    font = {}
    for name in content.list('fonts/melee.title.fon'):
        if name.endswith('.png'):
            font[chr(int(name.split('.')[0], 16))] = trim(glyph_mask(content.image('fonts/melee.title.fon/' + name)))
    i = stem(font['L'])
    font['I'] = i
    font['O'] = make_o(font['U'])
    font['N'] = make_n(font['U'].width, i, True)
    title = compose('ONLINE MELEE', font, 8, 28)
    gradient_image = content.image('fonts/fontgradient-007.png')
    # The gradient frame's hot spot puts it 4 pixels below the glyph top.
    gradient = [gradient_image.get(0, max(0, min(gradient_image.height - 1, y - 4)))[:3] for y in range(title.height)]
    return colorize(title, gradient)

# ---------------------------------------------------------------- backgrounds


def original_stars(content):
    """The Super-Melee star field without its title and buttons."""
    background = content.image('ui/meleemenu-000.png')
    clean = background.crop(0, 26, 250, 240)
    out = Image(320, 240)
    for y0 in range(0, 240, clean.height):
        for x0 in range(0, 320, clean.width):
            part = clean.crop(0, 0, min(clean.width, 320 - x0), min(clean.height, 240 - y0))
            out.paste(part, x0, y0)
    for i in range(0, len(out.px), 4):
        out.px[i + 3] = 255
    return out

# ---------------------------------------------------------------- atlas


def pack(images):
    """Shelf packing; returns (atlas, {name: [x, y, w, h]})."""
    order = sorted(images, key=lambda name: (-images[name].height, name))
    width = 1024 if any(images[n].width > 512 for n in order) or sum(i.width * i.height for i in images.values()) > 512 * 512 else 512
    width = max(width, max(i.width for i in images.values()) + 2)
    x = y = shelf = 0
    places = {}
    for name in order:
        image = images[name]
        if x + image.width + 1 > width:
            x, y, shelf = 0, y + shelf + 1, 0
        places[name] = [x, y, image.width, image.height]
        x += image.width + 1
        shelf = max(shelf, image.height)
    atlas = Image(width, y + shelf)
    for name, (px, py, _, _) in places.items():
        atlas.paste(images[name], px, py)
    return atlas, places

# ---------------------------------------------------------------- edition


ASCII = ''.join(chr(c) for c in range(32, 127))
LETTERS = ' !ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz'
FONTS = {
    # role: (folder, characters or None for all)
    'original': {'body': ('fonts/micro.fon', None), 'label': ('fonts/starcon.fon', None),
                 'small': ('fonts/tiny.fon', None), 'menu': ('fonts/slides.fon', ASCII)},
    'hd': {'body': ('fonts/micro.fon', None), 'label': ('fonts/starcon.fon', None),
           'small': ('fonts/tiny.fon', None), 'menu': ('fonts/slab.fon', LETTERS)},
}


def build(edition, content_path, output):
    content = Content(edition, content_path)
    scale = 1 if edition == 'original' else 4
    output = Path(output)
    output.mkdir(parents=True, exist_ok=True)
    images, fonts = {}, {}
    for role, (folder, chars) in FONTS[edition].items():
        font = load_font(content, folder, scale, chars)
        glyphs = {}
        for code, glyph in font['glyphs'].items():
            images[role + ':' + str(code)] = glyph
            glyphs[code] = glyph
        fonts[role] = font
    images['title'] = original_title(content) if edition == 'original' else hd_title(content)
    # The BATTLE! picture of the Super-Melee screen, normal and selected.
    battle = ('ui/meleemenu-025.png', 'ui/meleemenu-026.png') if edition == 'original' else \
        ('ui/meleemenu/meleemenu-004.png', 'ui/meleemenu/meleemenu-005.png')
    images['battle'], images['battleHi'] = (content.image(name) for name in battle)
    atlas, places = pack(images)
    write_png(output / 'online-art.png', atlas)
    if edition == 'original':
        write_png(output / 'online-stars.png', original_stars(content))
        (output / 'online-title.png').write_bytes(content.read('ui/title.png'))
    else:
        (output / 'online-stars.png').write_bytes(content.read('ui/meleemenu/meleemenu-000.png'))
        (output / 'online-title.png').write_bytes(content.read('ui/mainmenu/title.png'))
    meta = {
        'version': 1, 'edition': edition, 'scale': scale,
        'images': {'atlas': 'online-art.png', 'stars': 'online-stars.png', 'title': 'online-title.png'},
        'sprites': {name: places[name] for name in ('title', 'battle', 'battleHi')},
        'fonts': {},
        'source': ('Original UQM content (sc2/content): fonts, Super-Melee title lettering and star field, title picture.'
                   if edition == 'original' else
                   'UQM MegaMod HD content (mm-0.8.5-hd-content.uqm): fonts, title lettering and gradient, star field, title picture.'),
    }
    for role, font in fonts.items():
        glyphs = {}
        for code in sorted(font['glyphs']):
            x, y, w, h = places[role + ':' + str(code)]
            glyphs[str(code)] = [x, y, w, h, font['ascent'] - font['hot'][code]]
        meta['fonts'][role] = {'ascent': font['ascent'],
                               'height': max(g[4] + g[3] for g in glyphs.values()),
                               'charSpace': font['charSpace'], 'kernAmount': font['kernAmount'],
                               'kern': {str(c): v for c, v in sorted(font['kern'].items())},
                               'glyphs': glyphs}
    (output / 'online-art.json').write_text(json.dumps(meta, separators=(',', ':')) + '\n')
    print('Built %s online art: %s (%dx%d atlas)' % (edition, output / 'online-art.json', atlas.width, atlas.height))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__.split('\n\n')[0])
    parser.add_argument('--edition', choices=['original', 'hd'], required=True)
    parser.add_argument('--content', required=True, help='sc2/content, or the HD content folder or package')
    parser.add_argument('--output', required=True)
    args = parser.parse_args()
    build(args.edition, args.content, args.output)
