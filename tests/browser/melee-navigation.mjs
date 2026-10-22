// Read-only navigation instrumentation plus real keyboard actions. No native
// flags, activity state, input arrays, or WebAssembly memory are modified.
import assert from 'node:assert/strict';
const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));

export async function recordMeleeScreens(page) {
  await page.evaluate(() => {
    if (window.meleeTestScreens) return;
    window.meleeTestScreens = [];
    const descriptor = Object.getOwnPropertyDescriptor(Module, 'gameScreen');
    let value = Module.gameScreen;
    if (value !== undefined) window.meleeTestScreens.push(value);
    // Preserve the production setter: it delivers every transition to the UI.
    Object.defineProperty(Module, 'gameScreen', { configurable: true, enumerable: true,
      get() { return descriptor?.get ? descriptor.get.call(Module) : value; },
      set(next) {
        window.meleeTestScreens.push(next);
        if (descriptor?.set) descriptor.set.call(Module, next); else value = next;
      },
    });
  });
}

export async function assertMeleeScreens(page) {
  const screens = await page.evaluate(() => window.meleeTestScreens);
  assert.ok(Array.isArray(screens), 'Screen tracing must begin before the first native launch');
  assert.ok(screens.every(screen => ['transition', 'melee', 'meleeMenu'].includes(screen)),
    'A title, intro, campaign or old main-menu route was visited: ' + JSON.stringify(screens));
  return screens;
}

export async function assertMeleeChoices(page, native = false) {
  await page.waitForFunction(native => Module.meleeOnly === true && online.state.screen === 'start' &&
    (!native || Module.gameScreen === 'meleeMenu'), native, { timeout: 30000 });
  await page.getByRole('button', { name: 'Melee', exact: true }).waitFor();
  await page.getByRole('button', { name: 'Network Melee', exact: true }).waitFor();
  const state = await page.evaluate(() => ({ items: online.state.items,
    labels: [...document.querySelectorAll('.uqm-online-hit')].filter(item => !item.hidden)
      .map(item => item.getAttribute('aria-label') || item.textContent.trim()),
    cornerHidden: online.corner.hidden, overlayHidden: online.overlay.hidden,
    session: !!online.state.session, started,
  }));
  assert.deepEqual(state.items, ['play', 'online']);
  assert.deepEqual(state.labels, ['Melee', 'Network Melee']);
  assert.equal(state.cornerHidden, true);
  assert.equal(state.overlayHidden, false);
  assert.equal(state.session, false);
  if (native) assert.equal(state.started, true, 'Returning leaves the native engine alive');
  await assertMeleeScreens(page);
  return state;
}

async function key(page, name) {
  await page.keyboard.press(name, { delay: 250 });
  await sleep(750);
}

export async function launchLocalMelee(page) {
  await assertMeleeChoices(page);
  await page.getByRole('button', { name: 'Melee', exact: true }).click();
  await page.waitForFunction(() => Module.gameScreen === 'melee' && online.state.screen === 'hidden',
    null, { timeout: 90000 });
  await page.locator('#canvas').focus();
  // Allow the native fleet setup fade to finish before it polls the next key.
  await sleep(1500);
  await assertMeleeScreens(page);
}

export async function leaveLocalFleet(page) {
  // The native BATTLE option starts selected. Four Down presses select its
  // actual Quit option (LOAD, SAVE, CONTROL, QUIT), then Enter activates it.
  await page.locator('#canvas').focus();
  for (let i = 0; i < 4; i++) await key(page, 'ArrowDown');
  await key(page, 'Enter');
  await assertMeleeChoices(page, true);
}

export async function leaveLocalBattle(page, confirmScreenshot) {
  // F10 is menu.key's quit binding. The real native confirmation defaults to
  // No; Left selects Yes and Enter accepts. This exercises CHECK_ABORT cleanup.
  await page.locator('#canvas').focus();
  await key(page, 'F10');
  if (confirmScreenshot) await page.screenshot({ path: confirmScreenshot });
  await key(page, 'ArrowLeft');
  await key(page, 'Enter');
  // Native combat abort returns to its fleet editor. It does not leave Melee:
  // first prove the fleet was drawn, then activate its real EXIT option.
  const deadline = Date.now() + 30000;
  let fleet;
  do {
    const png = await page.locator('#canvas').screenshot();
    fleet = await page.evaluate(async encoded => {
      const bytes = Uint8Array.from(atob(encoded), c => c.charCodeAt(0));
      const bitmap = await createImageBitmap(new Blob([bytes], { type: 'image/png' }));
      const canvas = document.createElement('canvas'); canvas.width = bitmap.width; canvas.height = bitmap.height;
      const context = canvas.getContext('2d'); context.drawImage(bitmap, 0, 0); bitmap.close();
      const {data,width,height} = context.getImageData(0,0,canvas.width,canvas.height);
      let light=0, magenta=0, count=0;
      for (let y=Math.floor(height*.065); y<height*.15; y+=4)
        for (let x=Math.floor(width*.15); x<width*.8; x+=4) {
          const i=(y*width+x)*4, r=data[i], g=data[i+1], b=data[i+2]; count++;
          if (r>150 && g>165 && b>175 && Math.max(r,g,b)-Math.min(r,g,b)<70) light++;
          if (r>80 && b>80 && g<Math.min(r,b)/2) magenta++;
        }
      return { light:light/count, magenta:magenta/count, screen:Module.gameScreen,
        overlay:online.state.screen, backgrounds:Module._UQM_GetMeleeRemasteredBackgroundCount() };
    }, png.toString('base64'));
    if (fleet.screen === 'melee' && fleet.overlay === 'hidden' && (fleet.light>.04 || fleet.magenta>.04)) break;
    await sleep(300);
  } while (Date.now()<deadline);
  assert.ok(fleet.screen === 'melee' && fleet.overlay === 'hidden' && (fleet.light>.04 || fleet.magenta>.04),
    'Combat abort must return to the actual native fleet screen: '+JSON.stringify(fleet));
  await sleep(500);
  assert.equal(await page.evaluate(() => Module._UQM_GetMeleeRemasteredBackgroundCount()), fleet.backgrounds,
    'Battle rendering must stop before quitting the native fleet screen');
  if (confirmScreenshot) await page.screenshot({path:confirmScreenshot.replace(/\.png$/, '-fleet.png')});
  await leaveLocalFleet(page);
  return fleet;
}

export async function exerciseLocalBattle(page, imagePrefix) {
  const native = () => page.evaluate(() => ({
    backgrounds: Module._UQM_GetMeleeRemasteredBackgroundCount(),
    draws: Module._UQM_GetMeleeRemasteredDrawCount(),
    images: Module._UQM_GetMeleeRemasteredImageCount(),
    screen: Module.gameScreen, connected: !!Module.webNet,
  }));
  const start = await native();
  assert.equal(start.connected, false, 'Local play must detach the closed network client');
  for (let attempt = 0; attempt < 6; attempt++) {
    await key(page, 'Enter');
    // HD's actual packaged uqm.key assigns player2 ESDF with Q as weapon.
    // pickmele.c uses each human's BATTLE_WEAPON, not the menu Enter binding.
    await key(page, 'q');
    await sleep(1000);
    if ((await native()).backgrounds > start.backgrounds) break;
  }
  await page.waitForFunction(before => Module._UQM_GetMeleeRemasteredBackgroundCount() > before,
    start.backgrounds, { timeout: 30000 });
  const capture = async file => {
    const png = await page.locator('#canvas').screenshot({ path: file });
    return page.evaluate(async base64 => {
      const bytes = Uint8Array.from(atob(base64), c => c.charCodeAt(0));
      const bitmap = await createImageBitmap(new Blob([bytes], { type: 'image/png' }));
      const canvas = document.createElement('canvas'); canvas.width = bitmap.width; canvas.height = bitmap.height;
      const context = canvas.getContext('2d'); context.drawImage(bitmap, 0, 0); bitmap.close();
      const { data, width, height } = context.getImageData(0, 0, canvas.width, canvas.height);
      let dark = 0; const samples = [];
      // Actual battle playfield only: exclude captain/HUD and ship-picker zones.
      for (let y = Math.floor(height * .08); y < height * .92; y += 4)
        for (let x = Math.floor(width * .03); x < width * .72; x += 4) {
          const i = (y * width + x) * 4;
          samples.push(data[i], data[i+1], data[i+2]);
          if (Math.max(data[i], data[i+1], data[i+2]) < 32) dark++;
        }
      return { samples, dark: dark / (samples.length / 3) };
    }, png.toString('base64'));
  };
  const before = await native();
  const first = await capture(imagePrefix + '-start.png');
  await page.keyboard.down('ArrowUp'); await page.keyboard.down('ArrowLeft');
  await page.keyboard.down('ControlRight');
  try { await sleep(3500); } finally {
    await page.keyboard.up('ControlRight'); await page.keyboard.up('ArrowLeft'); await page.keyboard.up('ArrowUp');
  }
  const last = await capture(imagePrefix + '-controls.png');
  const after = await native();
  let changed = 0;
  assert.equal(first.samples.length, last.samples.length);
  for (let i = 0; i < first.samples.length; i += 3)
    if (Math.max(...[0,1,2].map(channel => Math.abs(first.samples[i+channel] - last.samples[i+channel]))) > 16) changed++;
  assert.ok(first.dark > .6 && last.dark > .6, 'Both frames must contain the actual battle playfield');
  assert.ok(changed >= 4, 'Real local combat inputs must produce visible playfield motion');
  assert.ok(after.backgrounds > before.backgrounds && after.draws > before.draws && after.images >= 48,
    'The native battle must advance without a network peer');
  assert.equal(after.connected, false);
  assert.equal(after.screen, 'melee');
  return { before, after, selectionKeys:['Enter','q'], changedPixels: changed, playfieldDark: [first.dark, last.dark] };
}
