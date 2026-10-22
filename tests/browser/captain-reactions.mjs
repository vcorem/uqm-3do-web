// Real captain portraits driven only by the existing Melee keyboard controls.
// Diagnostic exports are read-only; fleet selection and battle state stay native.
import assert from 'node:assert/strict';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';

const inventory = JSON.parse(await readFile(new URL('../../hd/remastered/art/captains/native-inventory.json', import.meta.url), 'utf8'));
const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));
const keys = ['ArrowLeft', 'ArrowRight', 'ArrowUp', 'ControlRight', 'ShiftRight'];

export async function readCaptain(page, race) {
  return page.evaluate(race => ({
    images: Module._UQM_GetMeleeRemasteredCaptainImageCount(race),
    total: Module._UQM_GetMeleeRemasteredCaptainDrawCount(race, -1),
    seen: Module._UQM_GetMeleeRemasteredCaptainSeenMask(race) >>> 0,
    rows: Array.from({ length: 20 }, (_, row) => Module._UQM_GetMeleeRemasteredCaptainDrawCount(race, row)),
    enabled: Module._UQM_GetMeleeRemastered(),
  }), race);
}

function changedPixels(first, second) {
  assert.equal(first.length, second.length, 'Captain capture dimensions changed');
  let changed = 0;
  for (let i = 0; i < first.length; i += 4) {
    if (Math.max(...[0, 1, 2].map(c => Math.abs(first[i + c] - second[i + c]))) > 8) changed++;
  }
  return changed;
}

async function capture(page, filename) {
  const native = await page.locator('#canvas').evaluate(canvas => ({ width: canvas.width, height: canvas.height }));
  assert.deepEqual(native, { width: 1280, height: 960 }, 'Captain crop requires the native 3DO HD viewport');
  const canvas = await page.locator('#canvas').boundingBox();
  assert.ok(canvas);
  // status.h: the guest is the top player, captain origin (976,340), 220×120.
  // Use a browser screenshot of the real native canvas, not a re-rendered asset.
  const png = await page.screenshot({ clip: {
    x: canvas.x + canvas.width * 976 / 1280,
    y: canvas.y + canvas.height * 340 / 960,
    width: canvas.width * 220 / 1280,
    height: canvas.height * 120 / 960,
  } });
  await writeFile(filename, png);
  return page.evaluate(async encoded => {
    const bytes = Uint8Array.from(atob(encoded), char => char.charCodeAt(0));
    const bitmap = await createImageBitmap(new Blob([bytes], { type: 'image/png' }));
    const canvas = document.createElement('canvas');
    canvas.width = bitmap.width; canvas.height = bitmap.height;
    const context = canvas.getContext('2d');
    context.drawImage(bitmap, 0, 0); bitmap.close();
    return Array.from(context.getImageData(0, 0, canvas.width, canvas.height).data);
  }, png.toString('base64'));
}

async function expectRows(page, race, before, rows, label, timeout = 6000) {
  try {
    await page.waitForFunction(({ race, before, rows }) => rows.every(row =>
      Module._UQM_GetMeleeRemasteredCaptainDrawCount(race, row) > before[row]),
    { race, before: before.rows, rows }, { polling: 25, timeout });
  } catch (error) {
    throw new Error(`${label}: expected native captain rows ${rows}; before ${JSON.stringify(before)}; after ${JSON.stringify(await readCaptain(page, race))}`, { cause: error });
  }
  const after = await readCaptain(page, race);
  for (const row of rows) assert.ok(after.rows[row] > before.rows[row], `${label}: captain row ${row} was not drawn`);
  return after;
}

export async function verifyCaptainReactions({ pages, shipIndex, output, name, entry,
  setPresentation, waitState, resumeBattle, checkSynchronization }) {
  const ship = inventory.ships.find(ship => ship.melee_index === shipIndex);
  assert.ok(ship, `Unknown captain race ${shipIndex}`);
  const [display, control] = pages;
  const folder = path.join(output, `${name}-captain`);
  await mkdir(folder, { recursive: true });
  const report = entry.captain = { race: ship.race, meleeIndex: shipIndex,
    nativeRows: ship.frame_count, phases: [], passed: false };
  const release = () => Promise.all(keys.map(key => control.keyboard.up(key)));
  const snapshot = label => capture(display, path.join(folder, `${label}.png`));
  try {
    await resumeBattle('before captain reactions');
    await release();
    await sleep(300);
    const initial = await readCaptain(display, shipIndex);
    assert.ok(initial.images >= ship.frame_count && initial.images % ship.frame_count === 0,
      'Every ANI row in each live captain instance must have an accepted replacement: ' + JSON.stringify(initial));
    assert.ok(initial.rows[0] > 0, 'The native captain background was never rendered');
    assert.equal((await readCaptain(control, shipIndex)).total, 0, 'Classic peer must not render replacement captains');
    report.initial = initial;

    // Captain windows use partial stamps. An idle toggle must repaint the whole
    // current pose immediately, rather than waiting for another input event.
    const remasteredBefore = await snapshot('idle-remastered');
    await setPresentation(display, 'classic');
    await sleep(350);
    const disabled = await readCaptain(display, shipIndex);
    const classic = await snapshot('idle-classic');
    await sleep(200);
    assert.equal((await readCaptain(display, shipIndex)).total, disabled.total,
      'Classic mode must issue no captain replacement draws');
    assert.ok(changedPixels(remasteredBefore, classic) >= 100, 'Idle style toggle left the old captain portrait on screen');
    await setPresentation(display, 'remastered');
    await expectRows(display, shipIndex, disabled, [0], 'idle style restore');
    await sleep(200);
    const baseline = await snapshot('idle-restored');
    assert.ok(changedPixels(classic, baseline) >= 100, 'Returning to Remastered did not repaint the captain');
    report.toggle = { changedToClassic: changedPixels(remasteredBefore, classic),
      changedToRemastered: changedPixels(classic, baseline), disabled };
    await Promise.all(pages.map(page => page.locator('#canvas').focus()));

    const phase = async (name, heldKeys, heldRows, restRows, releaseTimeout = 6000) => {
      await resumeBattle(`captain ${name}`);
      const before = await readCaptain(display, shipIndex);
      await Promise.all(heldKeys.map(key => control.keyboard.down(key)));
      const held = await expectRows(display, shipIndex, before, heldRows, name);
      await sleep(220);
      const pixels = await snapshot(`${name}-held`);
      let activeToggle;
      if (name === 'combined') {
        // Controls are held by the other peer, so using this display's actual
        // settings UI does not release the active native animation inputs.
        await setPresentation(display, 'classic');
        await sleep(250);
        const disabled = await readCaptain(display, shipIndex);
        const classicHeld = await snapshot(`${name}-held-classic`);
        assert.ok(changedPixels(pixels, classicHeld) >= 100,
          'The active portrait did not change when Classic was selected');
        await setPresentation(display, 'remastered');
        await expectRows(display, shipIndex, disabled, [0, ...heldRows], `${name} active style restore`);
        await sleep(220);
        const restoredHeld = await snapshot(`${name}-held-restored`);
        const changedPose = changedPixels(pixels, restoredHeld);
        assert.ok(changedPose <= 2,
          `${ship.race}: style round-trip changed ${changedPose} pixels of the held captain pose`);
        activeToggle = { disabled, changedToClassic: changedPixels(pixels, classicHeld),
          changedHeldPosePixels: changedPose };
      }
      const beforeRelease = await readCaptain(display, shipIndex);
      await release();
      const restored = await expectRows(display, shipIndex, beforeRelease, restRows, `${name} release`, releaseTimeout);
      await sleep(220);
      const released = await snapshot(`${name}-released`);
      const changed = changedPixels(baseline, pixels);
      const residue = changedPixels(baseline, released);
      report.phases.push({ name, heldKeys, heldRows, restRows, before, held, restored,
        changedPixels: changed, residualPixels: residue, ...(activeToggle ? { activeToggle } : {}) });
      assert.ok(changed >= 4, `${ship.race} ${name}: native action counters changed but the captain picture did not`);
      assert.ok(residue <= 2, `${ship.race} ${name}: released action left ${residue} changed captain pixels`);
    };
    await phase('left', ['ArrowLeft'], [4, 5], [3]);
    await phase('right', ['ArrowRight'], [2, 1], [3]);
    await phase('thrust', ['ArrowUp'], [7, 8], [6]);
    await phase('weapon', ['ControlRight'], [10, 11], [9]);
    await phase('combined', ['ArrowLeft', 'ArrowUp', 'ControlRight'], [5, 8, 11], [3, 6, 9]);

    if (shipIndex === 13) {
      // The Scout latches its lever instead of returning to neutral. Three
      // ordinary presses must draw the extended lever sequence and death frame.
      const before = await readCaptain(display, shipIndex);
      for (let press = 1; press <= 3; press++) {
        await control.keyboard.press('ShiftRight', { delay: 300 });
        await sleep(200);
        await snapshot(`glory-device-${press}`);
        report.phases.push({ name: `glory-device-${press}`, state: await readCaptain(display, shipIndex) });
      }
      await waitState('selectShip');
      report.detonated = await readCaptain(display, shipIndex);
      assert.ok(report.detonated.rows[19] > before.rows[19], 'The native Shofixti final captain frame was not rendered');
      await resumeBattle('after captain glory-device test');
    } else if (shipIndex === 0) {
      // The original Blazer mode forces SPECIAL and suppresses thrust/weapon
      // until its fuel runs out. Releasing the key does not cancel this mode.
      // The combined weapon phase may empty the battery. Allow native energy
      // regeneration to provide the two units required for transformation.
      await sleep(3000);
      await phase('special-latched-blazer', ['ShiftRight'], [13, 14], [12], 20000);
    } else if (shipIndex === 4) {
      // druuge_preprocess clears SPECIAL during its 30-tick cooldown. The
      // native animator therefore draws the one-tick transition and returns
      // to rest; rows12/13 alias the same PNG, and row14 is not reached.
      // A naturally reselected Mauler starts with a full battery and cannot
      // sacrifice crew. Fire normally first so the furnace can accept input.
      await resumeBattle('prepare native Druuge furnace');
      await control.keyboard.press('ControlRight', { delay: 1200 });
      await sleep(220);
      const before = await readCaptain(display, shipIndex);
      await control.keyboard.down('ShiftRight');
      const pulse = await expectRows(display, shipIndex, before, [13, 12], 'Druuge furnace pulse');
      await release();
      await sleep(220);
      const released = await snapshot('special-native-pulse');
      const residue = changedPixels(baseline, released);
      assert.ok(residue <= 2, `Druuge furnace pulse changed its native neutral composition (${residue} pixels)`);
      report.phases.push({ name: 'special-native-pulse', before, pulse, residualPixels: residue,
        nativeRule: 'druuge_preprocess allows a one-tick SPECIAL pulse; rows13 and12 are the same neutral PNG' });
    } else {
      await phase('special', ['ShiftRight'], [13, 14], [12]);
      // Cover concurrent special overlays on ships whose native abilities do
      // not suppress these other inputs (Arilou teleport intentionally does).
      if ([3, 5, 16, 20, 23].includes(shipIndex)) {
        await phase('combined-special', ['ArrowRight', 'ArrowUp', 'ControlRight', 'ShiftRight'],
          [1, 8, 11, 14], [3, 6, 9, 12]);
      }
    }

    const final = await readCaptain(display, shipIndex);
    const unique = [...new Set(ship.frames.map(frame => frame.filename))];
    report.uniquePngCoverage = unique.map(filename => {
      const rows = ship.frames.filter(frame => frame.filename === filename).map(frame => frame.index);
      // Preserve the difference between packaged coverage and actual native
      // reachability. Never force status flags just to visit a dormant row.
      const staticOnly = shipIndex === 4 && rows.length === 1 && rows[0] === 14
        ? 'Native Druuge one-tick SPECIAL/cooldown cannot advance its captain offset to2; asset is verified by captain-assets.py'
        : shipIndex === 13 && rows.length === 1 && rows[0] === 12
          ? 'Native Shofixti safe lever is already in the background; shofixti_preprocess latches forward on release and never returns to offset0; asset is verified by captain-assets.py'
          : null;
      return { filename, rows, seenRows: rows.filter(row => final.rows[row] > 0),
        ...(staticOnly ? { staticOnly } : {}) };
    });
    for (const frame of report.uniquePngCoverage) {
      assert.ok(frame.staticOnly || frame.seenRows.length > 0,
        `${ship.race}: no native draw used ${frame.filename} (ANI rows ${frame.rows})`);
    }
    assert.equal((await readCaptain(control, shipIndex)).total, 0, 'Classic peer emitted a captain replacement draw');
    report.final = final;
    report.synchronization = await checkSynchronization(30);
    await display.locator('#canvas').screenshot({ path: path.join(folder, 'native-battle-final.png') });
    report.passed = true;
  } finally {
    await release().catch(() => {});
    await writeFile(path.join(folder, 'results.json'), JSON.stringify(report, null, 2) + '\n');
  }
}
