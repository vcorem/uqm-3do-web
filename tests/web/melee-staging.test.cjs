const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const os = require('node:os');
const { spawnSync } = require('node:child_process');

function write(file, text) { fs.mkdirSync(path.dirname(file), { recursive: true }); fs.writeFileSync(file, text); }
test('content staging keeps all original collision assets intact and only publishes runtime resources', t => {
    const folder = fs.mkdtempSync(path.join(os.tmpdir(), 'uqm-melee-stage-'));
    t.after(() => fs.rmSync(folder, { recursive: true, force: true }));
    const source = path.join(folder, 'source'), target = path.join(folder, 'content');
    const original = path.join(target, 'addons/mm-hd/ships/human/cruiser-big.ani');
    write(original, 'original collision metadata\n');
    write(path.join(source, 'render/ships/human/cruiser-big-000.png'), 'runtime presentation image');
    write(path.join(source, 'art/atlas.png'), 'large authoring source');
    write(path.join(source, 'music/battle.ogg'), 'OggS battle');
    write(path.join(source, 'music/fleet-menu.ogg'), 'OggS fleet');
    write(path.join(target, 'remastered/render/removed-frame.png'), 'obsolete own output');
    const script = path.resolve('scripts/stage-melee-remastered.py');
    const result = spawnSync('python3', ['-c',
        'import runpy,sys; from pathlib import Path; m=runpy.run_path(sys.argv[1]); f=m["stage_content"]; f.__globals__["SOURCE"]=Path(sys.argv[2]); f(Path(sys.argv[3]))',
        script, source, target], { encoding: 'utf8' });
    assert.equal(result.status, 0, result.stdout + result.stderr);
    assert.equal(fs.readFileSync(original, 'utf8'), 'original collision metadata\n');
    assert.equal(fs.existsSync(path.join(target, 'remastered/render/removed-frame.png')), false);
    assert.equal(fs.existsSync(path.join(target, 'remastered/render/ships/human/cruiser-big-000.png')), true);
    assert.equal(fs.existsSync(path.join(target, 'remastered/art')), false, 'authoring atlases are not in the preload');
    assert.match(fs.readFileSync(path.join(target, 'addons/remastered/remastered.rmp'), 'utf8'),
        /music\.battle = MUSICRES:addons\/remastered\/music\/battle\.ogg/);
    assert.equal(fs.readFileSync(path.join(target, 'addons/remastered/music/fleet-menu.ogg'), 'utf8'), 'OggS fleet');
    // Validation occurs before replacing the existing compiled image tree.
    fs.writeFileSync(path.join(source, 'music/battle.ogg'), 'broken');
    const fail = spawnSync('python3', ['-c',
        'import runpy,sys; from pathlib import Path; m=runpy.run_path(sys.argv[1]); f=m["stage_content"]; f.__globals__["SOURCE"]=Path(sys.argv[2]); f(Path(sys.argv[3]))',
        script, source, target], { encoding: 'utf8' });
    assert.notEqual(fail.status, 0);
    assert.match(fail.stderr, /invalid Melee soundtrack/);
    assert.equal(fs.existsSync(path.join(target, 'remastered/render/ships/human/cruiser-big-000.png')), true);
});
