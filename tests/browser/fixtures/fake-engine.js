// Test-only stand-in for the Emscripten engine script, for fast screenshots of
// the page shells: download progress, runtime start, callMain, Module.gameScreen
// and the main-menu netplay hook. Query: ?fakeLoadMs=1500&fakeHook=0|1
(function () {
  'use strict';
  const params = new URLSearchParams(location.search);
  const loadMs = Number(params.get('fakeLoadMs') || 1200);
  const hook = params.get('fakeHook') !== '0';
  const canvas = document.getElementById('canvas');
  const ctx = canvas.getContext('2d');
  window.FS = { writeFile() {}, readFile() { return new Uint8Array(); }, mkdirTree() {}, syncfs(_, done) { if (done) done(null); } };
  window.ENV = {};
  Module.detect3DOVideos = () => ({ complete: true, missing: [], total: 0 });
  (Module.preRun || []).forEach(fn => { try { fn(); } catch (_) { /* Not needed here. */ } });
  const total = 298763132;
  let step = 0;
  const steps = 12;
  const timer = setInterval(() => {
    step++;
    Module.setStatus('Downloading data... (' + Math.floor(total * Math.min(1, step / steps)) + '/' + total + ')');
    if (step >= steps) {
      clearInterval(timer);
      Module.setStatus('');
      if (Module.onRuntimeInitialized) Module.onRuntimeInitialized();
    }
  }, loadMs / steps);
  const log = window.fakeEngineLog = [];
  function paint(screen) {
    const w = canvas.width, h = canvas.height;
    ctx.fillStyle = screen === 'melee' ? '#000040' : '#000';
    ctx.fillRect(0, 0, w, h);
    ctx.fillStyle = '#d6d6d6';
    ctx.font = Math.round(h / 12) + 'px serif';
    ctx.textAlign = 'center';
    if (screen === 'mainMenu') {
      ['New Game', 'Load Game', 'Super Melee!', 'Setup', 'Quit'].forEach((label, i) => ctx.fillText(label, w / 2, h * (0.3 + i * 0.11)));
      ctx.strokeStyle = '#888'; ctx.strokeRect(w * 0.25, h * 0.12, w * 0.5, h * 0.76);
      ctx.font = Math.round(h / 30) + 'px monospace'; ctx.textAlign = 'right'; ctx.fillText('v0.8.0', w - 8, h - 8);
    } else {
      ctx.fillText(screen === 'melee' ? 'SUPER-MELEE (test engine)' : 'Starting…', w / 2, h / 2);
    }
  }
  function setScreen(screen) {
    Module.gameScreen = screen;
    log.push(screen);
    paint(screen);
  }
  Module.callMain = args => {
    window.fakeArgs = args;
    const role = (args.find(arg => arg.indexOf('--web-netplay=') === 0) || '').split('=')[1];
    setScreen('transition');
    setTimeout(() => setScreen(role ? 'melee' : 'mainMenu'), role ? 200 : 500);
    setInterval(() => {
      if (Module.gameScreen === 'mainMenu') {
        if (hook) Module.supportsNetplayHook = true;
        const take = hook && typeof Module.takePendingNetplayRole === 'function' ? Module.takePendingNetplayRole() : -1;
        if (take === 0 || take === 1) { log.push('hook:' + take); setScreen('melee'); }
      } else if (Module.gameScreen === 'melee' && Module.webNet && ['closed', 'error'].includes(Module.webNet.state) && !Module.leaving) {
        // The engine shows its own "connection closed" popup, then returns to the main menu.
        Module.leaving = true;
        setTimeout(() => { Module.leaving = false; setScreen('mainMenu'); }, 600);
      }
    }, 50);
  };
  window.fakeEngine = { setScreen, paint };
})();
