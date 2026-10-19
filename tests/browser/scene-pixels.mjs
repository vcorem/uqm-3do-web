// Self-contained because Playwright serializes this function into the page.
// Analyze screenshot pixels: WebGL drawing buffers may have already cleared.
export async function measureSceneImage(encoded) {
  const bytes = Uint8Array.from(atob(encoded), c => c.charCodeAt(0));
  const bitmap = await createImageBitmap(new Blob([bytes], { type: 'image/png' }));
  const surface = document.createElement('canvas');
  surface.width = bitmap.width; surface.height = bitmap.height;
  const ctx = surface.getContext('2d'); ctx.drawImage(bitmap, 0, 0); bitmap.close();
  const width = surface.width, height = surface.height;
  const pixels = ctx.getImageData(0, 0, width, height).data;
  const sceneWidth = Math.floor(width * 0.75);
  let lit = 0, hash = 2166136261;
  const colors = new Set();
  for (let y = 0; y < height; y++) for (let x = 0; x < sceneWidth; x++) {
    const i = (y * width + x) * 4;
    if (Math.max(pixels[i], pixels[i + 1], pixels[i + 2]) > 32) lit++;
    colors.add((pixels[i] >> 4) * 256 + (pixels[i + 1] >> 4) * 16 + (pixels[i + 2] >> 4));
    hash = Math.imul(hash ^ pixels[i], 16777619) >>> 0;
    hash = Math.imul(hash ^ pixels[i + 1], 16777619) >>> 0;
    hash = Math.imul(hash ^ pixels[i + 2], 16777619) >>> 0;
  }
  // Interior Hyperspace playfield only: omit title, frame and right HUD.
  // The central ship/nearby system can stay visible even when the ambient
  // display list is lost. Exclude that entire region from this measurement.
  const x0 = Math.ceil(width * 0.09), x1 = Math.floor(width * 0.71);
  const y0 = Math.ceil(height * 0.14), y1 = Math.floor(height * 0.90);
  const excluded = (x, y) => x >= width * 0.24 && x <= width * 0.55 &&
    y >= height * 0.37 && y <= height * 0.70;
  const histogram = new Map();
  let sampled = 0;
  for (let y = y0; y < y1; y++) for (let x = x0; x < x1; x++) {
    if (excluded(x, y)) continue;
    const i = (y * width + x) * 4;
    const rgb = pixels[i] * 65536 + pixels[i + 1] * 256 + pixels[i + 2];
    histogram.set(rgb, (histogram.get(rgb) || 0) + 1); sampled++;
  }
  let background = 0, backgroundCount = 0;
  for (const [rgb, count] of histogram) if (count > backgroundCount) { background = rgb; backgroundCount = count; }
  const base = [background >>> 16, (background >>> 8) & 255, background & 255];
  const cells = new Set();
  let ambientPixels = 0;
  for (let y = y0; y < y1; y++) for (let x = x0; x < x1; x++) {
    if (excluded(x, y)) continue;
    const i = (y * width + x) * 4;
    // Count detail above the dominant background, not the uniform red field.
    if (Math.max(pixels[i] - base[0], pixels[i + 1] - base[1], pixels[i + 2] - base[2]) < 16) continue;
    ambientPixels++;
    const column = Math.min(3, Math.floor((x - x0) * 4 / (x1 - x0)));
    const row = Math.min(3, Math.floor((y - y0) * 4 / (y1 - y0)));
    cells.add(row * 4 + column);
  }
  return { width, height, litFraction: lit / (sceneWidth * height), colors: colors.size, sceneHash: hash,
    ambientStars: { pixels: ambientPixels, fraction: ambientPixels / sampled, cells: cells.size,
      background: base, backgroundFraction: backgroundCount / sampled, sampled } };
}

export function hasAmbientStars(scene) {
  // One moving system sprite or the ship cannot satisfy spatial distribution.
  // The prior C engine has detail in 11+ cells; permit motion and scaling.
  const { fraction, cells, background, backgroundFraction } = scene.ambientStars;
  return background[0] > background[1] + 20 && background[0] > background[2] + 20 &&
    backgroundFraction >= 0.8 && fraction >= 0.0003 && cells >= 5;
}
