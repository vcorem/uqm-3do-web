// Test pages for the in-frame start screen and lobby: the real page shells with
// fixtures/fake-engine.js in place of the engine, the real page scripts, and the
// art that web/online/build-art.py makes from the game content.
import { execFileSync } from 'node:child_process';
import { copyFile, mkdir, readFile, writeFile, access } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { meleePage } from './melee-page.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const ART = ['online-art.json', 'online-art.png', 'online-stars.png', 'online-title.png'];
const exists = file => access(file).then(() => true, () => false);

export const hdContent = path.join(root, '.tools/hd-source/content');

// 'melee' is Melee Remastered: the HD page with the variant of package-editions.py.
export async function buildHarness(output, editions = ['original', 'hd', 'melee']) {
  const built = [];
  for (const edition of editions) {
    const dir = path.join(output, edition);
    await mkdir(dir, { recursive: true });
    const hd = edition !== 'original';
    const content = hd ? hdContent : path.join(root, 'sc2/content');
    if (hd && !await exists(content)) continue;
    if (!(await Promise.all(ART.map(name => exists(path.join(dir, name))))).every(Boolean) || process.env.UQM_REBUILD_ART === '1') {
      execFileSync('python3', [path.join(root, 'web/online/build-art.py'), '--edition', hd ? 'hd' : 'original', '--content', content, '--output', dir], { stdio: 'inherit' });
    }
    const shell = await readFile(path.join(root, hd ? 'hd/wasm/shell.html' : 'sc2/wasm/shell.html'), 'utf8');
    if (!shell.includes('{{{ SCRIPT }}}')) throw new Error('The shell has no {{{ SCRIPT }}} marker');
    const page = shell.replace('{{{ SCRIPT }}}', '<script src="fake-engine.js"></script>');
    await writeFile(path.join(dir, 'index.html'), edition === 'melee' ? meleePage(page) : page);
    await copyFile(path.join(root, 'tests/browser/fixtures/fake-engine.js'), path.join(dir, 'fake-engine.js'));
    await copyFile(path.join(root, 'web/game-help/game-help.js'), path.join(dir, 'game-help.js'));
    // The shells start the engine through the data loader (it finds no data file here and starts it as usual).
    await copyFile(path.join(root, 'web/engine-loader/engine-loader.js'), path.join(dir, 'engine-loader.js'));
    await copyFile(path.join(root, 'web/online/online-ui.js'), path.join(dir, 'online-ui.js'));
    await copyFile(path.join(root, 'sc2/wasm/netplay-client.js'), path.join(dir, 'netplay-client.js'));
    // Both HD pages load the Melee page controls; the Melee page also draws its start-screen art.
    if (hd) execFileSync('python3', [path.join(root, 'scripts/stage-melee-remastered.py'), '--output', dir], { stdio: 'inherit' });
    built.push(edition);
  }
  return built;
}

export const shimPath = path.join(root, 'tests/browser/fixtures/lobby-client-shim.js');
