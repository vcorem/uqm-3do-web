// scripts/package-editions.py with small stand-in builds: the UQM editions, Melee
// Remastered (the HD builds again at /melee/) and the optional Cosmo Clash build at
// /cosmoclash/ with its card on the picker.
const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { spawnSync } = require('node:child_process');

const root = path.join(__dirname, '../..');
const script = path.join(root, 'scripts/package-editions.py');
const ENGINE = ['index.html', 'uqm-debug.js', 'uqm-debug.wasm', 'uqm-debug.data', 'uqm-debug.worker.js', 'COPYING.txt',
    'toolchain-fixes.json', 'game-help.js', 'engine-loader.js', 'online-ui.js', 'online-art.json', 'online-art.png',
    'online-stars.png', 'online-title.png'];
const EXTRA = { original: ['netplay-client.js'], hd: ['content-licenses.txt', 'netplay-client.js',
    'melee-presentation.js', 'melee-presentation.css', 'melee-nebula.jpg', 'melee-credits.txt'] };
const COSMO_PAGE = '<!doctype html><script type="module" crossorigin src="/cosmoclash/assets/index-AbCd1234.js"></script>' +
    '<link rel="stylesheet" crossorigin href="/cosmoclash/assets/index-Ef56Gh78.css"></head><body></body>';

function temp(t) {
    const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'uqm-package-'));
    t.after(() => fs.rmSync(dir, { recursive: true, force: true }));
    return dir;
}
function write(file, content) {
    fs.mkdirSync(path.dirname(file), { recursive: true });
    fs.writeFileSync(file, content);
}
function uqmBuilds(dir, rust = false) {
    const builds = rust ? ['original', 'hd', 'original-rust', 'hd-rust'] : ['original', 'hd'];
    for (const build of builds) {
        for (const name of [...ENGINE, ...EXTRA[build.replace('-rust', '')]]) {
            write(path.join(dir, build, name), name === 'index.html' ? '<head></head><header>UQM</header>' : build + ':' + name);
        }
    }
    return builds.flatMap(build => ['--' + build, path.join(dir, build)]);
}
/** The UQM_VARIANT that package-editions.py wrote into a page, or null. */
function variant(page) {
    const match = fs.readFileSync(page, 'utf8').match(/<script>window\.UQM_VARIANT = (.*?);<\/script>/);
    return match ? JSON.parse(match[1]) : null;
}
/** A Cosmo Clash build as `vite build --base /cosmoclash/` writes it; files overrides or removes (null) files. */
function cosmoBuild(dir, files = {}) {
    const all = {
        'index.html': COSMO_PAGE,
        'assets/index-AbCd1234.js': 'const base="/cosmoclash/";load(base+"models/gondi.glb");fetch(`/cosmoclash/sounds/${name}`);',
        'assets/index-Ef56Gh78.css': '.splash{background:url(/cosmoclash/textures/logo.jpg)}',
        'cosmoclash.wasm': Buffer.from([0, 97, 115, 109]),
        'models/gondi.glb': 'glTF',
        'sounds/Damage.ogg': 'OggS',
        'textures/logo.jpg': 'jpg',
        '.DS_Store': 'hidden',
        ...files,
    };
    fs.rmSync(dir, { recursive: true, force: true });
    for (const [name, content] of Object.entries(all)) if (content !== null) write(path.join(dir, name), content);
    return dir;
}
function run(args) {
    const result = spawnSync('python3', ['-I', script, ...args], { encoding: 'utf8' });
    return { status: result.status, output: result.stdout + result.stderr };
}
function files(dir) {
    return fs.readdirSync(dir, { recursive: true }).map(name => name.split(path.sep).join('/')).sort();
}

test('without --cosmoclash the site has no Cosmo Clash folder, card, image or link', t => {
    const dir = temp(t);
    const site = path.join(dir, 'site');
    const result = run([...uqmBuilds(dir), '--output', site]);
    assert.equal(result.status, 0, result.output);
    assert.equal(fs.existsSync(path.join(site, 'cosmoclash')), false);
    assert.equal(fs.existsSync(path.join(site, 'cosmoclash-card.jpg')), false);
    const picker = fs.readFileSync(path.join(site, 'index.html'), 'utf8');
    assert.match(picker, /data-games="uqm"/);
    assert.doesNotMatch(picker, /cosmoclash|Cosmo Clash/i);
    assert.deepEqual([...picker.matchAll(/<img\b[^>]*\bsrc="([^"]+)"/g)].map(match => match[1]), ['melee-card.jpg']);
    assert.match(picker, /id="edition-original"[\s\S]*id="edition-hd"[\s\S]*id="edition-melee"/);
    assert.match(picker, /Melee<\/a>\)\. JavaScript is required/, 'the noscript text stays whole');
});

test('Melee Remastered publishes the HD builds again with the Melee page and its own save folders', t => {
    const dir = temp(t);
    const site = path.join(dir, 'site');
    const result = run([...uqmBuilds(dir, true), '--output', site]);
    assert.equal(result.status, 0, result.output);
    assert.match(result.output, /Packaged original, hd, original-rust, hd-rust, melee, melee-rust:/);
    for (const [folder, build] of [['melee', 'hd'], ['melee-rust', 'hd-rust']]) {
        for (const name of [...ENGINE, ...EXTRA.hd].filter(name => name !== 'index.html')) {
            assert.equal(fs.readFileSync(path.join(site, folder, name), 'utf8'), build + ':' + name, folder + '/' + name);
        }
    }
    assert.equal(variant(path.join(site, 'hd/index.html')), null, 'the HD page stays the campaign page');
    assert.deepEqual(variant(path.join(site, 'hd-rust/index.html')),
        { engine: 'rust', label: 'Rust engine (port)', saveDir: '/home/web_user/.uqm-hd-rust' });
    assert.deepEqual(variant(path.join(site, 'melee/index.html')),
        { engine: 'c', saveDir: '/home/web_user/.uqm-melee', meleeOnly: true });
    assert.deepEqual(variant(path.join(site, 'melee-rust/index.html')),
        { engine: 'rust', label: 'Rust engine (port)', saveDir: '/home/web_user/.uqm-melee-rust', meleeOnly: true });
    assert.deepEqual(fs.readFileSync(path.join(site, 'melee-card.jpg')), fs.readFileSync(path.join(root, 'web/editions/melee-card.jpg')));
    assert.match(fs.readFileSync(path.join(site, 'index.html'), 'utf8'), /data-engines="c,rust"/);
    // Without the HD Rust build there is no Rust Melee page either.
    const plain = path.join(dir, 'plain');
    assert.equal(run([...uqmBuilds(dir), '--output', plain]).status, 0);
    assert.equal(fs.existsSync(path.join(plain, 'melee-rust')), false);
    assert.equal(fs.existsSync(path.join(plain, 'melee/index.html')), true);
    // An HD build without the Melee page files is refused.
    const builds = uqmBuilds(dir);
    fs.rmSync(path.join(dir, 'hd', 'melee-presentation.js'));
    const refused = run([...builds, '--output', path.join(dir, 'other')]);
    assert.notEqual(refused.status, 0);
    assert.match(refused.output, /Missing build artifact: .*melee-presentation\.js/);
});

test('--cosmoclash publishes the whole build at /cosmoclash/ and keeps the third card', t => {
    const dir = temp(t);
    const site = path.join(dir, 'site');
    const cosmo = cosmoBuild(path.join(dir, 'cosmo'));
    const result = run([...uqmBuilds(dir), '--cosmoclash', cosmo, '--output', site]);
    assert.equal(result.status, 0, result.output);
    assert.match(result.output, /Packaged original, hd, melee, cosmoclash/);
    assert.deepEqual(files(path.join(site, 'cosmoclash')), ['assets', 'assets/index-AbCd1234.js', 'assets/index-Ef56Gh78.css',
        'cosmoclash.wasm', 'index.html', 'models', 'models/gondi.glb', 'sounds', 'sounds/Damage.ogg', 'textures', 'textures/logo.jpg']);
    assert.equal(fs.readFileSync(path.join(site, 'cosmoclash/index.html'), 'utf8'), COSMO_PAGE, 'the game page is copied unchanged');
    assert.deepEqual(fs.readFileSync(path.join(site, 'cosmoclash-card.jpg')),
        fs.readFileSync(path.join(root, 'web/editions/cosmoclash-card.jpg')));
    const picker = fs.readFileSync(path.join(site, 'index.html'), 'utf8');
    // Only the engine marker changes (these stand-in builds have no Rust engines).
    assert.equal(picker, fs.readFileSync(path.join(root, 'web/editions/index.html'), 'utf8')
        .replace('data-engines="c,rust"', 'data-engines="c"'));
    assert.match(picker, /id="edition-cosmoclash"/);
    assert.match(picker, /by Ori Hanegby, Guy Corem and Orr Kowarsky/);
    assert.match(picker, /<img src="cosmoclash-card\.jpg"/);

    // A new build replaces the old one: old hashed names do not stay.
    const next = cosmoBuild(cosmo, { 'assets/index-AbCd1234.js': null, 'assets/index-Zz99Yy88.js': 'export {};',
        'index.html': COSMO_PAGE.replace('index-AbCd1234.js', 'index-Zz99Yy88.js') });
    assert.equal(run([...uqmBuilds(dir), '--cosmoclash', next, '--output', site]).status, 0);
    assert.deepEqual(files(path.join(site, 'cosmoclash/assets')), ['index-Ef56Gh78.css', 'index-Zz99Yy88.js']);
    // A later package without Cosmo Clash removes it from the site.
    assert.equal(run([...uqmBuilds(dir), '--output', site]).status, 0);
    assert.equal(fs.existsSync(path.join(site, 'cosmoclash')), false);
    assert.equal(fs.existsSync(path.join(site, 'cosmoclash-card.jpg')), false);
});

test('a Cosmo Clash build that cannot work at /cosmoclash/ is refused before anything is written', t => {
    const dir = temp(t);
    const site = path.join(dir, 'site');
    const builds = uqmBuilds(dir);
    const big = path.join(dir, 'big.ogg');
    fs.writeFileSync(big, '');
    fs.truncateSync(big, 25 * 1024 * 1024 + 1);
    for (const [label, files, message] of [
        ['built for /', { 'index.html': COSMO_PAGE.replaceAll('/cosmoclash/assets/', '/assets/') }, /--base \/cosmoclash\//],
        ['another origin', { 'index.html': COSMO_PAGE + '<script src="https://cdn.example/three.js"></script>' }, /another origin/],
        ['a protocol-relative URL', { 'index.html': COSMO_PAGE + '<link rel="stylesheet" href="//fonts.example/a.css">' }, /another origin/],
        ['a page URL outside its folder', { 'index.html': COSMO_PAGE + '<link rel="icon" href="/favicon.ico">' }, /outside \/cosmoclash\//],
        ['a missing script', { 'assets/index-AbCd1234.js': null }, /missing file: \/cosmoclash\/assets\/index-AbCd1234\.js/],
        ['no Wasm', { 'cosmoclash.wasm': null }, /Missing Cosmo Clash build artifact.*cosmoclash\.wasm/],
        ['a file over 25 MiB', { 'sounds/big.ogg': fs.readFileSync(big) }, /larger than 25 MiB/],
    ]) {
        const cosmo = cosmoBuild(path.join(dir, 'cosmo'), files);
        const result = run([...builds, '--cosmoclash', cosmo, '--output', site]);
        assert.notEqual(result.status, 0, label);
        assert.match(result.output, message, label);
        assert.equal(fs.existsSync(site), false, label + ': nothing written');
        const check = run(['--check-cosmoclash', cosmo]);
        assert.equal(check.status, 1, label);
        assert.match(check.output, message, label);
    }
    const good = cosmoBuild(path.join(dir, 'cosmo'));
    const check = run(['--check-cosmoclash', good]);
    assert.equal(check.status, 0, check.output);
    assert.match(check.output, /ready for \/cosmoclash\//);
    // Root paths in the scripts are accepted: the game adds the base at run time
    // (cosmoclash/src/base.ts); tests/browser/cosmoclash-site.mjs checks the real requests.
    const rootPaths = cosmoBuild(path.join(dir, 'cosmo'), {
        'assets/index-AbCd1234.js': 'const base="/cosmoclash/";load("/models/gondi.glb");fetch(`/sounds/${name}`);new Audio("/sounds/Damage.ogg");',
        'assets/index-Ef56Gh78.css': '.splash{background:url(/cosmoclash/textures/logo.jpg)}',
    });
    const accepted = run(['--check-cosmoclash', rootPaths]);
    assert.equal(accepted.status, 0, accepted.output);
    assert.equal(run([...builds, '--cosmoclash', rootPaths, '--output', site]).status, 0);
    fs.rmSync(site, { recursive: true, force: true });
    assert.match(run(['--cosmoclash', good, '--output', site]).output, /--original, --hd and --output are required/);
    const inside = run([...builds, '--cosmoclash', good, '--output', path.join(good, 'site')]);
    assert.match(inside.output, /separate from every engine build directory/);
});

test('the build script makes and checks Cosmo Clash first, unless UQM_COSMO=0', () => {
    const text = fs.readFileSync(path.join(root, 'scripts/build-editions.sh'), 'utf8');
    const cosmo = text.indexOf('npm run build --prefix "$root/cosmoclash" -- --base /cosmoclash/');
    assert.ok(cosmo > 0 && cosmo < text.indexOf('build-web.sh'), 'Cosmo Clash builds before the UQM engines');
    assert.match(text, /if \[\[ "\$\{UQM_COSMO:-1\}" != 0 \]\]/);
    assert.match(text, /--check-cosmoclash "\$root\/cosmoclash\/dist"/);
    assert.match(text, /package_args\+=\(--cosmoclash "\$root\/cosmoclash\/dist"\)/);
});
