const assert = require('node:assert/strict');
const fs = require('node:fs');
const vm = require('node:vm');
const test = require('node:test');
const source = fs.readFileSync('hd/wasm/pre.js', 'utf8');
const savePath = '/home/web_user/.uqm-hd';
function harness(config = '', initialError = null) {
  const files = new Map([[savePath + '/uqm.cfg', config], ['/home/web_user/.uqm/save/uqmsave.00', 'original-save-bytes']]);
  const mounts = [], unmounts = [], requests = [];
  let ready = false;
  const context = {
    Module: {}, window: {}, console: { log() {}, warn() {} }, IDBFS: {},
    addRunDependency() {}, removeRunDependency() { ready = true; },
    FS: {
      mkdirTree(p) { assert.equal(p, savePath); },
      mount(_, opts, p) { mounts.push(p); }, unmount(p) { unmounts.push(p); },
      syncfs(populate, callback) { requests.push(populate); callback(initialError); },
      readFile(p) { if (!files.has(p)) throw new Error('not found'); return files.get(p); },
      analyzePath(p) { return { exists: files.has(p) }; },
      writeFile(p, bytes) { assert.ok(p.startsWith(savePath + '/')); files.set(p, bytes); }
    }
  };
  vm.runInNewContext(source, context);
  context.Module.preRun[0]();
  return { context, files, mounts, unmounts, requests, ready };
}
test('HD settings load in their own database and preserve existing Original save bytes', () => {
  const h = harness('resolutionfactor = INT32:0\nloresBlowupScale = INT32:3\nsubtitles = BOOLEAN:false\nmusicvol = INT32:80\n');
  assert.deepEqual(h.mounts, [savePath]);
  assert.deepEqual(h.requests, [true]);
  assert.equal(h.files.get('/home/web_user/.uqm/save/uqmsave.00'), 'original-save-bytes');
  const config = h.files.get(savePath + '/uqm.cfg');
  assert.match(config, /^resolutionfactor = INT32:2$/m);
  assert.match(config, /^loresBlowupScale = INT32:0$/m);
  assert.match(config, /^subtitles = BOOLEAN:true$/m);
  assert.match(config, /^musicvol = INT32:80$/m);
  assert.equal((config.match(/^resolutionfactor = /gm) || []).length, 1);
  assert.equal(h.ready, true);
});
test('blocked HD storage still receives HD settings and cannot flush over unread saves', () => {
  const h = harness('', new Error('IndexedDB blocked'));
  assert.deepEqual(h.unmounts, [savePath]);
  assert.equal(h.ready, true);
  h.context.window.wasm_syncfs();
  assert.deepEqual(h.requests, [true]);
  assert.match(h.files.get(savePath + '/uqm.cfg'), /resolutionfactor = INT32:2/);
});
test('HD settings are idempotent and keep user MegaMod preferences', () => {
  const h = harness();
  const original = h.files.get(savePath + '/uqm.cfg');
  h.files.set(savePath + '/megamod.cfg', 'difficulty = INT32:2\n');
  h.context.Module.prepareHDConfig(savePath);
  assert.equal(h.files.get(savePath + '/uqm.cfg'), original);
  assert.equal(h.files.get(savePath + '/megamod.cfg'), 'difficulty = INT32:2\n');
});
