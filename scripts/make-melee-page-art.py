#!/usr/bin/env python3
"""Make the page art of Melee Remastered from its own painted art (hd/remastered/art).

- web/editions/melee-card.jpg: the picker card, an Earthling Cruiser and an
  Ur-Quan Dreadnought over the arena nebula, with the explosion atlas.
- hd/remastered/art/nebula-start.jpg: the nebula at the game frame size, the
  background of the page's start screen and lobby (stage-melee-remastered.py).

It needs Pillow and NumPy; run it again after an art change.
"""
import argparse
import math
from pathlib import Path

import numpy as np
from PIL import Image, ImageFilter

ROOT = Path(__file__).resolve().parents[1]
ART = ROOT / 'hd' / 'remastered' / 'art'
W, H = 1200, 720


def cell(sheet, slot, cells=2):
    """One ship or explosion frame of a square atlas, cropped to its visible pixels."""
    image = Image.open(ART / sheet).convert('RGBA')
    size = image.width // cells
    x, y = slot % cells, slot // cells
    frame = image.crop((x * size, y * size, (x + 1) * size, (y + 1) * size))
    return frame.crop(frame.getchannel('A').point(lambda a: 255 if a > 24 else 0).getbbox())


def heading(dx, dy):
    """The counter-clockwise rotation that turns a sprite drawn nose-up toward (dx, dy) on screen."""
    return math.degrees(math.atan2(-dy, dx)) - 90


def turned(sprite, height, angle):
    scale = height / sprite.height
    sprite = sprite.resize((round(sprite.width * scale), round(height)), Image.LANCZOS)
    return sprite.rotate(angle, resample=Image.BICUBIC, expand=True)


def array(image):
    return np.asarray(image, dtype=np.float32) / 255


def over(base, sprite, center, shade=1.0):
    """Alpha-composites an RGBA sprite at a center point; shade < 1 darkens a distant ship."""
    x0, y0 = round(center[0] - sprite.width / 2), round(center[1] - sprite.height / 2)
    layer = Image.new('RGBA', (W, H))
    layer.paste(sprite, (x0, y0))
    rgba = array(layer)
    alpha = rgba[..., 3:]
    return base * (1 - alpha) + rgba[..., :3] * shade * alpha


def screen(base, light):
    return 1 - (1 - base) * (1 - np.clip(light, 0, 1))


def glow(points, blur):
    """A light layer: (x, y, radius, (r, g, b), strength) discs, blurred into soft light."""
    layer = np.zeros((H, W, 3), np.float32)
    yy, xx = np.mgrid[0:H, 0:W]
    for x, y, radius, color, strength in points:
        disc = np.clip(1 - np.hypot(xx - x, yy - y) / radius, 0, 1) ** 2 * strength
        layer += disc[..., None] * np.array(color, np.float32)[None, None, :] / 255
    image = Image.fromarray(np.uint8(np.clip(layer, 0, 1) * 255))
    return array(image.filter(ImageFilter.GaussianBlur(blur)))


def bolt(start, end, color, width, blur):
    """A glowing shot: a bright core line and a wide soft halo."""
    layer = Image.new('RGB', (W, H))
    from PIL import ImageDraw
    draw = ImageDraw.Draw(layer)
    draw.line([start, end], fill=tuple(color), width=width * 3)
    halo = array(layer.filter(ImageFilter.GaussianBlur(blur)))
    core = Image.new('RGB', (W, H))
    ImageDraw.Draw(core).line([start, end], fill=(255, 255, 240), width=max(1, width // 2))
    return np.clip(halo * 1.6 + array(core.filter(ImageFilter.GaussianBlur(1))), 0, 1)


def explosion(frame, size, center):
    sprite = cell('explosion.png', frame, cells=3)
    sprite = sprite.resize((size, round(sprite.height * size / sprite.width)), Image.LANCZOS)
    layer = Image.new('RGBA', (W, H))
    layer.paste(sprite, (round(center[0] - sprite.width / 2), round(center[1] - sprite.height / 2)))
    rgba = array(layer)
    return rgba[..., :3] * rgba[..., 3:]


def start_background(output):
    nebula = Image.open(ART / 'nebula.png').convert('RGB').resize((1280, 960), Image.LANCZOS)
    nebula.save(output, quality=84, optimize=True, progressive=True)
    print('Wrote', output, nebula.size)


def make(output):
    nebula = Image.open(ART / 'nebula.png').convert('RGB')
    scale = 1.18 * W / nebula.width
    nebula = nebula.resize((round(nebula.width * scale), round(nebula.height * scale)), Image.LANCZOS)
    left, top = round((nebula.width - W) * 0.30), round((nebula.height - H) * 0.62)
    base = array(nebula.crop((left, top, left + W, top + H)))
    base = np.clip(base * 1.25, 0, 1)

    cruiser_at, dreadnought_at = (345, 478), (862, 285)
    dx, dy = dreadnought_at[0] - cruiser_at[0], dreadnought_at[1] - cruiser_at[1]
    toward = math.hypot(dx, dy)
    ux, uy = dx / toward, dy / toward

    # Depth: two far ships, dark and small, behind the duel.
    base = over(base, turned(cell('ships-1.png', 2), 92, heading(-1, 0.25)), (128, 150), shade=0.6)
    base = over(base, turned(cell('ships-1.png', 3), 70, heading(1, -0.4)), (1080, 610), shade=0.42)

    # Engine light under the ships, then the ships themselves.
    tail = (cruiser_at[0] - ux * 132, cruiser_at[1] - uy * 132)
    base = screen(base, glow([
        (tail[0] - ux * 30, tail[1] - uy * 30, 80, (90, 170, 255), 1.0),
        (dreadnought_at[0] + ux * 205, dreadnought_at[1] + uy * 205, 110, (255, 140, 40), 0.9),
    ], 18))
    base = screen(base, bolt(tail, (tail[0] - ux * 56, tail[1] - uy * 56), (110, 190, 255), 8, 8))
    dreadnought = turned(cell('ships-1.png', 1), 440, heading(-dx, -dy))
    cruiser = turned(cell('ships-1.png', 0), 300, heading(dx, dy))
    base = over(base, dreadnought, dreadnought_at)
    base = over(base, cruiser, cruiser_at)

    # The fight: the Dreadnought's fusion blasts and the Cruiser's nuclear missile.
    for t in (0.36, 0.55):
        x, y = cruiser_at[0] + dx * t, cruiser_at[1] + dy * t
        base = screen(base, bolt((x + ux * 34, y + uy * 34), (x - ux * 34, y - uy * 34), (90, 255, 120), 6, 7))
    head = (cruiser_at[0] + dx * 0.47 + uy * 58, cruiser_at[1] + dy * 0.47 - ux * 58)
    base = screen(base, bolt((head[0] - ux * 64, head[1] - uy * 64), head, (255, 150, 70), 2, 5) * 0.7)
    base = screen(base, glow([(head[0], head[1], 16, (255, 240, 200), 1.0)], 3))
    # A hit on the Dreadnought's flank.
    hit = (dreadnought_at[0] - 74, dreadnought_at[1] + 116)
    base = screen(base, explosion(2, 180, hit))
    base = screen(base, glow([(hit[0], hit[1], 44, (255, 245, 220), 0.8)], 10))

    # Grade: a soft vignette keeps the card's text side calm.
    yy, xx = np.mgrid[0:H, 0:W]
    distance = np.hypot((xx - W * 0.55) / (W * 0.62), (yy - H * 0.48) / (H * 0.66))
    base *= np.clip(1.08 - distance ** 2 * 0.55, 0.35, 1)[..., None]
    image = Image.fromarray(np.uint8(np.clip(base, 0, 1) * 255))
    image.save(output, quality=82, optimize=True, progressive=True)
    print('Wrote', output, image.size)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, default=ROOT / 'web' / 'editions' / 'melee-card.jpg')
    parser.add_argument('--start', type=Path, default=ART / 'nebula-start.jpg')
    args = parser.parse_args()
    make(args.output)
    start_background(args.start)
