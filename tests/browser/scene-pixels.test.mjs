// Frozen real-engine regression: a visible ship/HUD must not mask lost stars.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { chromium } from 'playwright';
import { measureSceneImage, hasAmbientStars } from './scene-pixels.mjs';
const browser = await chromium.launch({ headless: true,
  ...(process.env.CHROME_BIN ? { executablePath: process.env.CHROME_BIN } : {}) });
try {
  const page = await browser.newPage();
  for (const [file, expected] of [['hyperspace-stars.png', true], ['hyperspace-no-stars.png', false]]) {
    const bytes = await readFile(new URL('./fixtures/' + file, import.meta.url));
    const scene = await page.evaluate(measureSceneImage, bytes.toString('base64'));
    // Both historically passed the weak nonblank check; only one has stars.
    assert.ok(scene.litFraction > 0.01 && scene.colors > 24);
    assert.equal(hasAmbientStars(scene), expected, `${file}: ${JSON.stringify(scene.ambientStars)}`);
    console.log(`${file}: ${scene.ambientStars.pixels} ambient pixels in ${scene.ambientStars.cells} cells; accepted=${expected}`);
  }
} finally { await browser.close(); }
